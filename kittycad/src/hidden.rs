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

    #[doc = "Create an email verification request for a user."]
    pub async fn listen_auth_email(
        &self,
        body: &crate::types::EmailAuthenticationForm,
    ) -> Result<crate::types::VerificationToken> {
        let mut rb = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "auth/email"),
        );
        rb = rb.bearer_auth(self.client.token);
        rb = rb.json(body);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Listen for callbacks for email verification for users."]
    pub async fn listen_auth_email_callback(
        &self,
        callback_url: Option<url::Url>,
        email: String,
        token: String,
    ) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "auth/email/callback"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "This endpoint removes the session cookie for a user.\n\nThis is used in logout scenarios."]
    pub async fn logout(&self) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "logout"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }
}
