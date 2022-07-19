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

    #[doc = "Create an email verification request for a user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_hidden_listen_auth_email() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result : crate :: types :: VerificationToken = client . hidden () . listen_auth_email (& crate :: types :: EmailAuthenticationForm { callback_url : Some (url :: Url :: from_str (\"https://35.232.176.100.162.147/9\") ?) , email : \"188.158.31.243.125.154.235.245.86.177.162.236.11@223.236.115.127.63.211.76.146.214.141.181.68\" }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Listen for callbacks for email verification for users.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_hidden_listen_auth_email_callback() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client . hidden () . listen_auth_email_callback (Some (url :: Url :: from_str (\"https://61.188.125.115.174.82/4\") ?) , \"139.147.206.168.189.210.146.67.252.232.83@194.21.101.153.162.100.182.22.197.227.175.43.89\" , \"wkipumb\" ,) . await ? ;\n    Ok(())\n}\n```"]
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

        query_params.push(("email", format!("{}", email)));
        query_params.push(("token", format!("{}", token)));
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
