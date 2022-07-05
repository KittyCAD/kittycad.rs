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
    * This endpoint is designed to be accessed by the user agent (browser), not the client requesting the token. So we do not actually return the token here; it will be returned in response to the poll on `/oauth2/device/token`.
    */
    pub async fn device_auth_confirm(
        &self,
        body: &crate::types::DeviceAuthVerifyParams,
    ) -> Result<()> {
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
    * This endpoint should be accessed in a full user agent (e.g., a browser). If the user is not logged in, we redirect them to the login page and use the `callback_url` parameter to get them to the UI verification form upon logging in. If they are logged in, we redirect them to the UI verification form on the website.
    *
    * **Parameters:**
    *
    * * `user_code: &str` -- The user code.
    */
    pub async fn device_auth_verify(&self, user_code: &str) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !user_code.is_empty() {
            query_args.push(("user_code".to_string(), user_code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/oauth2/device/verify?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Listen for callbacks for the OAuth 2.0 provider.
    *
    * This function performs a `GET` to the `/oauth2/provider/{provider}/callback` endpoint.
    *
    * **Parameters:**
    *
    * * `provider: crate::types::AccountProvider` -- An account provider.
    * * `code: &str` -- The authorization code.
    * * `state: &str` -- The state that we had passed in through the user consent URL.
    */
    pub async fn listen_oauth2_provider_callback(
        &self,
        code: &str,
        provider: crate::types::AccountProvider,
        state: &str,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !code.is_empty() {
            query_args.push(("code".to_string(), code.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/oauth2/provider/{}/callback?{}",
            crate::progenitor_support::encode_path(&provider.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get the consent URL and other information for the OAuth 2.0 provider.
    *
    * This function performs a `GET` to the `/oauth2/provider/{provider}/consent` endpoint.
    *
    * **Parameters:**
    *
    * * `provider: crate::types::AccountProvider` -- An account provider.
    * * `callback_url: &str` -- The URL to redirect back to after we have authenticated.
    */
    pub async fn listen_oauth2_provider_consent(
        &self,
        callback_url: &str,
        provider: crate::types::AccountProvider,
    ) -> Result<crate::types::OAuth2ClientInfo> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !callback_url.is_empty() {
            query_args.push(("callback_url".to_string(), callback_url.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/oauth2/provider/{}/consent?{}",
            crate::progenitor_support::encode_path(&provider.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
