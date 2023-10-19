//! A fully generated & opinionated API client for the KittyCAD API.
//!
//! [![docs.rs](https://docs.rs/kittycad/badge.svg)](https://docs.rs/kittycad)
//!
//! ## API Details
//!
//! API server for KittyCAD
//!
//!
//!
//! ### Contact
//!
//!
//! | url | email |
//! |----|----|
//! | <https://kittycad.io> | api@kittycad.io |
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [OpenAPI specs](https://api.kittycad.io) based on API spec version `0.1.0`. This way it will remain up to date as features are added.
//!
//! The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! kittycad = "0.2.35"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use kittycad::Client;
//!
//! let client = Client::new(String::from("api-key"));
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `KITTYCAD_API_TOKEN`
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use kittycad::Client;
//!
//! let client = Client::new_from_env();
//! ```
#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// AI uses machine learning to generate 3D meshes.
///
/// FROM: <https://docs.kittycad.io/api/ai>
#[cfg(feature = "requests")]
pub mod ai;
/// API calls that have been performed by users can be queried by the API. This is helpful for debugging as well as billing.
///
/// FROM: <https://docs.kittycad.io/api/api-calls>
#[cfg(feature = "requests")]
pub mod api_calls;
/// API tokens allow users to call the API outside of their session token that is used as a cookie in the user interface. Users can create, delete, and list their API tokens. But, of course, you need an API token to do this, so first be sure to generate one in the account UI.
///
/// FROM: <https://docs.kittycad.io/api/api-tokens>
#[cfg(feature = "requests")]
pub mod api_tokens;
/// Endpoints for third party app grant flows.
///
/// FROM: <https://docs.kittycad.io/api/apps>
#[cfg(feature = "requests")]
pub mod apps;
/// Endpoints that allow for code execution or creation of code execution environments.
///
/// FROM: <https://docs.kittycad.io/api/executor>
#[cfg(feature = "requests")]
pub mod executor;
/// CAD file operations. Create, get, and list CAD file conversions. More endpoints will be added here in the future as we build out transforms, etc on CAD models.
///
/// FROM: <https://docs.kittycad.io/api/file>
#[cfg(feature = "requests")]
pub mod file;
/// Hidden API endpoints that should not show up in the docs.
///
/// FROM: <https://docs.kittycad.io/api/hidden>
#[cfg(feature = "requests")]
pub mod hidden;
/// Meta information about the API.
///
/// FROM: <https://docs.kittycad.io/api/meta>
#[cfg(feature = "requests")]
pub mod meta;
/// Modeling API for updating your 3D files using the KittyCAD engine.
///
/// FROM: <https://docs.kittycad.io/api/modeling>
#[cfg(feature = "requests")]
pub mod modeling;
/// Endpoints that implement OAuth 2.0 grant flows.
///
/// FROM: <https://docs.kittycad.io/api/oauth2>
#[cfg(feature = "requests")]
pub mod oauth2;
/// Operations around payments and billing.
///
/// FROM: <https://docs.kittycad.io/api/payments>
#[cfg(feature = "requests")]
pub mod payments;
#[cfg(test)]
#[cfg(feature = "requests")]
mod tests;
pub mod types;
/// Unit conversion operations.
///
/// FROM: <https://docs.kittycad.io/api/file>
#[cfg(feature = "requests")]
pub mod unit;
/// A user is someone who uses the KittyCAD API. Here, we can create, delete, and list users. We can also get information about a user. Operations will only be authorized if the user is requesting information about themselves.
///
/// FROM: <https://docs.kittycad.io/api/users>
#[cfg(feature = "requests")]
pub mod users;

#[cfg(feature = "requests")]
use std::env;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "requests")]
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

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
    client_http1_only: reqwest_middleware::ClientWithMiddleware,

    #[cfg(not(feature = "retry"))]
    client: reqwest::Client,
    #[cfg(not(feature = "retry"))]
    #[cfg(not(target_arch = "wasm32"))]
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
    #[tracing::instrument]
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
                        base_url: "https://api.kittycad.io".to_string(),

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
                    base_url: "https://api.kittycad.io".to_string(),

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
    #[tracing::instrument]
    #[cfg(target_arch = "wasm32")]
    pub fn new_from_reqwest<T>(token: T, builder_http: reqwest::ClientBuilder) -> Self
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
                        base_url: "https://api.kittycad.io".to_string(),

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
                    base_url: "https://api.kittycad.io".to_string(),

                    client: c,
                },
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
    }

    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument]
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

    /// Set the base URL for the client to something other than the default: <https://api.kittycad.io>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `KITTYCAD_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self {
        let token = env::var("KITTYCAD_API_TOKEN").expect("must set KITTYCAD_API_TOKEN");

        Client::new(token)
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<RequestBuilder> {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(method, u);

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

    /// AI uses machine learning to generate 3D meshes.
    ///
    /// FROM: <https://docs.kittycad.io/api/ai>
    pub fn ai(&self) -> ai::Ai {
        ai::Ai::new(self.clone())
    }

    /// API calls that have been performed by users can be queried by the API. This is helpful for debugging as well as billing.
    ///
    /// FROM: <https://docs.kittycad.io/api/api-calls>
    pub fn api_calls(&self) -> api_calls::ApiCalls {
        api_calls::ApiCalls::new(self.clone())
    }

    /// API tokens allow users to call the API outside of their session token that is used as a cookie in the user interface. Users can create, delete, and list their API tokens. But, of course, you need an API token to do this, so first be sure to generate one in the account UI.
    ///
    /// FROM: <https://docs.kittycad.io/api/api-tokens>
    pub fn api_tokens(&self) -> api_tokens::ApiTokens {
        api_tokens::ApiTokens::new(self.clone())
    }

    /// Endpoints for third party app grant flows.
    ///
    /// FROM: <https://docs.kittycad.io/api/apps>
    pub fn apps(&self) -> apps::Apps {
        apps::Apps::new(self.clone())
    }

    /// Endpoints that allow for code execution or creation of code execution environments.
    ///
    /// FROM: <https://docs.kittycad.io/api/executor>
    pub fn executor(&self) -> executor::Executor {
        executor::Executor::new(self.clone())
    }

    /// CAD file operations. Create, get, and list CAD file conversions. More endpoints will be added here in the future as we build out transforms, etc on CAD models.
    ///
    /// FROM: <https://docs.kittycad.io/api/file>
    pub fn file(&self) -> file::File {
        file::File::new(self.clone())
    }

    /// Hidden API endpoints that should not show up in the docs.
    ///
    /// FROM: <https://docs.kittycad.io/api/hidden>
    pub fn hidden(&self) -> hidden::Hidden {
        hidden::Hidden::new(self.clone())
    }

    /// Meta information about the API.
    ///
    /// FROM: <https://docs.kittycad.io/api/meta>
    pub fn meta(&self) -> meta::Meta {
        meta::Meta::new(self.clone())
    }

    /// Modeling API for updating your 3D files using the KittyCAD engine.
    ///
    /// FROM: <https://docs.kittycad.io/api/modeling>
    pub fn modeling(&self) -> modeling::Modeling {
        modeling::Modeling::new(self.clone())
    }

    /// Endpoints that implement OAuth 2.0 grant flows.
    ///
    /// FROM: <https://docs.kittycad.io/api/oauth2>
    pub fn oauth2(&self) -> oauth2::Oauth2 {
        oauth2::Oauth2::new(self.clone())
    }

    /// Operations around payments and billing.
    ///
    /// FROM: <https://docs.kittycad.io/api/payments>
    pub fn payments(&self) -> payments::Payments {
        payments::Payments::new(self.clone())
    }

    /// Unit conversion operations.
    ///
    /// FROM: <https://docs.kittycad.io/api/file>
    pub fn unit(&self) -> unit::Unit {
        unit::Unit::new(self.clone())
    }

    /// A user is someone who uses the KittyCAD API. Here, we can create, delete, and list users. We can also get information about a user. Operations will only be authorized if the user is requesting information about themselves.
    ///
    /// FROM: <https://docs.kittycad.io/api/users>
    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }
}
