//! A library for generating rust client sdks from OpenAPI specs.
#![deny(missing_docs)]

pub mod client;
pub mod functions;
pub mod template;
#[cfg(test)]
pub mod tests;
pub mod types;

#[macro_use]
extern crate quote;

use std::{collections::HashMap, io::Write};

use anyhow::Result;
use clap::Parser;
use slog::Drain;
use tokio::fs;

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

/// Parse an OpenAPI v3 spec JSON string as an OpenAPI struct.
pub fn load_json_spec(s: &str) -> Result<openapiv3::OpenAPI> {
    serde_json::from_str(s).map_err(|e| anyhow::anyhow!(e))
}

/// Parse a file as an OpenAPI spec.
pub async fn load_api<P>(p: P) -> Result<openapiv3::OpenAPI>
where
    P: AsRef<std::path::Path>,
{
    let p = p.as_ref();
    // Read the file into a string.
    let contents = fs::read_to_string(p).await?;
    if let Some(ext) = p.extension() {
        if ext == std::ffi::OsStr::new("yaml") || ext == std::ffi::OsStr::new("yml") {
            return Ok(serde_yaml::from_str(&contents)?);
        }
    }

    load_json_spec(&contents)
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
    a("#![allow(clippy::needless_lifetimes)]"); // TODO: Fix this.

    a("#![cfg_attr(docsrs, feature(doc_cfg))]");
    a("");

    // Write our persistent modules.
    for module in persistent_modules() {
        if module == "tests" {
            a("#[cfg(test)]");
        }
        a(&format!("mod {};", module));
    }

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
pub async fn generate(spec: &openapiv3::OpenAPI, opts: &Opts) -> Result<()> {
    // Generate the client.
    let out = crate::internal_generate(spec, opts)?;

    // Create the top-level crate directory:
    fs::create_dir_all(&opts.output).await?;

    // Write the Cargo.toml file:
    let mut toml = opts.output.clone();
    toml.push("Cargo.toml");
    let tomlout = generate_cargo_toml(opts);
    crate::save(&toml, tomlout.as_str())?;

    /*
     * Generate our documentation for the library.
     */
    let docs = crate::template::generate_docs(spec, opts)?;
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
    fs::create_dir_all(&src).await?;

    // Clean up any old files we might have.
    // Walk the src/ directory and delete any files that aren't a persistent module.
    let mut src_list = fs::read_dir(&src).await?;
    while let Some(file) = src_list.next_entry().await? {
        // Return early if it is a directory.
        if file.file_type().await?.is_dir() {
            continue;
        }
        // Get the file name.
        let file_name = file
            .file_name()
            .to_str()
            .ok_or_else(|| {
                anyhow::anyhow!("failed to get file name for {}", file.path().display())
            })?
            .trim_end_matches(".rs")
            .to_string();

        let persistent_modules = persistent_modules();
        if persistent_modules.contains(&file_name) {
            continue;
        }

        // Delete the file.
        fs::remove_file(file.path()).await?;
    }

    // Create the Rust source file containing the generated client.
    let lib = format!("{}\n{}", docs, out);
    let mut librs = src.clone();
    librs.push("lib.rs");
    crate::save(librs, lib.as_str())?;

    // Create the Rust source types file containing the generated types.
    let mut type_space = crate::types::generate_types(spec)?;

    // Create the Rust source files for each of the tags functions.
    let (files, modified_spec) = crate::functions::generate_files(&mut type_space, opts)?;
    // We have a map of our files, let's write to them.
    for (f, content) in files {
        let mut tagrs = src.clone();
        tagrs.push(format!("{}.rs", f));
        let proper_tag_name = crate::types::proper_name(&f);
        let proper_tag_name_ident = format_ident!("{}", proper_tag_name);

        let output = quote! {
            use anyhow::Result;

            use crate::Client;

            #[derive(Clone, Debug)]
            pub struct #proper_tag_name_ident {
                pub client: Client,
            }

            impl #proper_tag_name_ident {
                #[doc(hidden)]
                pub fn new(client: Client) -> Self {
                    Self { client }
                }

                #content
            }
        };
        // TODO: make fmt
        crate::save(tagrs, &crate::types::get_text_fmt(&output)?)?;
    }

    // Save the types, now that we've run the functions.
    //, )
    let mut typesrs = src.clone();
    typesrs.push("types.rs");
    crate::save(
        typesrs,
        crate::types::get_text_fmt(&type_space.rendered)?.as_str(),
    )?;

    // Run fmt in our output directory.
    run_cargo_fmt(opts).await?;

    // Run clippy in our output directory.
    run_cargo_clippy(opts).await?;

    // Also add our installation information to the modified_spec.
    let mut extension: HashMap<String, String> = HashMap::new();
    extension.insert(
        "install".to_string(),
        format!(
            "[dependencies]\n{} = \"{}\"",
            opts.name.replace('_', "-").to_lowercase(),
            opts.version
        ),
    );
    extension.insert(
        "client".to_string(),
        crate::functions::generate_example_client(opts),
    );

    // Add in our version information
    let mut modified_spec = modified_spec;
    modified_spec
        .info
        .extensions
        .insert("x-rust".to_string(), serde_json::json!(extension));

    // Create a JSON patch file with our changes.
    let patch = json_patch::diff(
        &serde_json::to_value(spec)?,
        &serde_json::to_value(modified_spec)?,
    );
    // Save our patch file.
    let mut patch_file = opts.output.clone();
    patch_file.push(format!("{}.rs.patch.json", opts.name));
    crate::save(&patch_file, &serde_json::to_string_pretty(&patch)?)?;
    log::info!("Patch file has been saved to {}", patch_file.display());

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

    /// The repo name, formatted like `{owner}/{name}`, if hosted on GitHub.
    #[clap(short, long)]
    pub repo_name: Option<String>,
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

    /// Get the name of the package.
    /// This is the not the name used in code.
    pub fn package_name(&self) -> String {
        inflector::cases::kebabcase::to_kebab_case(&self.name)
    }

    /// Get the name of the package as it is used in code.
    pub fn code_package_name(&self) -> String {
        inflector::cases::snakecase::to_snake_case(&self.name)
    }
}

/// Return a list of the persistent modules.
/// These are modules we do not nuke at generation time.
fn persistent_modules() -> Vec<String> {
    vec!["tests".to_string()]
}

fn generate_cargo_toml(opts: &Opts) -> String {
    let repo_info = if let Some(repo) = &opts.repo_name {
        let output = if opts.output.display().to_string() == "." {
            "".to_string()
        } else {
            opts.output.display().to_string()
        };

        format!(
            r#"repository = "https://github.com/{}/tree/main/{}""#,
            repo, output
        )
    } else {
        "".to_string()
    };

    format!(
        r#"[package]
name = "{}"
description = "{}"
version = "{}"
documentation = "https://docs.rs/{}"
readme = "README.md"
{}
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
futures = "0.3.21"
http = "^0.2.8"
hyperx = "1"
itertools = "^0.10.3"
log = {{ version = "^0.4", features = ["serde"] }}
mime = "0.3"
parse-display = "^0.5"
percent-encoding = "2.1"
phonenumber = "^0.3.1"
reqwest = {{ version = "0.11", default-features = false, features = ["json", "multipart", "rustls-tls"] }}
schemars = {{ version = "0.8", features = ["bytes", "chrono", "url", "uuid1"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
serde_with = "1"
serde_urlencoded = "^0.7"
tabled = {{ version = "0.8.0", features = ["color"] }}
thiserror = "1"
tracing = "^0.1"
url = {{ version = "2", features = ["serde"] }}
uuid = {{ version = "1", features = ["serde", "v4"] }}

[dev-dependencies]
expectorate = "1"
futures-util = "^0.3.21"
pretty_assertions = "1"
tokio = {{ version = "1.20.0", features = ["full"] }}

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
"#,
        opts.name, opts.description, opts.version, opts.name, repo_info,
    )
}

async fn run_cargo_fmt(opts: &Opts) -> Result<()> {
    log::info!("Running `cargo fmt`...");

    // Shell out and run cargo clippy on the output directory.
    let output = if opts.output.display().to_string() == "." {
        "".to_string()
    } else {
        opts.output.display().to_string()
    };

    let mut cmd = tokio::process::Command::new("cargo");
    cmd.args([
        "+nightly",
        "fmt",
        "--",
        "--config",
        "format_code_in_doc_comments=true,imports_granularity=Crate,group_imports=StdExternalCrate,format_strings=true,max_width=100",
    ])
    .current_dir(output);

    let output = cmd.output().await?;
    if !output.status.success() {
        anyhow::bail!(
            "cargo fmt failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

async fn run_cargo_clippy(opts: &Opts) -> Result<()> {
    log::info!("Running `cargo clippy`...");

    // Shell out and run cargo clippy on the output directory.
    let output = if opts.output.display().to_string() == "." {
        "".to_string()
    } else {
        opts.output.display().to_string()
    };

    let mut cmd = tokio::process::Command::new("cargo");
    cmd.args([
        "clippy",
        "--fix",
        "--allow-dirty",
        "--allow-no-vcs", // We need this arg for the tests.
    ])
    .current_dir(output);

    let output = cmd.output().await?;
    if !output.status.success() {
        anyhow::bail!(
            "cargo clippy failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
