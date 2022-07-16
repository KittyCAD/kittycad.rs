#![deny(missing_docs)]

mod client;
mod functions;
mod template;
mod types;

#[macro_use]
extern crate quote;

use std::{
    ffi::OsStr,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use inflector::cases::snakecase::to_snake_case;
use openapiv3::OpenAPI;
use serde::Deserialize;

fn save<P>(p: P, data: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let p = p.as_ref();
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(p)?;
    f.write_all(data.as_bytes())?;
    f.flush()?;
    Ok(())
}

fn load<P, T>(p: P) -> Result<T>
where
    P: AsRef<Path>,
    for<'de> T: Deserialize<'de>,
{
    let p = p.as_ref();
    let f = File::open(p)?;
    if let Some(ext) = p.extension() {
        if ext == OsStr::new("yaml") || ext == OsStr::new("yml") {
            return Ok(serde_yaml::from_reader(f)?);
        }
    }
    Ok(serde_json::from_reader(f)?)
}

fn load_api<P>(p: P) -> Result<OpenAPI>
where
    P: AsRef<Path>,
{
    load(p)
}

trait ExtractJsonMediaType {
    fn is_binary(&self) -> Result<bool>;
    fn content_json(&self) -> Result<openapiv3::MediaType>;
}

impl ExtractJsonMediaType for openapiv3::Response {
    fn content_json(&self) -> Result<openapiv3::MediaType> {
        // We do not need to check the length of the content because there might be
        // more than one. For example, if xml or some other format is also defined.
        if let Some(mt) = self.content.get("application/json") {
            Ok(mt.clone())
        } else {
            bail!(
                "could not find application/json, only found {}",
                self.content.keys().next().unwrap()
            );
        }
    }

    fn is_binary(&self) -> Result<bool> {
        if self.content.is_empty() {
            /*
             * XXX If there are no content types, I guess it is not binary?
             */
            return Ok(false);
        }

        // We do not need to check the length of the content because there might be
        // more than one. For example, if xml or some other format is also defined.
        if let Some(mt) = self.content.get("application/octet-stream") {
            if !mt.encoding.is_empty() {
                bail!("XXX encoding");
            }

            if let Some(s) = &mt.schema {
                use openapiv3::{SchemaKind, StringFormat, Type, VariantOrUnknownOrEmpty::Item};

                if let Ok(s) = s.item() {
                    if s.schema_data.nullable {
                        bail!("XXX nullable binary?");
                    }
                    if s.schema_data.default.is_some() {
                        bail!("XXX default binary?");
                    }
                    if s.schema_data.discriminator.is_some() {
                        bail!("XXX binary discriminator?");
                    }
                    match &s.schema_kind {
                        SchemaKind::Type(Type::String(st)) => {
                            if st.min_length.is_some() || st.max_length.is_some() {
                                bail!("binary min/max length");
                            }
                            if !matches!(st.format, Item(StringFormat::Binary)) {
                                bail!("expected binary format string, got {:?}", st.format);
                            }
                            if st.pattern.is_some() {
                                bail!("XXX pattern");
                            }
                            if !st.enumeration.is_empty() {
                                bail!("XXX binary enumeration {:?}", st);
                            }
                            return Ok(true);
                        }
                        x => {
                            bail!("XXX schemakind type {:?}", x);
                        }
                    }
                } else {
                    return Ok(false);
                }
            } else {
                bail!("binary thing had no schema?");
            }
        }

        Ok(false)
    }
}

impl ExtractJsonMediaType for openapiv3::RequestBody {
    fn content_json(&self) -> Result<openapiv3::MediaType> {
        // We do not need to check the length of the content because there might be
        // more than one. For example, if xml or some other format is also defined.
        if let Some(mt) = self.content.get("application/json") {
            Ok(mt.clone())
        } else {
            bail!(
                "could not find application/json, only found {}",
                self.content.keys().next().unwrap()
            );
        }
    }

    fn is_binary(&self) -> Result<bool> {
        if self.content.is_empty() {
            /*
             * XXX If there are no content types, I guess it is not binary?
             */
            return Ok(false);
        }

        // We do not need to check the length of the content because there might be
        // more than one. For example, if xml or some other format is also defined.
        if let Some(mt) = self.content.get("application/octet-stream") {
            if !mt.encoding.is_empty() {
                bail!("XXX encoding");
            }

            if let Some(s) = &mt.schema {
                use openapiv3::{SchemaKind, StringFormat, Type, VariantOrUnknownOrEmpty::Item};

                if let Ok(s) = s.item() {
                    if s.schema_data.nullable {
                        bail!("XXX nullable binary?");
                    }
                    if s.schema_data.default.is_some() {
                        bail!("XXX default binary?");
                    }
                    if s.schema_data.discriminator.is_some() {
                        bail!("XXX binary discriminator?");
                    }
                    match &s.schema_kind {
                        SchemaKind::Type(Type::String(st)) => {
                            if st.min_length.is_some() || st.max_length.is_some() {
                                bail!("binary min/max length");
                            }
                            if !matches!(st.format, Item(StringFormat::Binary)) {
                                bail!("expected binary format string, got {:?}", st.format);
                            }
                            if st.pattern.is_some() {
                                bail!("XXX pattern");
                            }
                            if !st.enumeration.is_empty() {
                                bail!("XXX enumeration");
                            }
                            return Ok(true);
                        }
                        x => {
                            bail!("XXX schemakind type {:?}", x);
                        }
                    }
                } else {
                    return Ok(false);
                }
            } else {
                bail!("binary thing had no schema?");
            }
        }

        Ok(false)
    }
}

trait ReferenceOrExt<T> {
    fn item(&self) -> Result<&T>;
}

impl<T> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item(&self) -> Result<&T> {
        match self {
            openapiv3::ReferenceOr::Item(i) => Ok(i),
            openapiv3::ReferenceOr::Reference { reference } => {
                bail!("reference not supported here: {}", reference);
            }
        }
    }
}

fn gen(api: &OpenAPI) -> Result<String> {
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

fn clean_tag_name(s: &str) -> String {
    let result = inflector::cases::snakecase::to_snake_case(s);

    if result == "oauth_2" {
        "oauth2".to_string()
    } else {
        result
    }
}
