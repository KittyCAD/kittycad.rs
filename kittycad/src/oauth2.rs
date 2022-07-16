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

    #[doc = "Start an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be accessed from an *unauthenticated* API client. It generates and records a `device_code` and `user_code` which must be verified and confirmed prior to a token being granted."]
    pub async fn device_auth_request(
        &self,
        body: &crate::types::DeviceAuthRequestForm,
    ) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/auth"),
        );
        rb = rb.bearer_auth(self.client.token);
        rb = rb.form(body);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Confirm an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be accessed by the user agent (browser), not the client requesting the token. So we do not actually return the token here; it will be returned in response to the poll on `/oauth2/device/token`."]
    pub async fn device_auth_confirm(
        &self,
        body: &crate::types::DeviceAuthVerifyParams,
    ) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/confirm"),
        );
        rb = rb.bearer_auth(self.client.token);
        rb = rb.json(body);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Request a device access token.\n\nThis endpoint should be polled by the client until the user code is verified and the grant is confirmed."]
    pub async fn device_access_token(
        &self,
        body: &crate::types::DeviceAccessTokenRequestForm,
    ) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/token"),
        );
        rb = rb.bearer_auth(self.client.token);
        rb = rb.form(body);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Verify an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint should be accessed in a full user agent (e.g., a browser). If the user is not logged in, we redirect them to the login page and use the `callback_url` parameter to get them to the UI verification form upon logging in. If they are logged in, we redirect them to the UI verification form on the website."]
    pub async fn device_auth_verify(&self, user_code: String) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "oauth2/device/verify"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Listen for callbacks for the OAuth 2.0 provider."]
    pub async fn listen_provider_callback(
        &self,
        provider: crate::types::AccountProvider,
        code: Option<String>,
        state: Option<String>,
    ) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "oauth2/provider/{provider}/callback"
            ),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Get the consent URL and other information for the OAuth 2.0 provider."]
    pub async fn listen_provider_consent(
        &self,
        provider: crate::types::AccountProvider,
        callback_url: Option<String>,
    ) -> Result<crate::types::Oauth2ClientInfo> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "oauth2/provider/{provider}/consent"
            ),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }
}
