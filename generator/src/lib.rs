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
use clap::Parser;
use slog::Drain;

use crate::types::exts::ReferenceOrExt;

/// Save a file.
fn save<P>(p: P, data: &str) -> Result<()>
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

fn internal_generate(spec: &openapiv3::OpenAPI, opts: &Opts) -> Result<String> {
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
    for (name, path) in spec.paths.iter() {
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
    for tag in spec.tags.iter() {
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
    a(&crate::client::generate_client(&opts.name, &opts.base_url));

    a("");

    /*
     * Generate a function for each tag.
     * Tags are how functions are grouped.
     */
    for tag in spec.tags.iter() {
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
fn clean_tag_name(s: &str) -> String {
    let result = inflector::cases::snakecase::to_snake_case(s);

    if result == "oauth_2" {
        "oauth2".to_string()
    } else {
        result
    }
}

/// Generate the client library.
pub fn generate(spec: &openapiv3::OpenAPI, opts: &Opts) -> Result<()> {
    // Generate the client.
    let out = crate::internal_generate(&spec, opts)?;

    // Create the top-level crate directory:
    fs::create_dir_all(&opts.output)?;

    // Write the Cargo.toml file:
    let mut toml = opts.output.clone();
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
        opts.name,
        opts.description,
        opts.version,
        opts.name,
        opts.output.display(),
    );
    crate::save(&toml, tomlout.as_str())?;

    /*
     * Generate our documentation for the library.
     */
    let docs = crate::template::generate_docs(
        &spec,
        &inflector::cases::snakecase::to_snake_case(&opts.name),
        &opts.description,
        &opts.version,
        &opts.spec_url,
    )?;
    let mut readme = opts.output.clone();
    readme.push("README.md");
    crate::save(
        readme,
        // Add a title to the README.md so it looks nicer in GitHub.
        &format!(
            "# `{}`\n\n{}",
            opts.name,
            docs.replace("//! ", "").replace("//!", "").as_str()
        ),
    )?;

    // Create the src/ directory.
    let mut src = opts.output.clone();
    src.push("src");
    fs::create_dir_all(&src)?;

    // Create the Rust source file containing the generated client.
    let lib = format!("{}\n{}", docs, out);
    let mut librs = src.clone();
    librs.push("lib.rs");
    crate::save(librs, lib.as_str())?;

    // Create the Rust source types file containing the generated types.
    let types = crate::types::generate_types(&spec)?;
    let mut typesrs = src.clone();
    typesrs.push("types.rs");
    crate::save(typesrs, types.as_str())?;

    // TODO: cleanup old tag files.

    // Create the Rust source files for each of the tags functions.
    match crate::functions::generate_files(&spec) {
        Ok(files) => {
            // We have a map of our files, let's write to them.
            for (f, content) in files {
                let mut tagrs = src.clone();
                tagrs.push(format!("{}.rs", crate::clean_tag_name(&f)));

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
                    crate::types::proper_name(&f),
                    crate::types::proper_name(&f),
                    crate::types::proper_name(&f),
                    content,
                );
                crate::save(tagrs, output.as_str())?;
            }
        }
        Err(e) => {
            println!("generate_files fail: {:?}", e);
        }
    }

    Ok(())
}

/// The options for our generator.
#[derive(Parser, Debug, Clone)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!("\n"))]
pub struct Opts {
    /// Print debug info.
    #[clap(short = 'D', long)]
    pub debug: bool,

    /// Print logs as json.
    #[clap(short, long)]
    pub json: bool,

    /// The input OpenAPI definition document (JSON | YAML).
    // TODO: We could also load from a URL.
    #[clap(short, long, parse(from_os_str), required = true)]
    pub input: std::path::PathBuf,

    /// The output directory for our generated client.
    #[clap(short, long, parse(from_os_str), default_value = ".", required = true)]
    pub output: std::path::PathBuf,

    /// The base url for the API.
    #[clap(short, long, required = true)]
    pub base_url: url::Url,

    /// The crate name for our generated client.
    #[clap(short, long, required = true)]
    pub name: String,

    /// The crate version for our generated client.
    #[clap(short, long, required = true)]
    pub version: String,

    /// The crate description for our generated client.
    #[clap(short, long, required = true)]
    pub description: String,

    /// The link to a hosted version of the spec.
    #[clap(short, long)]
    pub spec_url: Option<String>,
}

impl Opts {
    /// Setup our logger.
    pub fn create_logger(&self) -> slog::Logger {
        if self.json {
            let drain = slog_json::Json::default(std::io::stderr()).fuse();
            self.async_root_logger(drain)
        } else {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            self.async_root_logger(drain)
        }
    }

    fn async_root_logger<T>(&self, drain: T) -> slog::Logger
    where
        T: slog::Drain + Send + 'static,
        <T as slog::Drain>::Err: std::fmt::Debug,
    {
        let level = if self.debug {
            slog::Level::Debug
        } else {
            slog::Level::Info
        };

        let level_drain = slog::LevelFilter(drain, level).fuse();
        let async_drain = slog_async::Async::new(level_drain).build().fuse();
        slog::Logger::root(async_drain, slog::o!())
    }
}
