//! Templates for our generated client libraries.

use std::fmt::Write as _;

use anyhow::Result;
use inflector::cases::screamingsnakecase::to_screaming_snake_case;

fn generate_docs_openapi_info(spec: &openapiv3::OpenAPI, opts: &crate::Opts) -> Result<String> {
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

    let spec_link_blurb = if let Some(link) = &opts.spec_url {
        format!(
            "This client is generated from the [OpenAPI specs]({}) {}. This way it will remain up \
             to date as features are added.",
            link, api_version
        )
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
        opts.description,
        opts.package_name(),
        opts.package_name(),
        desc,
        tos,
        contact,
        license,
        spec_link_blurb,
    ))
}

/// Generate the main docs for our client library.
pub fn generate_docs(spec: &openapiv3::OpenAPI, opts: &crate::Opts) -> Result<String> {
    let info = generate_docs_openapi_info(spec, opts)?;
    if opts.token_endpoint.is_some() {
        return Ok(format!(
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
//!     String::from("client-id"),
//!     String::from("client-secret"),
//!     String::from("redirect-uri"),
//!     String::from("token"),
//!     String::from("refresh-token"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `{}_CLIENT_ID`
//! - `{}_CLIENT_SECRET`
//! - `{}_REDIRECT_URI`
//! {}
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use {}::Client;
//!
//! let client = Client::new_from_env(String::from("token"), String::from("refresh-token"));
//! ```
//!"#,
            info,
            opts.package_name(),
            opts.target_version,
            opts.code_package_name(),
            get_env_variable_prefix(&opts.name),
            get_env_variable_prefix(&opts.name),
            get_env_variable_prefix(&opts.name),
            if let Some(add_env_prefix) = &opts.add_env_prefix {
                format!(
                    r#"- `{}_CLIENT_ID`
//! - `{}_CLIENT_SECRET`
//! - `{}_REDIRECT_URI`"#,
                    get_env_variable_prefix(add_env_prefix),
                    get_env_variable_prefix(add_env_prefix),
                    get_env_variable_prefix(add_env_prefix)
                )
            } else {
                "".to_string()
            },
            opts.code_package_name(),
        ));
    }

    if opts.basic_auth {
        return Ok(format!(
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
//!     String::from("username"),
//!     String::from("password"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `{}_USERNAME`
//! - `{}_PASSWORD`
//! {}
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
            opts.package_name(),
            opts.target_version,
            opts.code_package_name(),
            get_env_variable_prefix(&opts.name),
            get_env_variable_prefix(&opts.name),
            if let Some(add_env_prefix) = &opts.add_env_prefix {
                format!(
                    r#"- `{}_USERNAME`
//! - `{}_PASSWORD`"#,
                    get_env_variable_prefix(add_env_prefix),
                    get_env_variable_prefix(add_env_prefix)
                )
            } else {
                "".to_string()
            },
            opts.code_package_name(),
        ));
    }

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
//! - `{}_API_TOKEN`
//! {}
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
        opts.package_name(),
        opts.target_version,
        opts.code_package_name(),
        get_env_variable_prefix(&opts.name),
        if let Some(add_env_prefix) = &opts.add_env_prefix {
            format!(
                r#"- `{}_API_TOKEN`"#,
                get_env_variable_prefix(add_env_prefix),
            )
        } else {
            "".to_string()
        },
        opts.code_package_name(),
    ))
}

/// Get the prefix of the environment variables.
pub fn get_env_variable_prefix(name: &str) -> String {
    to_screaming_snake_case(name)
        .trim_end_matches("_API")
        .to_string()
}

/// Generate utils for our client library.
pub fn generate_utils(opts: &crate::Opts) -> Option<String> {
    opts.date_time_format.as_ref().map(|date_time_format| {
      format!(
        r#"

pub mod date_time_format {{
    use chrono::{{DateTime, NaiveDateTime, Utc}};
    use serde::{{self, Deserialize, Deserializer}};
    const FORMAT: &str = "{format}";
    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {{
        let s: String = String::deserialize(deserializer)?;
        match NaiveDateTime::parse_from_str(&s, FORMAT) {{
            Ok(t) => Ok(t.and_utc()),
            Err(_) => {{
                match serde_json::from_str::<DateTime<Utc>>(&format!("\"{{}}\"", s)) {{
                    Ok(t) => Ok(t),
                    Err(e) => {{
                        Err(serde::de::Error::custom(format!("deserializing {{}} as DateTime<Utc> failed: {{}}", s, e)))
                    }}
                }}
            }}
        }}
    }}
}}

pub mod nullable_date_time_format {{
    use chrono::{{DateTime, NaiveDateTime, Utc}};
    use serde::{{self, Deserialize, Deserializer}};
    const FORMAT: &str = "{format}";
    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {{
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {{
            // This is standard.
            match NaiveDateTime::parse_from_str(&s, FORMAT) {{
                Ok(t) => Ok(Some(t.and_utc())),
                Err(_) => {{
                    match serde_json::from_str::<DateTime<Utc>>(&format!("\"{{}}\"", s)) {{
                        Ok(t) => Ok(Some(t)),
                        Err(e) => {{
                            Err(serde::de::Error::custom(format!("deserializing {{}} as DateTime<Utc> failed: {{}}", s, e)))
                        }}
                    }}
                }}
            }}
        }} else {{
            Ok(None)
        }}
    }}
}}

"#,
        format = date_time_format,
    )})
}
