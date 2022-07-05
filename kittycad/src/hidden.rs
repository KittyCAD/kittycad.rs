use anyhow::Result;

use crate::Client;

pub struct Hidden {
    pub client: Client,
}

impl Hidden {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Hidden { client }
    }

    /**
    * Create an email verification request for a user.
    *
    * This function performs a `POST` to the `/auth/email` endpoint.
    */
    pub async fn listen_auth_email(
        &self,
        body: &crate::types::EmailAuthenticationForm,
    ) -> Result<()> {
        let url = "/auth/email".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Listen for callbacks for email verification for users.
    *
    * This function performs a `GET` to the `/auth/email/callback` endpoint.
    *
    * **Parameters:**
    *
    * * `callback_url: &url::Url` -- The URL to redirect back to after we have authenticated.
    * * `email: &str` -- The user's email.
    * * `token: &str` -- The verification token.
    */
    pub async fn listen_auth_email_callback(
        &self,
        callback_url: &url::Url,
        email: &str,
        token: &str,
    ) -> Result<String> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !callback_url.to_string().is_empty() {
            query_args.push(("callback_url".to_string(), callback_url.to_string()));
        }
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !token.is_empty() {
            query_args.push(("token".to_string(), token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/auth/email/callback?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This endpoint removes the session cookie for a user.
    *
    * This function performs a `POST` to the `/logout` endpoint.
    *
    * This is used in logout scenarios.
    */
    pub async fn logout(&self) -> Result<()> {
        let url = "/logout".to_string();
        self.client.post(&url, None).await
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
    ) -> Result<String> {
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
