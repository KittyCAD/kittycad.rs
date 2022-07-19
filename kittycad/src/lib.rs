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
//! kittycad = "0.1.9"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
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
//! ```
//! use kittycad::Client;
//!
//! let client = Client::new_from_env();
//! ```
#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(hidden)]
/// API calls that have been performed by users can be queried by the API. This is helpful for debugging as well as billing.
///
/// FROM: <https://docs.kittycad.io/api/api-calls>
pub mod api_calls;
/// API tokens allow users to call the API outside of their session token that is used as a cookie in the user interface. Users can create, delete, and list their API tokens. But, of course, you need an API token to do this, so first be sure to generate one in the account UI.
///
/// FROM: <https://docs.kittycad.io/api/api-tokens>
pub mod api_tokens;
/// CAD file operations. Create, get, and list CAD file conversions. More endpoints will be added here in the future as we build out transforms, etc on CAD models.
///
/// FROM: <https://docs.kittycad.io/api/file>
pub mod file;
/// Hidden API endpoints that should not show up in the docs.
///
/// FROM: <https://docs.kittycad.io/api/hidden>
pub mod hidden;
/// Meta information about the API.
///
/// FROM: <https://docs.kittycad.io/api/meta>
pub mod meta;
/// Endpoints that implement OAuth 2.0 grant flows.
///
/// FROM: <https://docs.kittycad.io/api/oauth2>
pub mod oauth2;
/// Operations around payments and billing.
///
/// FROM: <https://docs.kittycad.io/api/payments>
pub mod payments;
/// Sessions allow users to call the API from their session cookie in the browser.
///
/// FROM: <https://docs.kittycad.io/api/sessions>
pub mod sessions;
#[cfg(test)]
mod tests;
pub mod types;
/// Unit conversion operations.
///
/// FROM: <https://docs.kittycad.io/api/file>
pub mod unit;
/// A user is someone who uses the KittyCAD API. Here, we can create, delete, and list users. We can also get information about a user. Operations will only be authorized if the user is requesting information about themselves.
///
/// FROM: <https://docs.kittycad.io/api/users>
pub mod users;

use std::env;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

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
    #[tracing::instrument]
    pub fn new<T>(token: T) -> Self
    where
        T: ToString,
    {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build();

        match client {
            Ok(c) => Client {
                token: token.to_string(),
                base_url: "https://api.kittycad.io".to_string(),

                client: c,
            },
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the base URL for the client to something other than the default: <https://api.kittycad.io>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display,
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
    ) -> anyhow::Result<reqwest::RequestBuilder> {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(method, &u);

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

        Ok(req)
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

    /// Sessions allow users to call the API from their session cookie in the browser.
    ///
    /// FROM: <https://docs.kittycad.io/api/sessions>
    pub fn sessions(&self) -> sessions::Sessions {
        sessions::Sessions::new(self.clone())
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
