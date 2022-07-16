/*
 * Declare the client object:
 */

pub fn generate_client() -> String {
    CLIENT_FUNCTIONS.to_string()
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
                    base_url: "https://api.kittycad.io".to_string(),

                    client: c,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the base URL for the client to something other than the default: `https://api.kittycad.io`.
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: KITTYCAD_API_TOKEN.
    pub fn new_from_env() -> Self
    {
        let token = env::var("KITTYCAD_API_TOKEN").expect("must set KITTYCAD_API_TOKEN");

        Client::new(
            token,
        )
    }
"#;
