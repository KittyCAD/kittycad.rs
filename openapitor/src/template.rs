//! Templates for our generated client libraries.

use std::fmt::Write as _;

use anyhow::Result;
use inflector::cases::{kebabcase::to_kebab_case, screamingsnakecase::to_screaming_snake_case};

fn generate_docs_openapi_info(
    spec: &openapiv3::OpenAPI,
    spec_link: &Option<String>,
    package_name: &str,
    description: &str,
) -> Result<String> {
    let mut desc = String::new();
    if let Some(d) = &spec.info.description {
        desc = d.replace('\n', "\n//! ");
    }

    let mut tos = String::new();
    if let Some(t) = &spec.info.terms_of_service {
        tos = format!("[API Terms of Service]({})", t);
    }

    let mut contact = String::new();
    if let Some(c) = &spec.info.contact {
        let mut num = 1;
        let mut name = String::new();
        if let Some(n) = &c.name {
            contact.push_str("| name ");
            name = n.to_string();
            num += 1;
        }
        let mut url = String::new();
        if let Some(u) = &c.url {
            contact.push_str("| url ");
            url = u.to_string();
            num += 1;
        }
        let mut email = String::new();
        if let Some(e) = &c.email {
            contact.push_str("| email ");
            email = e.to_string();
            num += 1;
        }
        if !contact.is_empty() {
            contact.push('|');
            contact = format!(
                r#"//! {}
//! "#,
                contact
            );
            for _ in 1..num {
                contact.push_str("|----");
            }
            contact.push_str("|\n//! ");
        }

        if !name.is_empty() {
            write!(contact, "| {} ", name)?;
        }
        if !url.is_empty() {
            write!(contact, "| <{}> ", url)?;
        }
        if !email.is_empty() {
            write!(contact, "| {} ", email)?;
        }
        if !contact.is_empty() {
            contact.push_str("|\n//! ");
        }

        contact = format!("### Contact\n//!\n//! \n{}", contact);
    }

    let mut license = String::new();
    if let Some(l) = &spec.info.license {
        license.push_str("| name ");

        let mut url = String::new();
        if let Some(u) = &l.url {
            license.push_str("| url ");
            url = u.to_string();
        }
        license.push('|');
        license = format!(
            r#"//! {}
//! "#,
            license
        );

        license.push_str("|----");
        if !url.is_empty() {
            license.push_str("|----");
        }
        license.push_str("|\n//! ");

        write!(license, "| {} ", l.name)?;
        if !url.is_empty() {
            write!(license, "| <{}> ", url)?;
        }
        license.push_str("|\n//! ");

        license = format!("### License\n//!\n//! \n{}", license);
    }

    let api_version = format!("based on API spec version `{}`", spec.info.version);

    let spec_link_blurb = if let Some(link) = spec_link {
        format!("This client is generated from the [OpenAPI specs]({}) {}. This way it will remain up to date as features are added.", link, api_version)
    } else {
        "".to_string()
    };

    Ok(format!(
        r#"//! {}
//!
//! [![docs.rs](https://docs.rs/{}/badge.svg)](https://docs.rs/{})
//!
//! ## API Details
//!
//! {}
//!
//! {}
//!
//! {}
//! {}
//!
//! ## Client Details
//!
//! {}
//!
//! The documentation for the crate is generated
//! along with the code to make this library easy to use.
//! "#,
        description,
        to_kebab_case(package_name),
        to_kebab_case(package_name),
        desc,
        tos,
        contact,
        license,
        spec_link_blurb,
    ))
}

/// Generate the main docs for our client library.
pub fn generate_docs(
    spec: &openapiv3::OpenAPI,
    name: &str,
    description: &str,
    version: &str,
    spec_link: &Option<String>,
) -> Result<String> {
    let info = generate_docs_openapi_info(spec, spec_link, name, description)?;
    Ok(format!(
        r#"{}
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! {} = "{}"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use {}::Client;
//!
//! let client = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `{}`
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use {}::Client;
//!
//! let client = Client::new_from_env();
//! ```
//!"#,
        info,
        name.replace('_', "-").to_lowercase(),
        version,
        name,
        get_token_env_variable(name),
        name,
    ))
}

/// Get the name of the environment variable for the API token.
pub fn get_token_env_variable(name: &str) -> String {
    format!("{}_API_TOKEN", to_screaming_snake_case(name))
}
