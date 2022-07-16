mod client;
mod functions;
mod template;

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

    a("mod progenitor_support {");
    a("    use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};");
    a("");
    /*
     * The percent-encoding crate abrogates its responsibility for providing
     * useful percent-encoding sets, so we must provide one for path components
     * here.
     */
    a("    const PATH_SET: &AsciiSet = &CONTROLS");
    /*
     * The query percent-encode set is the C0 control percent-encode set and
     * U+0020 SPACE, U+0022 ("), U+0023 (#), U+003C (<), and U+003E (>).
     */
    a("        .add(b' ')");
    a("        .add(b'\"')");
    a("        .add(b'#')");
    a("        .add(b'<')");
    a("        .add(b'>')");
    /*
     * The path percent-encode set is the query percent-encode set and U+003F
     * (?), U+0060 (`), U+007B ({), and U+007D (}).
     */
    a("        .add(b'?')");
    a("        .add(b'`')");
    a("        .add(b'{')");
    a("        .add(b'}');");
    a("");
    a("    #[allow(dead_code)]");
    a("    pub(crate) fn encode_path(pc: &str) -> String {");
    a("        utf8_percent_encode(pc, PATH_SET).to_string()");
    a("    }");
    a("}");
    a("");

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

fn main() -> Result<()> {
    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.reqopt(
        "i",
        "",
        "OpenAPI definition document (JSON | YAML)",
        "INPUT",
    );
    opts.reqopt("o", "", "Generated Rust crate directory", "OUTPUT");
    opts.reqopt("n", "", "Target Rust crate name", "CRATE");
    opts.reqopt("v", "", "Target Rust crate version", "VERSION");
    opts.reqopt("d", "", "Target Rust crate description", "DESCRIPTION");
    opts.reqopt("", "spec-link", "Link to the spec", "SPEC_LINK");
    opts.optflag("", "debug", "Print debug output");

    let args = match opts.parse(std::env::args().skip(1)) {
        Ok(args) => {
            if !args.free.is_empty() {
                eprintln!("{}", opts.usage("progenitor"));
                bail!("unexpected positional arguments");
            }
            args
        }
        Err(e) => {
            eprintln!("{}", opts.usage("progenitor"));
            bail!(e);
        }
    };

    let input_spec = args.opt_str("i").unwrap();

    let api = load_api(&input_spec)?;

    let debug = |s: &str| {
        if args.opt_present("debug") {
            println!("{}", s);
        }
    };

    debug("");

    let name = args.opt_str("n").unwrap();
    let version = args.opt_str("v").unwrap();
    let output_dir = args.opt_str("o").unwrap();
    let spec_link = args.opt_str("spec-link").unwrap();

    let fail = match gen(&api) {
        Ok(out) => {
            let description = args.opt_str("d").unwrap();

            /*
             * Create the top-level crate directory:
             */
            let root = PathBuf::from(&output_dir);
            std::fs::create_dir_all(&root)?;

            /*
             * Write the Cargo.toml file:
             */
            let mut toml = root.clone();
            toml.push("Cargo.toml");
            let tomlout = format!(
                r#"[package]
name = "{}"
description = "{}"
version = "{}"
documentation = "https://docs.rs/{}/"
repository = "https://github.com/Kittycad/kittycad.rs/tree/main/{}"
readme = "README.md"
edition = "2018"
license = "MIT"

[dependencies]
anyhow = "1"
async-trait = "^0.1.53"
bytes = {{ version = "1", features = ["serde"] }}
clap = {{ version = "^3.2.12", features = ["cargo", "derive", "env", "unicode"] }}
chrono = {{ version = "0.4", features = ["serde"] }}
chrono-humanize = "^0.2.1"
data-encoding = "^2.3.2"
dirs = {{ version = "^4.0.0", optional = true }}
format_serde_error = "^0.3.0"
http = "^0.2.8"
hyperx = "1"
log = {{ version = "^0.4", features = ["serde"] }}
mime = "0.3"
parse-display = "^0.5"
percent-encoding = "2.1"
reqwest = {{ version = "0.11", default-features = false, features = ["json", "multipart", "rustls-tls"] }}
schemars = {{ version = "0.8", features = ["bytes", "chrono", "url", "uuid1"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
serde_with = "1"
serde_urlencoded = "^0.7"
tabled = {{ version = "0.7.0", features = ["color"] }}
thiserror = "^1"
url = {{ version = "2", features = ["serde"] }}
uuid = {{ version = "1", features = ["serde", "v4"] }}

[dev-dependencies]
pretty_assertions = "1"
tokio = {{ version = "1.20.0", features = ["full"] }}

[features]
# enable etag-based http_cache functionality
httpcache = ["dirs"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
"#,
                name, description, version, name, output_dir,
            );
            save(&toml, tomlout.as_str())?;

            /*
             * Generate our documentation for the library.
             */
            let docs = template::generate_docs(&api, &to_snake_case(&name), &version, &spec_link)?;
            let mut readme = root.clone();
            readme.push("README.md");
            save(
                readme,
                // Add a title to the README.md so it looks nicer in GitHub.
                &format!(
                    "# `{}`\n\n{}",
                    name,
                    docs.replace("//! ", "").replace("//!", "").as_str()
                ),
            )?;

            /*
             * Create the src/ directory:
             */
            let mut src = root;
            src.push("src");
            std::fs::create_dir_all(&src)?;

            /*
             * Create the Rust source file containing the generated client:
             */
            let lib = format!("{}\n{}", docs, out);
            let mut librs = src.clone();
            librs.push("lib.rs");
            save(librs, lib.as_str())?;

            /*
             * Create the Rust source types file containing the generated types:
             */
            let types = types::generate_types(&api)?;
            let mut typesrs = src.clone();
            typesrs.push("types.rs");
            save(typesrs, types.as_str())?;

            /*
             * Create the Rust source files for each of the tags functions:
             */

            match functions::generate_files(&api) {
                Ok(files) => {
                    // We have a map of our files, let's write to them.
                    for (f, content) in files {
                        let mut tagrs = src.clone();
                        tagrs.push(format!("{}.rs", clean_tag_name(&f)));

                        let output = format!(
                            r#"use anyhow::Result;

            use crate::Client;

            pub struct {} {{
                pub client: Client,
            }}

            impl {} {{
                #[doc(hidden)]
                pub fn new(client: Client) -> Self
                {{
                    {} {{
                        client,
                    }}
                }}

                {}
            }}"#,
                            types::proper_name(&f),
                            types::proper_name(&f),
                            types::proper_name(&f),
                            content,
                        );
                        save(tagrs, output.as_str())?;
                    }

                    false
                }
                Err(e) => {
                    println!("generate_files fail: {:?}", e);
                    true
                }
            }
        }
        Err(e) => {
            println!("gen fail: {:?}", e);
            true
        }
    };

    if fail {
        bail!("generation experienced errors");
    }

    Ok(())
}
