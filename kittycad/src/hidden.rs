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

    #[doc = "Create an email verification request for a user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_hidden_listen_auth_email() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::VerificationToken = client\n        .hidden()\n        .listen_auth_email(&kittycad::types::EmailAuthenticationForm {\n            callback_url: Some(url::Url::from_str(\"http://187.241.0.189/0\")?),\n            email: \"185.165.52.76.104.17.221.118.112.93.245.124.46.164@105.73.252.111.7.8.5.141\",\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Listen for callbacks for email verification for users.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_hidden_listen_auth_email_callback() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .hidden()\n        .listen_auth_email_callback(\n            Some(url::Url::from_str(\"http://43.96.0.109/6\")?),\n            \"175.130.95.25.106.242.8.7.186.126.134.107@229.7.190.50.107.137.171.9.46.140.1.123\",\n            \"l\",\n        )\n        .await?;\n    Ok(())\n}\n```"]
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
