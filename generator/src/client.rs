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
    host: String,

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
                    host: "https://api.kittycad.io".to_string(),

                    client: c,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the host for the client to something other than the default: `https://api.kittycad.io`.
    pub fn set_host<H>(&mut self, host: H)
    where
        H: Into<String> + std::fmt::Display,
    {
        self.host = host.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: KITTYCAD_API_TOKEN.
    pub fn new_from_env() -> Self
    {
        let token = env::var("KITTYCAD_API_TOKEN").expect("must set KITTYCAD_API_TOKEN");

        Client::new(
            token,
        )
    }

    async fn url_and_auth(
        &self,
        uri: &str,
    ) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();

        let auth = format!("Bearer {}", self.token);
        parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
    }

    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::RequestBuilder>
    {
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
            log::debug!("body: {:?}", String::from_utf8(body.as_bytes().unwrap().to_vec()).unwrap());
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
    ) -> Result<reqwest::Response>
    {
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
            log::debug!("response payload {}", String::from_utf8_lossy(&response_body));
            let parsed_response = if status == http::StatusCode::NO_CONTENT || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>(){
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
                       let e : crate::types::Error = resp.into();
                       e.into()
                    },
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
        let r = self
            .request(method, uri, body)
            .await?;
        Ok(r)
    }

    async fn get<D>(&self, uri: &str,  message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::GET,
            &(self.host.to_string() + uri),
            message,
        ).await
    }

    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::POST,
            &(self.host.to_string() + uri),
            message,
        ).await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PATCH,
            &(self.host.to_string() + uri),
            message,
        ).await
    }

    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::PUT,
            &(self.host.to_string() + uri),
            message,
        ).await
    }

    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.to_string() + uri),
            message,
        ).await
}"#;
