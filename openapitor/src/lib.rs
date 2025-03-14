//! A library for generating rust client sdks from OpenAPI specs.
#![deny(missing_docs)]

pub mod client;
pub mod functions;
pub mod template;
#[cfg(test)]
mod tests;
pub mod types;

#[macro_use]
extern crate quote;

use std::{collections::HashMap, fs, io::Write};

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

/// Parse an OpenAPI v3 spec JSON string as an OpenAPI struct.
pub fn load_json_spec(s: &str) -> Result<openapiv3::OpenAPI> {
    serde_json::from_str(s).map_err(|e| anyhow::anyhow!(e))
}

/// Parse an OpenAPI v3 spec YAML string as an OpenAPI struct.
pub fn load_yaml_spec(s: &str) -> Result<openapiv3::OpenAPI> {
    serde_yaml::from_str(s).map_err(|e| anyhow::anyhow!(e))
}

/// Parse a file as an OpenAPI spec.
pub fn load_api<P>(p: P) -> Result<openapiv3::OpenAPI>
where
    P: AsRef<std::path::Path>,
{
    let p = p.as_ref();
    // Read the file into a string.
    let contents = fs::read_to_string(p)?;
    if let Some(ext) = p.extension() {
        if ext == std::ffi::OsStr::new("yaml") || ext == std::ffi::OsStr::new("yml") {
            return load_yaml_spec(&contents);
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
    a("#![allow(elided_named_lifetimes)]");
    a("#![allow(missing_docs)]"); // TODO: Make this a deny.
    a("#![allow(unused_imports)]");
    a("#![allow(clippy::needless_lifetimes)]"); // TODO: Fix this.
    a("#![allow(clippy::too_many_arguments)]"); // TODO: Fix this.

    a("#![cfg_attr(docsrs, feature(doc_cfg))]");
    a("");

    // Write our persistent modules.
    for module in persistent_modules() {
        if module == "tests" {
            a("#[cfg(test)]");
        }
        a(&format!("mod {module};"));
        // Ensure that if there's no file, an empty file is created.
        std::process::Command::new("touch")
            .arg(format!("{}/src/{module}.rs", opts.output.display()))
            .spawn()?
            .wait()?;
    }

    // Hopefully there is never a "tag" named after these reserved libs.
    a("pub mod types;");

    // If we have any options that would require us to generate a utils file
    // we need include it in the client.
    if opts.needs_utils_file() {
        a("pub mod utils;");
    }

    // First get the tags for all the paths, then later we can ignore tags that
    // have no paths.
    let default_tag = "default".to_string();
    let mut tags_with_paths = Vec::<String>::new();
    for (_name, path) in spec.paths.iter() {
        let op = path.item()?;

        let mut get_tags = |op: Option<&openapiv3::Operation>| -> Result<()> {
            // Ensure we have an operation for this path and method, otherwise return early.
            let op = if let Some(op) = op {
                op
            } else {
                return Ok(());
            };

            // Some specs don't have tags at all, so just use default for now.
            let tag = op.tags.first().unwrap_or(&default_tag);

            // Add our tag to our vector.
            tags_with_paths.push(tag.to_string());

            Ok(())
        };

        get_tags(op.get.as_ref())?;
        get_tags(op.put.as_ref())?;
        get_tags(op.post.as_ref())?;
        get_tags(op.delete.as_ref())?;
        get_tags(op.head.as_ref())?;
        get_tags(op.patch.as_ref())?;
        get_tags(op.trace.as_ref())?;
    }

    // Combine our tags with our tags from the paths, because some APIs do not add the
    // tags to the top level tags components.
    let mut tags = spec.tags.clone();
    for tag in &tags_with_paths {
        // Make sure we don't add it twice or that we don't already have it.
        if !tags
            .iter()
            .map(|t| t.name.to_string())
            .any(|x| clean_tag_name(&x) == clean_tag_name(tag))
        {
            // Add this tag to our list of tags.
            tags.push(openapiv3::Tag {
                name: tag.to_string(),
                description: Default::default(),
                external_docs: Default::default(),
                extensions: Default::default(),
            })
        }
    }

    /*
     * Import the module for each tag.
     * Tags are how functions are grouped.
     */
    for tag in tags.iter() {
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
        a("#[cfg(feature = \"requests\")]");
        a(&format!("pub mod {};", clean_tag_name(&tag.name)));
    }

    a("");

    // Print the client template.
    a(&crate::client::generate_client(opts));

    a("");

    /*
     * Generate a function for each tag.
     * Tags are how functions are grouped.
     */
    for tag in tags.iter() {
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
    let out = crate::internal_generate(spec, opts)?;

    // Create the top-level crate directory:
    fs::create_dir_all(&opts.output)?;

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
    fs::create_dir_all(&src)?;

    // Clean up any old files we might have.
    // Walk the src/ directory and delete any files that aren't a persistent module.
    let src_list = fs::read_dir(&src)?;
    for file in src_list {
        let file = file?;
        // Return early if it is a directory.
        if file.file_type()?.is_dir() {
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
        if persistent_modules.contains(&file_name.as_str()) {
            continue;
        }

        // Delete the file.
        fs::remove_file(file.path())?;
    }

    // Create the Rust source file containing the generated client.
    let lib = format!("{}\n{}", docs, out);
    let mut librs = src.clone();
    librs.push("lib.rs");
    crate::save(librs, lib.as_str())?;

    if let Some(utils) = crate::template::generate_utils(opts) {
        let mut utilsrs = src.clone();
        utilsrs.push("utils.rs");
        crate::save(utilsrs, utils.as_str())?;
    }

    // Create the Rust source types file containing the generated types.
    let mut type_space = crate::types::generate_types(spec, opts.clone())?;

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
    let mut typesrs = src.clone();
    typesrs.push("types.rs");
    crate::save(
        typesrs,
        crate::types::get_text_fmt(&type_space.rendered)?.as_str(),
    )?;

    // Run fmt in our output directory.
    run_cargo_fmt(opts)?;

    // Run clippy in our output directory.
    if opts.clippy_fix {
        run_cargo_clippy(opts)?;
    }

    // Also add our installation information to the modified_spec.
    let mut extension: HashMap<String, String> = HashMap::new();
    extension.insert(
        "install".to_string(),
        format!(
            "[dependencies]\n{} = \"{}\"",
            opts.name.replace('_', "-").to_lowercase(),
            opts.target_version
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
#[command(version = clap::crate_version!(), author = clap::crate_authors!("\n"))]
pub struct Opts {
    /// Print debug info.
    #[arg(short = 'D', long)]
    pub debug: bool,

    /// Print logs as json.
    #[arg(short, long)]
    pub json: bool,

    /// The input OpenAPI definition document (JSON | YAML).
    // TODO: We could also load from a URL.
    #[arg(short, long, required = true)]
    pub input: std::path::PathBuf,

    /// The output directory for our generated client.
    #[arg(short, long, default_value = ".", required = true)]
    pub output: std::path::PathBuf,

    /// The base url for the API.
    #[arg(short, long, required = true)]
    pub base_url: url::Url,

    /// The crate name for our generated client.
    #[arg(short, long, required = true)]
    pub name: String,

    /// The crate version for our generated client.
    #[arg(short, long, required = true)]
    pub target_version: String,

    /// The crate description for our generated client.
    #[arg(short, long, required = true)]
    pub description: String,

    /// The link to a hosted version of the spec.
    #[arg(short, long)]
    pub spec_url: Option<String>,

    /// The repo name, formatted like `{owner}/{name}`, if hosted on GitHub.
    #[arg(short, long)]
    pub repo_name: Option<String>,

    /// The token endpoint, if this client uses OAuth 2.0.
    #[arg(long)]
    pub token_endpoint: Option<url::Url>,

    /// The user consent endpoint, if this client uses OAuth 2.0.
    #[arg(long)]
    pub user_consent_endpoint: Option<url::Url>,

    /// The date-time format for the API, defaults to Rust rfc3339 parser
    #[arg(long)]
    pub date_time_format: Option<String>,

    /// Use basic auth for authentication instead of bearer tokens
    #[arg(long)]
    pub basic_auth: bool,

    /// Default timeout on the client
    #[arg(long)]
    pub request_timeout_seconds: u64,

    /// An additional env variable prefix (the default is the name of the package).
    /// If given, both the package name and this prefix will be used.
    #[arg(long)]
    pub add_env_prefix: Option<String>,

    /// Run clippy --fix on the output code
    #[arg(long, default_value = "false")]
    pub clippy_fix: bool,
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

    /// Get whether these options require a utils.rs file to be generated.
    pub fn needs_utils_file(&self) -> bool {
        self.date_time_format.is_some()
    }
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            debug: Default::default(),
            json: Default::default(),
            input: Default::default(),
            output: Default::default(),
            base_url: url::Url::parse("http://example.com").unwrap(),
            name: Default::default(),
            target_version: Default::default(),
            description: Default::default(),
            spec_url: Default::default(),
            repo_name: Default::default(),
            token_endpoint: Default::default(),
            user_consent_endpoint: Default::default(),
            date_time_format: Default::default(),
            basic_auth: Default::default(),
            clippy_fix: false,
            add_env_prefix: Default::default(),
            request_timeout_seconds: 60,
        }
    }
}

/// Return a list of the persistent modules.
/// These are modules we do not nuke at generation time.
fn persistent_modules() -> Vec<&'static str> {
    vec!["tests", "methods"]
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
edition = "2021"
license = "MIT"

[dependencies]
anyhow = "1"
async-trait = {{ version = "^0.1.53", optional = true }}
base64 = "0.22"
bigdecimal = {{ version = "0.4", features = ["serde"] }}
bytes = {{ version = "1", features = ["serde"] }}
clap = {{ version = "4.2.4", features = ["cargo", "derive", "env", "unicode"], optional = true }}
data-encoding = "^2.3.2"
dirs = {{ version = "^5.0.1", optional = true }}
format_serde_error = {{ version = "^0.3.0", optional = true }}
futures = {{ version = "0.3.26", optional = true }}
http = {{ version = "1", optional = true }}
itertools = "0.13.0"
log = {{ version = "^0.4", features = ["serde"], optional = true }}
mime_guess = "2.0.4"
parse-display = "0.10.0"
phonenumber = "0.3.5"
rand = {{ version = "0.9", optional = true }}
getrandom = {{ version = "0.3" }}
reqwest = {{ version = "0.12.14", default-features = false, features = ["json", "multipart", "rustls-tls"], optional = true }}
reqwest-conditional-middleware = {{ version = "0.4", optional = true }}
reqwest-middleware = {{ version = "0.4", optional = true, features = ["json", "multipart", "http2", "rustls-tls"] }}
reqwest-retry = {{ version = "0.7", optional = true }}
reqwest-tracing = {{ version = "0.5.4", features = ["opentelemetry_0_24"], optional = true }}
schemars = {{ version = "0.8.17", features = ["bigdecimal04", "bytes", "chrono", "url", "uuid1"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_bytes = "0.11"
serde_json = "1"
serde_urlencoded = {{ version = "^0.7", optional = true }}
tabled = {{ version = "0.18.0", features = ["ansi"], optional = true }}
thiserror = "2"
tracing = {{ version = "^0.1", optional = true }}
url = {{ version = "2", features = ["serde"] }}
uuid = {{ version = "1", features = ["serde", "v4", "v7"] }}

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
chrono = {{ version = "0.4", default-features = false, features = ["now", "serde", "std"] }}
tokio = {{ version = "1.38.0", features = ["rt", "macros"] }}

[target.'cfg(target_arch = "wasm32")'.dependencies]
chrono = {{ version = "0.4", default-features = false, features = ["serde", "std"] }}

[dev-dependencies]
expectorate = "1"
futures-util = "^0.3.26"
pretty_assertions = "1"
rand = "0.9"
tokio = {{ version = "1.38.0", features = ["rt", "macros"] }}
tokio-tungstenite = "0.24"

[features]
default = ["requests", "retry"]
clap = ["dep:clap"]
tabled = ["dep:tabled"]
requests = ["dep:async-trait", "dep:format_serde_error", "dep:futures", "dep:http", "dep:log", "dep:rand", "dep:reqwest", "dep:serde_urlencoded", "dep:tracing"]
retry = ["dep:reqwest-conditional-middleware", "dep:reqwest-retry", "dep:reqwest-middleware", "dep:reqwest-tracing"]
js = ["uuid/js", "getrandom/wasm_js"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
"#,
        opts.name, opts.description, opts.target_version, opts.name, repo_info,
    )
}

fn run_cargo_fmt(opts: &Opts) -> Result<()> {
    log::info!("Running `cargo fmt`...");

    // Shell out and run cargo fmt on the output directory.
    let output = if opts.output.display().to_string() == "." {
        "".to_string()
    } else {
        opts.output.display().to_string()
    };

    let mut cmd = std::process::Command::new("cargo");
    cmd.args([
        "fmt",
        "--",
        "--config",
        "format_code_in_doc_comments=true,imports_granularity=Crate,\
         group_imports=StdExternalCrate,format_strings=true,max_width=100",
    ])
    .current_dir(output);

    let output = cmd.output()?;
    if !output.status.success() {
        anyhow::bail!(
            "cargo fmt failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn run_cargo_clippy(opts: &Opts) -> Result<()> {
    log::info!("Running `cargo clippy`...");

    // Shell out and run cargo clippy on the output directory.
    let output = if opts.output.display().to_string() == "." {
        "".to_string()
    } else {
        opts.output.display().to_string()
    };

    let mut cmd = std::process::Command::new("cargo");
    cmd.args([
        "clippy",
        "--fix",
        "--features",
        "clap",
        "--features",
        "tabled",
        "--allow-dirty",
        "--allow-no-vcs", // We need this arg for the tests.
    ])
    .current_dir(output);

    let output = cmd.output()?;
    if !output.status.success() {
        anyhow::bail!(
            "cargo clippy failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Opts::command().debug_assert();
}
