//! Client templates for our generated library.

/// Generate the base of the API client.
pub fn generate_client(opts: &crate::Opts) -> String {
    if let Some(token_endpoint) = &opts.token_endpoint {
        // Ensure we also have a user consent endpoint.
        if opts.user_consent_endpoint.is_none() {
            panic!("user_consent_endpoint is required if token_endpoint is provided");
        }

        return CLIENT_FUNCTIONS_OAUTH_TOKEN
            .replace(
                "ENV_VARIABLE_CODE",
                &get_env_variable_code_oauth_token(opts),
            )
            .replace("TOKEN_ENDPOINT", token_endpoint.as_ref())
            .replace(
                "USER_CONSENT_ENDPOINT",
                opts.user_consent_endpoint.as_ref().unwrap().as_ref(),
            )
            .replace(
                "TIMEOUT_NUM_SECONDS",
                &opts.request_timeout_seconds.to_string(),
            )
            .replace("BASE_URL", opts.base_url.to_string().trim_end_matches('/'));
    }

    if opts.basic_auth {
        return CLIENT_FUNCTIONS_BASIC_AUTH
            .replace(
                "TIMEOUT_NUM_SECONDS",
                &opts.request_timeout_seconds.to_string(),
            )
            .replace("ENV_VARIABLE_CODE", &get_env_variable_code_basic_auth(opts))
            .replace("BASE_URL", opts.base_url.to_string().trim_end_matches('/'));
    }

    CLIENT_FUNCTIONS_TOKEN
        .replace(
            "TIMEOUT_NUM_SECONDS",
            &opts.request_timeout_seconds.to_string(),
        )
        .replace("ENV_VARIABLE_CODE", &get_env_variable_code_token(opts))
        .replace("BASE_URL", opts.base_url.to_string().trim_end_matches('/'))
}

fn get_env_variable_code_basic_auth(opts: &crate::Opts) -> String {
    let start = if let Some(add_env_prefix) = &opts.add_env_prefix {
        r#"let username = if let Ok(username) = env::var("ENV_VARIABLE_PREFIX_USERNAME") {
        username
    } else if let Ok(username) = env::var("ADD_ENV_VARIABLE_PREFIX_USERNAME") {
        username
    } else {
        panic!("must set ENV_VARIABLE_PREFIX_USERNAME or ADD_ENV_VARIABLE_PREFIX_USERNAME");
    };
    let password = if let Ok(password) = env::var("ENV_VARIABLE_PREFIX_PASSWORD") {
        password
    } else if let Ok(password) = env::var("ADD_ENV_VARIABLE_PREFIX_PASSWORD") {
        password
    } else {
        panic!("must set ENV_VARIABLE_PREFIX_PASSWORD or ADD_ENV_VARIABLE_PREFIX_PASSWORD");
    };
    let base_url = if let Ok(base_url) = env::var("ENV_VARIABLE_PREFIX_HOST") {
        base_url
    } else if let Ok(base_url) = env::var("ADD_ENV_VARIABLE_PREFIX_HOST") {
        base_url
    } else {            
        "BASE_URL".to_string()
    };"#
        .replace(
            "ADD_ENV_VARIABLE_PREFIX",
            &crate::template::get_env_variable_prefix(add_env_prefix),
        )
        .replace(
            "ENV_VARIABLE_PREFIX",
            &crate::template::get_env_variable_prefix(&opts.name),
        )
    } else {
        r#"let username = env::var("ENV_VARIABLE_PREFIX_USERNAME").expect("must set ENV_VARIABLE_PREFIX_USERNAME");
        let password = env::var("ENV_VARIABLE_PREFIX_PASSWORD").expect("must set ENV_VARIABLE_PREFIX_PASSWORD");
        let base_url = env::var("ENV_VARIABLE_PREFIX_HOST").unwrap_or("BASE_URL".to_string());"#.replace("ENV_VARIABLE_PREFIX", &crate::template::get_env_variable_prefix(&opts.name))
    };

    format!(
        r#"{}


    let mut c = Client::new(
        username,
        password,
    );
    c.set_base_url(base_url);
    c
    "#,
        start
    )
}

const CLIENT_FUNCTIONS_BASIC_AUTH: &str = r#"
#[cfg(feature = "requests")]
use std::env;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "requests")]
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    ".rs/",
    env!("CARGO_PKG_VERSION"),
);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
#[cfg(feature = "requests")]
pub struct Client {
    username: String,
    password: String,
    base_url: String,

    #[cfg(feature = "retry")]
    client: reqwest_middleware::ClientWithMiddleware,
    #[cfg(not(feature = "retry"))]
    client: reqwest::Client,
}

#[cfg(feature = "requests")]
impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument(skip_all)]
    pub fn new<T>(
        username: T,
        password: T,
    ) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .timeout(std::time::Duration::from_secs(TIMEOUT_NUM_SECONDS))
            .connect_timeout(std::time::Duration::from_secs(60))
            .build();
        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match client {
                Ok(c) => {
                    let client = reqwest_middleware::ClientBuilder::new(c)
                        // Trace HTTP requests. See the tracing crate to make use of these traces.
                        .with(reqwest_tracing::TracingMiddleware::default())
                        // Retry failed requests.
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();

                    Client {
                        username: username.to_string(),
                        password: password.to_string(),
                        base_url: "BASE_URL".to_string(),

                        client,
                    }
                }
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            Client {
                username: username.to_string(),
                password: password.to_string(),
                base_url: "BASE_URL".to_string(),

                client,
            }
        }
    }

    /// Set the base URL for the client to something other than the default: <BASE_URL>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `ENV_VARIABLE_PREFIX_USERNAME`
    /// and `ENV_VARIABLE_PREFIX_PASSWORD`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self
    {
        ENV_VARIABLE_CODE
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<reqwest_middleware::RequestBuilder>
    {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(
            method,
            &u,
        );

        // Add in our authentication.
        req = req.basic_auth(&self.username, Some(&self.password));

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(body) = body {
            req = req.body(body);
        }

        Ok(req)
    }
"#;

fn get_env_variable_code_token(opts: &crate::Opts) -> String {
    let start = if let Some(add_env_prefix) = &opts.add_env_prefix {
        r#"let token = if let Ok(token) = env::var("ENV_VARIABLE_PREFIX_API_TOKEN") {
        token
    } else if let Ok(token) = env::var("ADD_ENV_VARIABLE_PREFIX_API_TOKEN") {
        token
    } else {
        panic!("must set ENV_VARIABLE_PREFIX_API_TOKEN or ADD_ENV_VARIABLE_PREFIX_API_TOKEN");
    };
    let base_url = if let Ok(base_url) = env::var("ENV_VARIABLE_PREFIX_HOST") {
        base_url
    } else if let Ok(base_url) = env::var("ADD_ENV_VARIABLE_PREFIX_HOST") {
        base_url
    } else {
        "BASE_URL".to_string()
    };"#
        .replace(
            "ADD_ENV_VARIABLE_PREFIX",
            &crate::template::get_env_variable_prefix(add_env_prefix),
        )
        .replace(
            "ENV_VARIABLE_PREFIX",
            &crate::template::get_env_variable_prefix(&opts.name),
        )
    } else {
        r#"let token = env::var("ENV_VARIABLE_PREFIX_API_TOKEN").expect("must set ENV_VARIABLE_PREFIX_API_TOKEN");
        let base_url = env::var("ENV_VARIABLE_PREFIX_HOST").unwrap_or("BASE_URL".to_string());
        "#.replace("ENV_VARIABLE_PREFIX", &crate::template::get_env_variable_prefix(&opts.name))
    };

    format!(
        r#"{}


    let mut c = Client::new(
        token,
    );
    c.set_base_url(base_url);
    c
    "#,
        start
    )
}

const CLIENT_FUNCTIONS_TOKEN: &str = r#"
#[cfg(feature = "requests")]
use std::env;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "requests")]
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    ".rs/",
    env!("CARGO_PKG_VERSION"),
);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
#[cfg(feature = "requests")]
pub struct Client {
    token: String,
    base_url: String,

    #[cfg(feature = "retry")]
    client: reqwest_middleware::ClientWithMiddleware,
    #[cfg(feature = "retry")]
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    client_http1_only: reqwest_middleware::ClientWithMiddleware,

    #[cfg(not(feature = "retry"))]
    client: reqwest::Client,
    #[cfg(not(feature = "retry"))]
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    client_http1_only: reqwest::Client,
}

/// A request builder.
#[cfg(feature = "retry")]
#[cfg(feature = "requests")]
pub struct RequestBuilder(pub reqwest_middleware::RequestBuilder);
#[cfg(not(feature = "retry"))]
#[cfg(feature = "requests")]
pub struct RequestBuilder(pub reqwest::RequestBuilder);

#[cfg(feature = "requests")]
impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    /// Also takes reqwest client builders, for customizing the client's behaviour.
    #[tracing::instrument(skip(token))]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_from_reqwest<T>(
        token: T,
        builder_http: reqwest::ClientBuilder,
        builder_websocket: reqwest::ClientBuilder,
    ) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match (builder_http.build(), builder_websocket.build()) {
                (Ok(c), Ok(c1)) => {
                    let client = reqwest_middleware::ClientBuilder::new(c)
                        // Trace HTTP requests. See the tracing crate to make use of these traces.
                        .with(reqwest_tracing::TracingMiddleware::default())
                        // Retry failed requests.
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    let client_http1_only = reqwest_middleware::ClientBuilder::new(c1)
                        .with(reqwest_tracing::TracingMiddleware::default())
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    Client {
                        token: token.to_string(),
                        base_url: "BASE_URL".to_string(),

                        client,
                        client_http1_only,
                    }
                }
                (Err(e), _) | (_, Err(e)) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            match (builder_http.build(), builder_websocket.build()) {
                (Ok(c), Ok(c1)) => Client {
                    token: token.to_string(),
                    base_url: "BASE_URL".to_string(),

                    client: c,
                    client_http1_only: c1,
                },
                (Err(e), _) | (_, Err(e)) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
    }

    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    /// Also takes reqwest client builders, for customizing the client's behaviour.
    #[tracing::instrument(skip(token))]
    #[cfg(target_arch = "wasm32")]
    pub fn new_from_reqwest<T>(
        token: T,
        builder_http: reqwest::ClientBuilder,
    ) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match builder_http.build() {
                Ok(c) => {
                    let client = reqwest_middleware::ClientBuilder::new(c)
                        // Trace HTTP requests. See the tracing crate to make use of these traces.
                        .with(reqwest_tracing::TracingMiddleware::default())
                        // Retry failed requests.
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    Client {
                        token: token.to_string(),
                        base_url: "BASE_URL".to_string(),

                        client,
                    }
                }
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            match builder_http.build() {
                Ok(c) => Client {
                    token: token.to_string(),
                    base_url: "BASE_URL".to_string(),

                    client: c,
                },
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
    }

    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument(skip(token))]
    pub fn new<T>(token: T) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(not(target_arch = "wasm32"))]
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            // For file conversions we need this to be long.
            .timeout(std::time::Duration::from_secs(600))
            .connect_timeout(std::time::Duration::from_secs(60));
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        let client_http1 = reqwest::Client::builder()
            // For file conversions we need this to be long.
            .user_agent(APP_USER_AGENT)
            .timeout(std::time::Duration::from_secs(600))
            .connect_timeout(std::time::Duration::from_secs(60))
            .http1_only();
        #[cfg(not(target_arch = "wasm32"))]
        return Self::new_from_reqwest(token, client, client_http1);
        #[cfg(target_arch = "wasm32")]
        Self::new_from_reqwest(token, client)
    }

    /// Set the base URL for the client to something other than the default: <BASE_URL>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `ENV_VARIABLE_PREFIX_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self
    {
       ENV_VARIABLE_CODE 
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<RequestBuilder>
    {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(
            method,
            &u,
        );

        // Add in our authentication.
        req = req.bearer_auth(&self.token);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(body) = body {
            req = req.body(body);
        }

        Ok(RequestBuilder(req))
    }
"#;

fn get_env_variable_code_oauth_token(opts: &crate::Opts) -> String {
    let start = if let Some(add_env_prefix) = &opts.add_env_prefix {
        r#"let client_id = if let Ok(client_id) = env::var("ENV_VARIABLE_PREFIX_CLIENT_ID") {
            client_id
        } else {
            if let Ok(client_id) = env::var("ADD_ENV_VARIABLE_PREFIX_CLIENT_ID") {
                client_id
            } else {
                panic!("must set ENV_VARIABLE_PREFIX_CLIENT_ID or ADD_ENV_VARIABLE_PREFIX_CLIENT_ID");
            }
        };
    };
    let client_secret = if let Ok(client_secret) = env::var("ENV_VARIABLE_PREFIX_CLIENT_SECRET") {
        client_secret
    } else if let Ok(client_secret) = env::var("ADD_ENV_VARIABLE_PREFIX_CLIENT_SECRET") {
        client_secret
    } else {
        panic!("must set ENV_VARIABLE_PREFIX_CLIENT_SECRET or ADD_ENV_VARIABLE_PREFIX_CLIENT_SECRET");
    };
    let redirect_uri = if let Ok(redirect_uri) = env::var("ENV_VARIABLE_PREFIX_REDIRECT_URI") {
        redirect_uri
    } else if let Ok(redirect_uri) = env::var("ADD_ENV_VARIABLE_PREFIX_REDIRECT_URI") {
        redirect_uri
    } else {
        panic!("must set ENV_VARIABLE_PREFIX_REDIRECT_URI or ADD_ENV_VARIABLE_PREFIX_REDIRECT_URI");
    };
    let base_url = if let Ok(base_url) = env::var("ENV_VARIABLE_PREFIX_HOST"){
        base_url
    } else if let Ok(base_url) = env::var("ADD_ENV_VARIABLE_PREFIX_HOST") {
        base_url
    } else {
        "BASE_URL".to_string()
    };"#
        .replace(
            "ADD_ENV_VARIABLE_PREFIX",
            &crate::template::get_env_variable_prefix(add_env_prefix),
        )
        .replace(
            "ENV_VARIABLE_PREFIX",
            &crate::template::get_env_variable_prefix(&opts.name),
        )
    } else {
        r#"let client_id = env::var("ENV_VARIABLE_PREFIX_CLIENT_ID").expect("must set ENV_VARIABLE_PREFIX_CLIENT_ID");
        let client_secret = env::var("ENV_VARIABLE_PREFIX_CLIENT_SECRET").expect("must set ENV_VARIABLE_PREFIX_CLIENT_SECRET");
        let redirect_uri = env::var("ENV_VARIABLE_PREFIX_REDIRECT_URI").expect("must set ENV_VARIABLE_PREFIX_REDIRECT_URI");
        let base_url = env::var("ENV_VARIABLE_PREFIX_HOST").unwrap_or("BASE_URL".to_string());"#.replace("ENV_VARIABLE_PREFIX", &crate::template::get_env_variable_prefix(&opts.name))
    };

    format!(
        r#"{}

    
    let mut c = Client::new(client_id, client_secret, redirect_uri, token, refresh_token);
    c.set_base_url(base_url);
    c
    "#,
        start
    )
}

const CLIENT_FUNCTIONS_OAUTH_TOKEN: &str = r#"
use std::{env, sync::Arc, convert::TryInto, ops::Add, time::{Duration, Instant}};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "requests")]
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    ".rs/",
    env!("CARGO_PKG_VERSION"),
);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
#[cfg(feature = "requests")]
pub struct Client {
    base_url: String,
    token: Arc<tokio::sync::RwLock<InnerToken>>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,

    auto_refresh: bool,

    #[cfg(feature = "retry")]
    client: reqwest_middleware::ClientWithMiddleware,
    #[cfg(not(feature = "retry"))]
    client: reqwest::Client,
}

/// An access token.
#[derive(Debug, JsonSchema, Clone, Default, Serialize, Deserialize)]
#[cfg(feature = "requests")]
pub struct AccessToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
    )]
    pub token_type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
    )]
    pub access_token: String,
    #[serde(default)]
    pub expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
    )]
    pub refresh_token: String,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
    )]
    pub scope: String,
}

/// Time in seconds before the access token expiration point that a refresh should
/// be performed. This value is subtracted from the `expires_in` value returned by
/// the provider prior to storing
#[cfg(feature = "requests")]
const REFRESH_THRESHOLD: Duration = Duration::from_secs(60);

#[derive(Debug, Clone)]
#[cfg(feature = "requests")]
struct InnerToken {
    access_token: String,
    refresh_token: String,
    expires_at: Option<Instant>,
}

#[cfg(feature = "requests")]
impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API credentials your requests will work.
    #[tracing::instrument(skip_all)]
    pub fn new<I, K, R, T, Q>(
        client_id: I,
        client_secret: K,
        redirect_uri: R,
        token: T,
        refresh_token: Q,
    ) -> Self
    where
        I: ToString + std::fmt::Debug,
        K: ToString + std::fmt::Debug,
        R: ToString + std::fmt::Debug,
        T: ToString + std::fmt::Debug,
        Q: ToString + std::fmt::Debug,
    {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build();

        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match client {
                Ok(c) => {
                    let client = reqwest_middleware::ClientBuilder::new(c)
                        // Trace HTTP requests. See the tracing crate to make use of these traces.
                        .with(reqwest_tracing::TracingMiddleware::default())
                        // Retry failed requests.
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();

                    Client {
                        base_url: "BASE_URL".to_string(),
                        client_id: client_id.to_string(),
                        client_secret: client_secret.to_string(),
                        redirect_uri: redirect_uri.to_string(),
                        token: Arc::new(tokio::sync::RwLock::new(InnerToken {
                            access_token: token.to_string(),
                            refresh_token: refresh_token.to_string(),
                            expires_at: None,
                        })),

                        auto_refresh: false,
                        client,
                    }
                }
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            Client {
                base_url: "BASE_URL".to_string(),
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                redirect_uri: redirect_uri.to_string(),
                token: Arc::new(tokio::sync::RwLock::new(InnerToken {
                    access_token: token.to_string(),
                    refresh_token: refresh_token.to_string(),
                    expires_at: None,
                })),

                auto_refresh: false,
                client,
            }
        }
    }

    /// Set the base URL for the client to something other than the default: <BASE_URL>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Enables or disables the automatic refreshing of access tokens upon expiration
    #[tracing::instrument]
    pub fn set_auto_access_token_refresh(&mut self, enabled: bool) -> &mut Self {
        self.auto_refresh = enabled;
        self
    }

    /// Sets a specific `Instant` at which the access token should be considered expired.
    /// The expiration value will only be used when automatic access token refreshing is
    /// also enabled. `None` may be passed in if the expiration is unknown. In this case
    /// automatic refreshes will be attempted when encountering an UNAUTHENTICATED status
    /// code on a response.
    #[tracing::instrument]
    pub async fn set_expires_at(&self, expires_at: Option<Instant>) -> &Self {
        self.token.write().await.expires_at = expires_at;
        self
    }

    /// Gets the `Instant` at which the access token used by this client is set to expire
    /// if one is known
    #[tracing::instrument]
    pub async fn expires_at(&self) -> Option<Instant> {
        self.token.read().await.expires_at
    }

    /// Sets the number of seconds in which the current access token should be considered
    /// expired
    #[tracing::instrument]
    pub async fn set_expires_in(&self, expires_in: i64) -> &Self {
        self.token.write().await.expires_at = Self::compute_expires_at(expires_in);
        self
    }

    /// Gets the number of seconds from now in which the current access token will be
    /// considered expired if one is known
    #[tracing::instrument]
    pub async fn expires_in(&self) -> Option<Duration> {
        self.token
            .read()
            .await
            .expires_at
            .map(|i| i.duration_since(Instant::now()))
    }

    /// Determines if the access token currently stored in the client is expired. If the
    /// expiration can not be determined, None is returned
    #[tracing::instrument]
    pub async fn is_expired(&self) -> Option<bool> {
        self.token
            .read()
            .await
            .expires_at
            .map(|expiration| expiration <= Instant::now())
    }

    #[tracing::instrument]
    fn compute_expires_at(expires_in: i64) -> Option<Instant> {
        let seconds_valid = expires_in
            .try_into()
            .ok()
            .map(Duration::from_secs)
            .and_then(|dur| dur.checked_sub(REFRESH_THRESHOLD))
            .or_else(|| Some(Duration::from_secs(0)));

        seconds_valid.map(|seconds_valid| Instant::now().add(seconds_valid))
    }

    /// Create a new Client struct from the environment variables:
    ///     - `ENV_VARIABLE_PREFIX_CLIENT_ID`
    ///     - `ENV_VARIABLE_PREFIX_CLIENT_SECRET`
    ///     - `ENV_VARIABLE_PREFIX_REDIRECT_URI`
    #[tracing::instrument(skip_all)]
    pub fn new_from_env<T, R>(token: T, refresh_token: R) -> Self
    where
        T: ToString + std::fmt::Debug,
        R: ToString + std::fmt::Debug,
    {
        ENV_VARIABLE_CODE
    }

    /// Return a user consent url with an optional set of scopes.
    /// If no scopes are provided, they will not be passed in the url.
    pub fn user_consent_url(&self, scopes: &[String]) -> String {
        let state = uuid::Uuid::new_v4();

        let url = format!(
            "USER_CONSENT_ENDPOINT?client_id={}&response_type=code&redirect_uri={}&state={}",
             self.client_id, self.redirect_uri, state
        );

        if scopes.is_empty() {
            return url;
        }

        // Add the scopes.
        format!("{}&scope={}", url, scopes.join(" "))
    }

    /// Refresh an access token from a refresh token. Client must have a refresh token
    /// for this to work.
    pub async fn refresh_access_token(&self) -> anyhow::Result<AccessToken> {
        let response = {
            let refresh_token = &self.token.read().await.refresh_token;

            if refresh_token.is_empty() {
                anyhow::bail!("refresh token cannot be empty");
            }

            let mut headers = reqwest::header::HeaderMap::new();
            headers.append(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            );

            let params = [
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("redirect_uri", &self.redirect_uri),
            ];
            let client = reqwest::Client::new();
            client
                .post("TOKEN_ENDPOINT")
                .headers(headers)
                .form(&params)
                .basic_auth(&self.client_id, Some(&self.client_secret))
                .send()
                .await?
        };

        // Unwrap the response.
        let t: AccessToken = response.json().await?;

        let refresh_token = self.token.read().await.refresh_token.clone();

        *self.token.write().await = InnerToken {
            access_token: t.access_token.clone(),
            refresh_token,
            expires_at: Self::compute_expires_at(t.expires_in),
        };

        Ok(t)
    }

    /// Get an access token from the code returned by the URL paramter sent to the
    /// redirect URL.
    pub async fn get_access_token(&mut self, code: &str, state: &str) -> anyhow::Result<AccessToken> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
            ("state", state),
        ];
        let client = reqwest::Client::new();
        let resp = client
            .post("TOKEN_ENDPOINT")
            .headers(headers)
            .form(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        // Unwrap the response.
        let t: AccessToken = resp.json().await?;

        *self.token.write().await = InnerToken {
            access_token: t.access_token.clone(),
            refresh_token: t.refresh_token.clone(),
            expires_at: Self::compute_expires_at(t.expires_in),
        };

        Ok(t)
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<reqwest_middleware::RequestBuilder>
    {
        if self.auto_refresh {
            let expired = self.is_expired().await;

            match expired {
                // We have a known expired token, we know we need to perform a refresh prior to
                // attempting to make a request
                Some(true) => {
                    self.refresh_access_token().await?;
                }

                // We have a (theoretically) known good token available. We make an optimistic
                // attempting at the request. If the token is no longer good, then something other
                // than the expiration is triggering the failure. We defer handling of these errors
                // to the caller
                Some(false) => (),

                // We do not know what state we are in. We could have a valid or expired token.
                // Generally this means we are in one of two cases:
                //   1. We have not yet performed a token refresh, nor has the user provided
                //      expiration data, and therefore do not know the expiration of the user
                //      provided token
                //   2. The provider is returning unusable expiration times, at which point we
                //      choose to ignore them
                None => (),
            }
        }

        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(
            method,
            &u,
        );

        // Add in our authentication.
        req = req.bearer_auth(&self.token.read().await.access_token);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(body) = body {
            req = req.body(body);
        }

        Ok(req)
    }
"#;
