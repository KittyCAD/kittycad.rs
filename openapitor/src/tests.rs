use anyhow::Result;
use test_context::{test_context, AsyncTestContext};

struct TestContext {
    tmp_dir: std::path::PathBuf,
}

impl TestContext {
    pub async fn new() -> Result<Self> {
        // Create a temporary directory for the test.
        let tmp_dir = std::env::temp_dir();
        let tmp_dir = tmp_dir.join(&format!("openapitor-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp_dir)?;
        let src_dir = tmp_dir.clone().join("src");
        std::fs::create_dir_all(&src_dir)?;

        Ok(TestContext { tmp_dir })
    }
}

#[async_trait::async_trait]
impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        TestContext::new().await.unwrap()
    }

    async fn teardown(self) {
        println!("Removing {}", self.tmp_dir.display());
        // Delete the temporary directory.
        std::fs::remove_dir_all(&self.tmp_dir).unwrap();
    }
}

#[test_context(TestContext)]
#[tokio::test]
async fn test_kittycad_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.kittycad.io".parse().unwrap(),
        name: "kittycad".to_string(),
        version: "1.0.0".to_string(),
        description: "KittyCad is a tool for generating 3D models of cats.".to_string(),
        spec_url: Some("https://api.kittycad.io".to_string()),
        repo_name: Some("kittycad/kittycad.rs".to_string()),
    };

    // Load our spec.
    let spec = crate::load_json_spec(include_str!("../../spec.json")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/kittycad.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(&test_file_path, test_file).unwrap();

    // Move the assets directory over so we have files to run.
    let assets_dir = ctx.tmp_dir.join("assets");
    std::fs::create_dir_all(&assets_dir).unwrap();
    // Get the current contents of the assets directory.
    let mut assets_dir_contents =
        std::fs::read_dir(&std::env::current_dir().unwrap().join("../assets")).unwrap();
    // Move each file over.
    while let Some(Ok(entry)) = assets_dir_contents.next() {
        let dest = assets_dir.join(entry.file_name());
        println!("Moving {} => {}", entry.path().display(), dest.display());
        std::fs::copy(&entry.path(), &dest).unwrap();
    }

    // Generate the library.
    crate::generate(&spec, &opts).await.unwrap();

    // Run tests.
    run_cargo_test(&opts).await.unwrap();
}

#[test_context(TestContext)]
#[tokio::test]
#[ignore] // TODO: eventually make this work
async fn test_github_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.github.com".parse().unwrap(),
        name: "octorust".to_string(),
        version: "1.0.0".to_string(),
        description: "GitHub is where we push our code and you do too!".to_string(),
        spec_url: Some("https://github.com/github/rest-api-description/raw/main/descriptions/api.github.com/api.github.com.json".to_string()),
        repo_name: Some("kittycad/octorust.rs".to_string()),
    };

    // Load our spec.
    let spec = crate::load_json_spec(include_str!("../tests/api.github.com.json")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/github.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(&test_file_path, test_file).unwrap();

    // Generate the library.
    crate::generate(&spec, &opts).await.unwrap();

    // Run tests.
    run_cargo_test(&opts).await.unwrap();
}

async fn run_cargo_test(opts: &crate::Opts) -> Result<()> {
    log::info!("Running `cargo test`...");

    // Shell out and run cargo clippy on the output directory.
    let output = if opts.output.display().to_string() == "." {
        "".to_string()
    } else {
        opts.output.display().to_string()
    };

    let mut cmd = tokio::process::Command::new("cargo");
    cmd.args(["test"])
        .current_dir(output)
        // So that we can run fresh and not fail.
        .env("EXPECTORATE", "overwrite");

    let output = cmd.output().await?;
    if !output.status.success() {
        anyhow::bail!(
            "cargo test failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
