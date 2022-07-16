use anyhow::Result;

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
                anyhow::bail!("unexpected positional arguments");
            }
            args
        }
        Err(e) => {
            eprintln!("{}", opts.usage("progenitor"));
            anyhow::bail!(e);
        }
    };

    let input_spec = args.opt_str("i").unwrap();

    let api = generator::load_api(&input_spec)?;

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

    let fail = match generator::generate(&api) {
        Ok(out) => {
            let description = args.opt_str("d").unwrap();

            /*
             * Create the top-level crate directory:
             */
            let root = std::path::PathBuf::from(&output_dir);
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
            generator::save(&toml, tomlout.as_str())?;

            /*
             * Generate our documentation for the library.
             */
            let docs = generator::template::generate_docs(
                &api,
                &inflector::cases::snakecase::to_snake_case(&name),
                &version,
                &spec_link,
            )?;
            let mut readme = root.clone();
            readme.push("README.md");
            generator::save(
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
            generator::save(librs, lib.as_str())?;

            /*
             * Create the Rust source types file containing the generated types:
             */
            let types = generator::types::generate_types(&api)?;
            let mut typesrs = src.clone();
            typesrs.push("types.rs");
            generator::save(typesrs, types.as_str())?;

            /*
             * Create the Rust source files for each of the tags functions:
             */

            match generator::functions::generate_files(&api) {
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
        anyhow::bail!("generation experienced errors");
    }

    Ok(())
}
