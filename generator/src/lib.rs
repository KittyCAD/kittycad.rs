//! A library for generating rust client sdks from OpenAPI specs.
#![deny(missing_docs)]

pub mod client;
pub mod functions;
pub mod template;
pub mod types;

#[macro_use]
extern crate quote;

use std::{fs, io::Write};

use anyhow::Result;

use crate::types::exts::ReferenceOrExt;

/// Save a file.
pub fn save<P>(p: P, data: &str) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    let p = p.as_ref();
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(p)?;
    f.write_all(data.as_bytes())?;
    f.flush()?;
    Ok(())
}

/// Load a file.
fn load<P, T>(p: P) -> Result<T>
where
    P: AsRef<std::path::Path>,
    for<'de> T: serde::Deserialize<'de>,
{
    let p = p.as_ref();
    let f = fs::File::open(p)?;
    if let Some(ext) = p.extension() {
        if ext == std::ffi::OsStr::new("yaml") || ext == std::ffi::OsStr::new("yml") {
            return Ok(serde_yaml::from_reader(f)?);
        }
    }
    Ok(serde_json::from_reader(f)?)
}

/// Load an OpenAPI spec.
pub fn load_api<P>(p: P) -> Result<openapiv3::OpenAPI>
where
    P: AsRef<std::path::Path>,
{
    load(p)
}

/// Generate a client library.
pub fn generate(api: &openapiv3::OpenAPI) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    /*
     * Deal with any dependencies we require to produce this client.
     */
    a("#![allow(missing_docs)]"); // TODO: Make this a deny.
    a("#![cfg_attr(docsrs, feature(doc_cfg))]");
    a("");
    a("#[cfg(test)]");
    a("mod tests;");
    // Hopefully there is never a "tag" named after these reserved libs.
    a("pub mod types;");
    a("#[doc(hidden)]");

    // First get the tags for all the paths, then later we can ignore tags that
    // have no paths.
    let mut tags_with_paths = Vec::<String>::new();
    for (name, path) in api.paths.iter() {
        let op = path.item()?;

        let mut get_tags =
            |name: &str, method: &str, op: Option<&openapiv3::Operation>| -> Result<()> {
                // Ensure we have an operation for this path and method, otherwise return early.
                let op = if let Some(op) = op {
                    op
                } else {
                    return Ok(());
                };

                let tag = op.tags.first().ok_or_else(|| {
                    anyhow::anyhow!("operation `{}` `{}` has no tags", name, method)
                })?;

                // Add our tag to our vector.
                // TODO: there is some repeated code above w functions.rs we could probably
                // clean up.
                tags_with_paths.push(tag.to_string());

                Ok(())
            };

        get_tags(name.as_str(), "GET", op.get.as_ref())?;
        get_tags(name.as_str(), "PUT", op.put.as_ref())?;
        get_tags(name.as_str(), "POST", op.post.as_ref())?;
        get_tags(name.as_str(), "DELETE", op.delete.as_ref())?;
        get_tags(name.as_str(), "HEAD", op.head.as_ref())?;
        get_tags(name.as_str(), "PATCH", op.patch.as_ref())?;
        get_tags(name.as_str(), "TRACE", op.trace.as_ref())?;
    }

    /*
     * Import the module for each tag.
     * Tags are how functions are grouped.
     */
    for tag in api.tags.iter() {
        if !tags_with_paths.contains(&tag.name) {
            // Continue if this tag has no paths.
            continue;
        }

        let mut docs = "".to_string();
        if let Some(d) = &tag.description {
            docs = format!("{}.", d.trim_end_matches('.'));
        }
        if let Some(e) = &tag.external_docs {
            if !e.url.is_empty() {
                docs = format!("{}\n\nFROM: <{}>", docs, e.url);
            }
        }
        docs = docs.trim().to_string();

        if !docs.is_empty() {
            a(&format!("/// {}", docs.replace('\n', "\n/// "),));
        }
        a(&format!("pub mod {};", clean_tag_name(&tag.name)));
    }

    a("");

    // Print the client template.
    a(&crate::client::generate_client());

    a("");

    /*
     * Generate a function for each tag.
     * Tags are how functions are grouped.
     */
    for tag in api.tags.iter() {
        if !tags_with_paths.contains(&tag.name) {
            // Continue if this tag has no paths.
            continue;
        }

        let mut docs = format!(
            "Return a reference to an interface that provides access to {} operations.",
            tag.name
        );
        if let Some(d) = &tag.description {
            docs = format!("{}.", d.trim_end_matches('.'));
        }
        if let Some(e) = &tag.external_docs {
            if !e.url.is_empty() {
                docs = format!("{}\n\nFROM: <{}>", docs, e.url);
            }
        }

        a(&format!(
            r#"/// {}
               pub fn {}(&self) -> {}::{} {{
                    {}::{}::new(self.clone())
               }}"#,
            docs.replace('\n', "\n/// "),
            clean_tag_name(&tag.name),
            clean_tag_name(&tag.name),
            types::proper_name(&tag.name),
            clean_tag_name(&tag.name),
            types::proper_name(&tag.name),
        ));
        a("");
    }

    a("}");

    Ok(out)
}

/// Clean a tag name.
pub fn clean_tag_name(s: &str) -> String {
    let result = inflector::cases::snakecase::to_snake_case(s);

    if result == "oauth_2" {
        "oauth2".to_string()
    } else {
        result
    }
}
