use anyhow::Result;

use crate::Client;

pub struct Oauth2 {
    pub client: Client,
}

impl Oauth2 {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Oauth2 { client }
    }

    /**
    * Start an OAuth 2.0 Device Authorization Grant.
    *
    * This function performs a `POST` to the `/oauth2/device/auth` endpoint.
    *
    * This endpoint is designed to be accessed from an *unauthenticated* API client. It generates and records a `device_code` and `user_code` which must be verified and confirmed prior to a token being granted.
    */
    pub async fn device_auth_request(&self) -> Result<()> {
        let url = "/oauth2/device/auth".to_string();
        self.client.post(&url, None).await
    }

    /**
    * Confirm an OAuth 2.0 Device Authorization Grant.
    *
    * This function performs a `POST` to the `/oauth2/device/confirm` endpoint.
    *
    * This endpoint is designed to be accessed by the user agent (browser), not the client requesting the token. So we do not actually return the token here; it will be returned in response to the poll on `/device/token`.
    */
    pub async fn device_auth_confirm(
        &self,
        body: &crate::types::DeviceAuthVerifyParams,
    ) -> Result<String> {
        let url = "/oauth2/device/confirm".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Request a device access token.
    *
    * This function performs a `POST` to the `/oauth2/device/token` endpoint.
    *
    * This endpoint should be polled by the client until the user code is verified and the grant is confirmed.
    */
    pub async fn device_access_token(&self) -> Result<()> {
        let url = "/oauth2/device/token".to_string();
        self.client.post(&url, None).await
    }

    /**
    * Verify an OAuth 2.0 Device Authorization Grant.
    *
    * This function performs a `GET` to the `/oauth2/device/verify` endpoint.
    *
    * This endpoint should be accessed in a full user agent (e.g., a browser). If the user is not logged in, we redirect them to the login page and use the `state` parameter to get them back here on completion. If they are logged in, serve up the console verification page so they can verify the user code.
    *
    * **Parameters:**
    *
    * * `user_code: &str` -- The user code.
    */
    pub async fn device_auth_verify(&self, user_code: &str) -> Result<String> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !user_code.is_empty() {
            query_args.push(("user_code".to_string(), user_code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/oauth2/device/verify?{}", query_);

        self.client.get(&url, None).await
    }
}
