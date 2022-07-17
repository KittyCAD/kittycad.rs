use crate::Client;
use anyhow::Result;
pub struct Oauth2 {
    pub client: Client,
}

impl Oauth2 {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Start an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be accessed from an *unauthenticated* API client. It generates and records a `device_code` and `user_code` which must be verified and confirmed prior to a token being granted."]
    pub async fn device_auth_request(
        &self,
        body: &crate::types::DeviceAuthRequestForm,
    ) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/auth"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.form(body);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Confirm an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be accessed by the user agent (browser), not the client requesting the token. So we do not actually return the token here; it will be returned in response to the poll on `/oauth2/device/token`."]
    pub async fn device_auth_confirm(
        &self,
        body: &crate::types::DeviceAuthVerifyParams,
    ) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/confirm"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Request a device access token.\n\nThis endpoint should be polled by the client until the user code is verified and the grant is confirmed."]
    pub async fn device_access_token(
        &self,
        body: &crate::types::DeviceAccessTokenRequestForm,
    ) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/token"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.form(body);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Verify an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint should be accessed in a full user agent (e.g., a browser). If the user is not logged in, we redirect them to the login page and use the `callback_url` parameter to get them to the UI verification form upon logging in. If they are logged in, we redirect them to the UI verification form on the website."]
    pub async fn device_auth_verify(&self, user_code: &str) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "oauth2/device/verify"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("user_code", user_code.to_string()));
        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Listen for callbacks for the OAuth 2.0 provider."]
    pub async fn listen_provider_callback(
        &self,
        code: Option<String>,
        provider: crate::types::AccountProvider,
        state: Option<String>,
    ) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "oauth2/provider/{provider}/callback"
                    .replace("{provider}", &format!("{}", provider))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = code {
            query_params.push(("code", p));
        }

        if let Some(p) = state {
            query_params.push(("state", p));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Get the consent URL and other information for the OAuth 2.0 provider."]
    pub async fn listen_provider_consent(
        &self,
        callback_url: Option<String>,
        provider: crate::types::AccountProvider,
    ) -> Result<crate::types::Oauth2ClientInfo> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "oauth2/provider/{provider}/consent"
                    .replace("{provider}", &format!("{}", provider))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = callback_url {
            query_params.push(("callback_url", p));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }
}
