//! Client templates for our generated library.

/// Generate the base of the API client.
pub fn generate_client(name: &str, base_url: &url::Url) -> String {
    CLIENT_FUNCTIONS
        .replace(
            "ENV_VARIABLE",
            &crate::template::get_token_env_variable(name),
        )
        .replace("BASE_URL", base_url.to_string().trim_end_matches('/'))
}

const CLIENT_FUNCTIONS: &str = r#"
use std::env;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    ".rs/",
    env!("CARGO_PKG_VERSION"),
);

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    token: String,
    base_url: String,

    client: reqwest::Client,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<T>(
        token: T,
    ) -> Self
    where
        T: ToString,
    {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build();

        match client {
            Ok(c) => {
                Client {
                    token: token.to_string(),
                    base_url: "BASE_URL".to_string(),

                    client: c,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the base URL for the client to something other than the default: <BASE_URL>.
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `ENV_VARIABLE`.
    pub fn new_from_env() -> Self
    {
        let token = env::var("ENV_VARIABLE").expect("must set ENV_VARIABLE");

        Client::new(
            token,
        )
    }
"#;
