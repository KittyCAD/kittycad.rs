use std::collections::BTreeMap;

use anyhow::{anyhow, bail, Context, Result};
use inflector::cases::{kebabcase::to_kebab_case, snakecase::to_snake_case};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Component {
    Constant(String),
    Parameter(String),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Template {
    components: Vec<Component>,
}

impl Template {
    pub fn compile(&self, query_params: BTreeMap<String, (String, String)>) -> String {
        let mut out = String::new();

        let mut a = |s: &str| {
            out.push_str(s);
            out.push('\n');
        };

        if !query_params.is_empty() {
            // Format the query params if they exist.
            a("let mut query_args: Vec<(String, String)> = Default::default();");

            for (nam, (value, prop)) in &query_params {
                if value == "Option<chrono::DateTime<chrono::Utc>>" {
                    a(&format!(
                        r#"if let Some(date) = {} {{ query_args.push(("{}".to_string(), date.to_rfc3339())); }}"#,
                        nam, prop
                    ));
                } else if value == "Option<uuid::Uuid>" {
                    a(&format!(
                        r#"if let Some(u) = {} {{ query_args.push(("{}".to_string(), u.to_string())); }}"#,
                        nam, prop
                    ));
                } else if value == "uuid::Uuid" {
                    a(&format!(
                        r#"if {}.to_string() != uuid::Uuid::nil().to_string() {{ query_args.push(("{}".to_string(), {}.to_string())); }}"#,
                        nam, prop, nam
                    ));
                } else if value == "i64" || value == "i32" {
                    a(&format!(
                        r#"if {} > 0 {{ query_args.push(("{}".to_string(), {}.to_string())); }}"#,
                        nam, prop, nam
                    ));
                } else if value == "bool" && prop == "sendNotificationEmail" {
                    a(&format!(
                        r#"query_args.push(("{}".to_string(), {}.to_string()));"#,
                        prop, nam
                    ));
                } else if value == "bool" {
                    a(&format!(
                        r#"if {} {{ query_args.push(("{}".to_string(), {}.to_string())); }}"#,
                        nam, prop, nam
                    ));
                } else if value == "&str" {
                    a(&format!(
                        r#"if !{}.is_empty() {{ query_args.push(("{}".to_string(), {}.to_string())); }}"#,
                        nam, prop, nam
                    ));
                } else if value == "&[String]" {
                    // TODO: I have no idea how these should be seperated and the docs
                    // don't give any answers either, for an array sent through query
                    // params.
                    // https://docs.github.com/en/rest/reference/migrations
                    a(&format!(
                        r#"if !{}.is_empty() {{ query_args.push(("{}".to_string(), {}.join(" "))); }}"#,
                        nam, prop, nam
                    ));
                } else {
                    a(&format!(
                        r#"if !{}.to_string().is_empty() {{  query_args.push(("{}".to_string(), {}.to_string())); }}"#,
                        nam, prop, nam
                    ));
                }
            }

            a("let query_ = serde_urlencoded::to_string(&query_args).unwrap();");
        }

        a("let url =");
        if self.components.is_empty() && query_params.is_empty() {
            a(r#""".to_string();"#);

            return out.to_string();
        }

        let mut has_params = false;
        for c in self.components.iter() {
            match c {
                Component::Constant(_) => (),
                Component::Parameter(_) => {
                    has_params = true;
                    break;
                }
            }
        }

        if !has_params && query_params.is_empty() {
            out.push('"');
            for c in self.components.iter() {
                out.push('/');
                match c {
                    Component::Constant(n) => out.push_str(n),
                    Component::Parameter(_) => (),
                }
            }
            out.push_str("\".to_string();");

            return out.to_string();
        }

        out.push_str("format!(\"");
        for c in self.components.iter() {
            out.push('/');
            match c {
                Component::Constant(n) => out.push_str(n),
                Component::Parameter(_) => {
                    out.push_str("{}");
                }
            }
        }

        if !query_params.is_empty() {
            out.push_str("?{}");
        }

        out.push_str("\",\n");
        for c in self.components.iter() {
            if let Component::Parameter(n) = &c {
                if n == "type"
                    || n == "ref"
                    || n == "foo"
                    || n == "enum"
                    || n == "const"
                    || n == "use"
                {
                    out.push_str(&format!(
                        "crate::progenitor_support::encode_path(&{}_.to_string()),",
                        to_snake_case(n)
                    ));
                } else {
                    out.push_str(&format!(
                        "crate::progenitor_support::encode_path(&{}.to_string()),",
                        to_snake_case(n)
                    ));
                }
            }
        }

        if !query_params.is_empty() {
            out.push_str("query_");
        }

        out.push_str(");\n");

        out
    }
}

pub fn parse(t: &str) -> Result<Template> {
    parse_inner(t).with_context(|| anyhow!("parse failure for template {:?}", t))
}

fn parse_inner(t: &str) -> Result<Template> {
    enum State {
        Start,
        ConstantOrParameter,
        Parameter,
        ParameterSlash,
        Constant,
    }

    let mut s = State::Start;
    let mut a = String::new();
    let mut components = Vec::new();

    for c in t.chars() {
        match s {
            State::Start => {
                if c == '/' {
                    s = State::ConstantOrParameter;
                } else {
                    bail!("path must start with a slash");
                }
            }
            State::ConstantOrParameter => {
                if c == '/' || c == '}' {
                    bail!("expected a constant or parameter");
                } else if c == '{' {
                    s = State::Parameter;
                } else {
                    s = State::Constant;
                    a.push(c);
                }
            }
            State::Constant => {
                if c == '/' {
                    components.push(Component::Constant(a));
                    a = String::new();
                    s = State::ConstantOrParameter;
                } else if c == '{' || c == '}' {
                    bail!("unexpected parameter");
                } else {
                    a.push(c);
                }
            }
            State::Parameter => {
                if c == '}' {
                    components.push(Component::Parameter(a));
                    a = String::new();
                    s = State::ParameterSlash;
                } else if c == '/' || c == '{' {
                    bail!("expected parameter");
                } else {
                    a.push(c);
                }
            }
            State::ParameterSlash => {
                if c == '/' || c == ':' || c == '.' {
                    // Google Admin API has ":issueCommand" so we want to allow that!
                    // Shopify sometimes ends after a parameter with ".json", so we want to allow
                    // that.
                    s = State::ConstantOrParameter;
                } else {
                    bail!("expected a slash after parameter");
                }
            }
        }
    }

    match s {
        State::Start => bail!("empty path"),
        State::ConstantOrParameter | State::ParameterSlash => (),
        State::Constant => components.push(Component::Constant(a)),
        State::Parameter => bail!("unterminated parameter"),
    }

    Ok(Template { components })
}

#[cfg(test)]
mod test {
    use anyhow::{anyhow, Context, Result};

    use super::{parse, Component, Template};

    #[test]
    fn basic() -> Result<()> {
        let trials = vec![
            (
                "/info",
                Template {
                    components: vec![Component::Constant("info".into())],
                },
            ),
            (
                "/measure/{number}",
                Template {
                    components: vec![
                        Component::Constant("measure".into()),
                        Component::Parameter("number".into()),
                    ],
                },
            ),
            (
                "/one/{two}/three",
                Template {
                    components: vec![
                        Component::Constant("one".into()),
                        Component::Parameter("two".into()),
                        Component::Constant("three".into()),
                    ],
                },
            ),
        ];

        for (path, want) in trials.iter() {
            let t = parse(path).with_context(|| anyhow!("path {}", path))?;
            assert_eq!(&t, want);
        }

        Ok(())
    }

    #[test]
    fn compile() -> Result<()> {
        let t = parse("/measure/{number}")?;
        let out = t.compile(Default::default());
        let want = "let url =
format!(\"/measure/{}\",
crate::progenitor_support::encode_path(&number.to_string()),);\n";
        assert_eq!(want, &out);
        Ok(())
    }
}

pub fn generate_docs_openapi_info(
    api: &openapiv3::OpenAPI,
    spec_link: &str,
    package_name: &str,
) -> String {
    let mut description = String::new();
    if let Some(d) = &api.info.description {
        description = d.replace('\n', "\n//! ");
    }

    let mut tos = String::new();
    if let Some(t) = &api.info.terms_of_service {
        tos = format!("[API Terms of Service]({})", t);
    }

    let mut contact = String::new();
    if let Some(c) = &api.info.contact {
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
            contact.push_str(&format!("| {} ", name));
        }
        if !url.is_empty() {
            contact.push_str(&format!("| <{}> ", url));
        }
        if !email.is_empty() {
            contact.push_str(&format!("| {} ", email));
        }
        if !contact.is_empty() {
            contact.push_str("|\n//! ");
        }

        contact = format!("### Contact\n//!\n//! \n{}", contact);
    }

    let mut license = String::new();
    if let Some(l) = &api.info.license {
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

        license.push_str(&format!("| {} ", l.name));
        if !url.is_empty() {
            license.push_str(&format!("| <{}> ", url));
        }
        license.push_str("|\n//! ");

        license = format!("### License\n//!\n//! \n{}", license);
    }

    let api_version = format!("based on API spec version `{}`", api.info.version);

    format!(
        r#"//! A fully generated, opinionated API client library for KittyCAD.
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
//! This client is generated from the [KittyCAD OpenAPI
//! specs]({}) {}. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//! "#,
        to_kebab_case(package_name),
        to_kebab_case(package_name),
        description,
        tos,
        contact,
        license,
        spec_link,
        api_version,
    )
}

pub fn generate_docs(
    api: &openapiv3::OpenAPI,
    name: &str,
    version: &str,
    spec_link: &str,
) -> String {
    let info = generate_docs_openapi_info(api, spec_link, name);
    format!(
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
//! ```
//! use {}::Client;
//!
//! let kittycad = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `KITTYCAD_API_TOKEN`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use {}::Client;
//!
//! let kittycad = Client::new_from_env();
//! ```
//!"#,
        info,
        name.replace('_', "-").to_lowercase(),
        version,
        name,
        name,
    )
}
