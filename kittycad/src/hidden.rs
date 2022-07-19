use crate::Client;
use anyhow::Result;
pub struct Hidden {
    pub client: Client,
}

impl Hidden {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Create an email verification request for a user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_hidden_listen_auth_email() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result : crate :: types :: VerificationToken = client . hidden () . listen_auth_email (& crate :: types :: EmailAuthenticationForm { callback_url : Some (url :: Url :: from_str (\"https://78.28.29.80.111/9\") ?) , email : \"173.119.184.156.81.111.4.43.84.128.45.48@225.205.129.81.145.237.108.157.229.108.167.236.89.130\" }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    pub async fn listen_auth_email<'a>(
        &'a self,
        body: &crate::types::EmailAuthenticationForm,
    ) -> Result<crate::types::VerificationToken, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "auth/email"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Listen for callbacks for email verification for users.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_hidden_listen_auth_email_callback() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .hidden()\n        .listen_auth_email_callback(\n            Some(url::Url::from_str(\"https://234.0.0.145/0\")?),\n            \"186.4.153.39.62.105.198.92@36.204.42.155.239.43.40.108.242.131.67\",\n            \"ji\",\n        )\n        .await?;\n    Ok(())\n}\n```"]
    pub async fn listen_auth_email_callback<'a>(
        &'a self,
        callback_url: Option<url::Url>,
        email: &'a str,
        token: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "auth/email/callback"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = callback_url {
            query_params.push(("callback_url", format!("{}", p)));
        }

        query_params.push(("email", email.to_string()));
        query_params.push(("token", token.to_string()));
        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "This endpoint removes the session cookie for a user.\n\nThis is used in logout scenarios.\n\n```rust,no_run\nasync fn example_hidden_logout() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client.hidden().logout().await?;\n    Ok(())\n}\n```"]
    pub async fn logout<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "logout"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
