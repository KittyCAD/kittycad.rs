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
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "auth/email"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

    #[doc = "Listen for callbacks for email verification for users."]
    pub async fn listen_auth_email_callback(
        &self,
        callback_url: Option<url::Url>,
        email: String,
        token: String,
    ) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "auth/email/callback"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.query(&[
            ("callback_url", callback_url),
            ("email", email),
            ("token", token),
        ]);
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

    #[doc = "This endpoint removes the session cookie for a user.\n\nThis is used in logout scenarios."]
    pub async fn logout(&self) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "logout"),
        );
        req = req.bearer_auth(&self.client.token);
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
}
