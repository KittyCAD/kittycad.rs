use anyhow::Result;
use test_context::{test_context, TestContext as BlockingTestContext};

fn json_response_error(input: &str) -> crate::types::error::SerdeError {
    let error = serde_json::from_str::<Vec<String>>(input).unwrap_err();
    openapitor_runtime::SerdeError::new(input.to_string(), error)
}

fn plain_diagnostic(error: &impl std::fmt::Display) -> String {
    regex::Regex::new(r"\x1b\[[0-9;]*m")
        .unwrap()
        .replace_all(&error.to_string(), "")
        .into_owned()
}

#[test]
fn test_json_error_context_and_status() {
    use std::error::Error as _;

    let input = "[\n  \"first\",\n  \"second\",\n  42,\n  \"fourth\"\n]";
    let error = json_response_error(input);
    assert!(error.get_contextualize());
    assert_eq!(error.get_context_lines(), 3);
    assert_eq!(error.get_context_characters(), 30);
    // Keep the diagnostic text produced by the previous formatter.
    let expected = concat!(
        "\n   | [\n   |   \"first\",\n   |   \"second\",\n 4 |   42,\n",
        "   |     ^ invalid type: integer `42`, expected a string at line 4 column 4\n",
        "   |   \"fourth\"\n   | ]\n",
    );
    assert_eq!(plain_diagnostic(&error), expected);

    let error = crate::types::error::Error::from_serde_error(error, reqwest::StatusCode::OK);
    assert_eq!(error.status(), Some(reqwest::StatusCode::OK));
    assert_eq!(plain_diagnostic(&error), format!("Serde Error: {expected}"));
    assert!(error
        .source()
        .unwrap()
        .is::<openapitor_runtime::SerdeError>());
    assert!(error.source().unwrap().source().is_none());
}

#[test]
fn test_json_error_without_response_body() {
    let error = serde_json::from_str::<Vec<String>>("").unwrap_err();
    let error = crate::types::error::Error::from(error);
    assert_eq!(
        error.status(),
        Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
    );
    assert_eq!(
        plain_diagnostic(&error),
        "Serde Error: EOF while parsing a value at line 1 column 0\n",
    );
}

#[test]
fn test_load_yaml_spec_fixture() {
    let spec = crate::load_yaml_spec(include_str!("../tests/gusto.v1.yaml")).unwrap();
    let json = serde_json::to_string(&spec).unwrap();
    assert_eq!(
        serde_json::to_value(&spec).unwrap(),
        serde_json::to_value(crate::load_json_spec(&json).unwrap()).unwrap(),
    );
}

#[test]
fn test_load_yaml_spec_rejects_invalid_input() {
    for input in ["", "openapi: [", "openapi: 3.0.3\ninfo: {}\npaths: {}"] {
        let error = crate::load_yaml_spec(input).unwrap_err();
        assert!(error.downcast_ref::<yaml_serde::Error>().is_some());
    }
}

struct TestContext {
    tmp_dir: std::path::PathBuf,
}

impl TestContext {
    pub fn new() -> Result<Self> {
        // Create a temporary directory for the test.
        let tmp_dir = std::env::temp_dir();
        let tmp_dir = tmp_dir.join(format!("openapitor-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp_dir)?;
        let src_dir = tmp_dir.join("src");
        std::fs::create_dir_all(src_dir)?;

        Ok(TestContext { tmp_dir })
    }
}

impl BlockingTestContext for TestContext {
    fn setup() -> Self {
        TestContext::new().unwrap()
    }

    fn teardown(self) {
        println!("Removing {}", self.tmp_dir.display());
        // Delete the temporary directory.
        std::fs::remove_dir_all(&self.tmp_dir).unwrap();
    }
}

#[test_context(TestContext)]
#[test]
#[ignore]
fn test_kittycad_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.kittycad.io".parse().unwrap(),
        name: "kittycad".to_string(),
        target_version: "1.0.0".to_string(),
        description: "KittyCAD is a tool for generating 3D models of cats.".to_string(),
        spec_url: Some("https://api.kittycad.io".to_string()),
        repo_name: Some("kittycad/kittycad.rs".to_string()),
        ..Default::default()
    };

    // Load our spec.
    let spec = crate::load_json_spec(include_str!("../../spec.json")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/kittycad.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(test_file_path, test_file).unwrap();

    // Move the assets directory over so we have files to run.
    let assets_dir = ctx.tmp_dir.join("assets");
    std::fs::create_dir_all(&assets_dir).unwrap();
    // Get the current contents of the assets directory.
    let mut assets_dir_contents =
        std::fs::read_dir(std::env::current_dir().unwrap().join("../assets")).unwrap();
    // Move each file over.
    while let Some(Ok(entry)) = assets_dir_contents.next() {
        let dest = assets_dir.join(entry.file_name());
        std::fs::copy(entry.path(), &dest).unwrap();
    }

    // Generate the library.
    crate::generate(&spec, &opts).unwrap();

    // Make the output tests directory.
    let output_tests_dir = ctx.tmp_dir.join("tests");
    std::fs::create_dir_all(output_tests_dir).unwrap();

    // Run tests.
    run_cargo_test(&opts).unwrap();
}

#[test_context(TestContext)]
#[test]
#[ignore]
fn test_github_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.github.com".parse().unwrap(),
        name: "octorust".to_string(),
        target_version: "1.0.0".to_string(),
        description: "GitHub is where we push our code and you do too!".to_string(),
        spec_url: Some("https://github.com/github/rest-api-description/raw/main/descriptions/api.github.com/api.github.com.json".to_string()),
        repo_name: Some("kittycad/octorust.rs".to_string()),
        ..Default::default()
    };

    // Load our spec.
    let spec = crate::load_json_spec(include_str!("../tests/api.github.com.json")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/github.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(test_file_path, test_file).unwrap();

    // Generate the library.
    crate::generate(&spec, &opts).unwrap();

    // Run tests.
    run_cargo_test(&opts).unwrap();
}

#[test_context(TestContext)]
#[test]
fn test_oxide_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.oxide.computer".parse().unwrap(),
        name: "oxide".to_string(),
        target_version: "1.0.0".to_string(),
        description: "Oxide builds computers.".to_string(),
        spec_url: Some(
            "https://raw.githubusercontent.com/oxidecomputer/omicron/main/openapi/nexus.json"
                .to_string(),
        ),
        repo_name: Some("oxide/oxide.rs".to_string()),
        ..Default::default()
    };

    // Load our spec.
    let spec = crate::load_json_spec(include_str!("../tests/oxide.json")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/oxide.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(test_file_path, test_file).unwrap();

    // Generate the library.
    crate::generate(&spec, &opts).unwrap();

    // Run tests.
    run_cargo_test(&opts).unwrap();
}

#[test_context(TestContext)]
#[test]
fn test_remote_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.remote.com".parse().unwrap(),
        name: "remote-api".to_string(),
        target_version: "1.0.0".to_string(),
        description: "HR crap!".to_string(),
        spec_url: Some("".to_string()),
        repo_name: Some("kittycad/remote.rs".to_string()),
        ..Default::default()
    };

    // Load our spec.
    let spec = crate::load_json_spec(include_str!("../tests/remote.json")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/remote.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(test_file_path, test_file).unwrap();

    // Generate the library.
    crate::generate(&spec, &opts).unwrap();

    // Run tests.
    run_cargo_test(&opts).unwrap();
}

#[test_context(TestContext)]
#[test]
#[ignore]
fn test_gusto_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.gusto.com".parse().unwrap(),
        name: "gusto-api".to_string(),
        target_version: "1.0.0".to_string(),
        description: "HR crap!".to_string(),
        spec_url: Some("".to_string()),
        repo_name: Some("kittycad/gusto.rs".to_string()),
        token_endpoint: Some("https://api.gusto.com/oauth/token".parse().unwrap()),
        user_consent_endpoint: Some("https://api.gusto.com/oauth/authorize".parse().unwrap()),
        ..Default::default()
    };

    // Load our spec.
    let spec = crate::load_yaml_spec(include_str!("../tests/gusto.v1.yaml")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/gusto.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(test_file_path, test_file).unwrap();

    // Generate the library.
    crate::generate(&spec, &opts).unwrap();

    // Run tests.
    run_cargo_test(&opts).unwrap();
}

#[test_context(TestContext)]
#[test]
#[ignore]
fn test_ramp_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.ramp.com".parse().unwrap(),
        name: "ramp-api".to_string(),
        target_version: "1.0.0".to_string(),
        description: " crap!".to_string(),
        spec_url: Some("".to_string()),
        repo_name: Some("kittycad/ramp.rs".to_string()),
        token_endpoint: Some(
            "https://api.ramp.com/v1/public/customer/token"
                .parse()
                .unwrap(),
        ),
        user_consent_endpoint: Some("https://app.ramp.com/v1/authorize".parse().unwrap()),
        ..Default::default()
    };

    // Load our spec.
    let spec = crate::load_yaml_spec(include_str!("../tests/ramp.json")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/ramp.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(test_file_path, test_file).unwrap();

    // Generate the library.
    crate::generate(&spec, &opts).unwrap();

    // Run tests.
    run_cargo_test(&opts).unwrap();
}

#[test_context(TestContext)]
#[test]
fn test_commonroom_generation(ctx: &mut TestContext) {
    let opts = crate::Opts {
        debug: true,
        json: false,
        input: ctx.tmp_dir.clone(),
        output: ctx.tmp_dir.clone(),
        base_url: "https://api.commonroom.io/community/v1".parse().unwrap(),
        name: "commonroom-api".to_string(),
        target_version: "1.0.0".to_string(),
        description: " crap!".to_string(),
        spec_url: Some("".to_string()),
        repo_name: Some("kittycad/commonroom.rs".to_string()),
        token_endpoint: None,
        user_consent_endpoint: None,
        ..Default::default()
    };

    // Load our spec.
    let spec = crate::load_yaml_spec(include_str!("../tests/commonroom.json")).unwrap();

    // Move our test file to our output directory.
    let test_file = include_str!("../tests/library/commonroom.tests.rs");
    // Write our temporary file.
    let test_file_path = ctx.tmp_dir.join("src").join("tests.rs");
    std::fs::write(test_file_path, test_file).unwrap();

    // Generate the library.
    crate::generate(&spec, &opts).unwrap();

    // Run tests.
    run_cargo_test(&opts).unwrap();
}

#[test]
fn test_client_error_size() {
    // This template is embedded in every generated client. Keep returned errors
    // within Clippy's default large-error-threshold, including enum overhead.
    let size = std::mem::size_of::<crate::types::error::Error>();
    assert!(size < 128, "generated client error is {} bytes", size);
}

#[test]
fn test_invalid_response_payload_preserves_response_and_source() {
    use std::error::Error as _;

    let response = reqwest::Response::from(
        http::Response::builder()
            .status(reqwest::StatusCode::BAD_GATEWAY)
            .header("x-request-id", "request-123")
            .body("invalid payload")
            .unwrap(),
    );
    let source = response.error_for_status_ref().unwrap_err();
    let source_message = source.to_string();
    #[cfg(feature = "retry")]
    let source = reqwest_middleware::Error::from(source);
    let error = crate::types::error::Error::InvalidResponsePayload {
        error: source,
        response: Box::new(response),
    };

    assert_eq!(error.status(), Some(reqwest::StatusCode::BAD_GATEWAY));
    assert_eq!(error.source().unwrap().to_string(), source_message);
    assert_eq!(
        error.to_string(),
        format!("Invalid Response Payload: {source_message}")
    );
    let crate::types::error::Error::InvalidResponsePayload { response, .. } = error else {
        panic!("expected an invalid response payload");
    };
    assert_eq!(response.headers()["x-request-id"], "request-123");
    assert_eq!(response.content_length(), Some(15));
}

#[test]
fn test_unexpected_response_preserves_diagnostics() {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-request-id", "request-123".parse().unwrap());
    let error = crate::types::error::Error::UnexpectedResponse {
        status: reqwest::StatusCode::BAD_GATEWAY,
        url: "https://example.com/websocket".to_string(),
        body: "upstream unavailable".to_string(),
        headers: Box::new(headers.clone()),
    };

    assert_eq!(error.status(), Some(reqwest::StatusCode::BAD_GATEWAY));
    assert_eq!(
        error.to_string(),
        format!(
            "Unexpected Response for https://example.com/websocket (HTTP {}). Headers: \
             {headers:?}, body: upstream unavailable",
            reqwest::StatusCode::BAD_GATEWAY
        )
    );
    let crate::types::error::Error::UnexpectedResponse { headers, .. } = error else {
        panic!("expected an unexpected response");
    };
    assert_eq!(headers["x-request-id"], "request-123");
}

fn run_cargo_test(opts: &crate::Opts) -> Result<()> {
    log::info!("Running `cargo test`...");

    // Shell out and run cargo clippy on the output directory.
    let output = if opts.output.display().to_string() == "." {
        "".to_string()
    } else {
        opts.output.display().to_string()
    };

    let mut cmd = std::process::Command::new("cargo");
    // Temporary clients do not inherit the workspace's [patch.crates-io]. Use
    // this test-only override to test the local runtime, including before
    // publication.
    let runtime_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../openapitor-runtime")
        .canonicalize()?;
    let runtime_patch = format!(
        "patch.crates-io.openapitor-runtime.path={}",
        serde_json::to_string(&runtime_path)?
    );
    cmd.args(["test", "--quiet"])
        .args(["--config", &runtime_patch])
        .current_dir(output)
        // So that we can run fresh and not fail.
        .env("EXPECTORATE", "overwrite");

    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        eprintln!("Stderr:");
        eprintln!("{stderr}");
        eprintln!("Stdout:");
        eprintln!("{stdout}");
        anyhow::bail!("cargo test failed, see above");
    }

    Ok(())
}
