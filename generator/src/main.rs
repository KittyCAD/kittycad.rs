use std::fs;

use anyhow::Result;
use clap::Parser;
use slog::Drain;

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
    #[clap(short, long, parse(from_os_str), required = true)]
    pub input: std::path::PathBuf,

    /// The output directory for our generated client.
    #[clap(short, long, parse(from_os_str), default_value = ".", required = true)]
    pub output: std::path::PathBuf,

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

fn main() -> Result<()> {
    // Parse the command line arguments.
    let opts: Opts = Opts::parse();

    // Setup our logger.
    let drain = opts.create_logger();
    let logger = slog::Logger::root(drain, slog::o!());

    let _scope_guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init()?;

    // Let's read the spec from the file.
    let spec = generator::load_api(&opts.input)?;

    // Generate the client.
    let out = generator::generate(&spec)?;

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
    generator::save(&toml, tomlout.as_str())?;

    /*
     * Generate our documentation for the library.
     */
    let docs = generator::template::generate_docs(
        &spec,
        &inflector::cases::snakecase::to_snake_case(&opts.name),
        &opts.version,
        &opts.spec_url,
    )?;
    let mut readme = opts.output.clone();
    readme.push("README.md");
    generator::save(
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
    generator::save(librs, lib.as_str())?;

    // Create the Rust source types file containing the generated types.
    let types = generator::types::generate_types(&spec)?;
    let mut typesrs = src.clone();
    typesrs.push("types.rs");
    generator::save(typesrs, types.as_str())?;

    // TODO: cleanup old tag files.

    // Create the Rust source files for each of the tags functions.
    match generator::functions::generate_files(&spec) {
        Ok(files) => {
            // We have a map of our files, let's write to them.
            for (f, content) in files {
                let mut tagrs = src.clone();
                tagrs.push(format!("{}.rs", generator::clean_tag_name(&f)));

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
                    generator::types::proper_name(&f),
                    generator::types::proper_name(&f),
                    generator::types::proper_name(&f),
                    content,
                );
                generator::save(tagrs, output.as_str())?;
            }
        }
        Err(e) => {
            println!("generate_files fail: {:?}", e);
        }
    }

    Ok(())
}
