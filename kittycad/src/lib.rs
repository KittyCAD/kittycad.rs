//! A fully generated, opinionated API client library for KittyCAD.
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
//! This client is generated from the [KittyCAD OpenAPI
//! specs](https://github.com/) based on API spec version `0.1.0`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! kittycad = "0.1.1"
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
//! let kittycad = Client::new(
//!     String::from("api-key"),
//! );
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
//! let kittycad = Client::new_from_env();
//! ```
//!
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

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
pub mod traits;
pub mod types;
/// Unit conversion operations.
///
/// FROM: <https://docs.kittycad.io/api/file>
pub mod unit;
/// A user is someone who uses the KittyCAD API. Here, we can create, delete, and list users. We can also get information about a user. Operations will only be authorized if the user is requesting information about themselves.
///
/// FROM: <https://docs.kittycad.io/api/users>
pub mod users;
#[doc(hidden)]
pub mod utils;

use anyhow::{anyhow, Error, Result};

mod progenitor_support {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    const PATH_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}');

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

use std::env;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    token: String,
    host: String,

    client: reqwest::Client,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
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
                host: "https://api.kittycad.io".to_string(),

                client: c,
            },
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the host for the client to something other than the default: `https://api.kittycad.io`.
    pub fn set_host<H>(&mut self, host: H)
    where
        H: Into<String> + std::fmt::Display,
    {
        self.host = host.to_string();
    }

    /// Create a new Client struct from the environment variable: KITTYCAD_API_TOKEN.
    pub fn new_from_env() -> Self {
        let token = env::var("KITTYCAD_API_TOKEN").expect("must set KITTYCAD_API_TOKEN");

        Client::new(token)
    }

    async fn url_and_auth(&self, uri: &str) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        let auth = format!("Bearer {}", self.token);
        parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
    }

    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::RequestBuilder> {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method.clone(), url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if let Some(body) = body {
            log::debug!(
                "body: {:?}",
                String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap()
            );
            req = req.body(body);
        }
        log::debug!("request: {:?}", &req);
        Ok(req)
    }

    pub async fn response_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::Response> {
        let req = self.request_raw(method, uri, body).await?;
        Ok(req.send().await?)
    }

    async fn request<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.response_raw(method, uri, body).await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!(
                "response payload {}",
                String::from_utf8_lossy(&response_body)
            );
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error: anyhow::Error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                // Parse the error as the error type.
                match serde_json::from_slice::<crate::types::ErrorResponse>(&response_body) {
                    Ok(resp) => {
                        let e: crate::types::Error = resp.into();
                        e.into()
                    }
                    Err(_) => {
                        anyhow!(
                            "code: {}, error: {:?}",
                            status,
                            String::from_utf8_lossy(&response_body),
                        )
                    }
                }
            };

            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, body).await?;
        Ok(r)
    }

    async fn get<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::GET, &(self.host.to_string() + uri), message)
            .await
    }

    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, &(self.host.to_string() + uri), message)
            .await
    }

    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, &(self.host.to_string() + uri), message)
            .await
    }

    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.to_string() + uri),
            message,
        )
        .await
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
