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

    #[doc = "Create an email verification request for a user.\n\n```\n/// Create an email verification request for a user.\nasync fn example_listen_auth_email() -> anyhow::Result<()> {\n    let result: crate::types::VerificationToken = client\n        .hidden()\n        .listen_auth_email(&crate::types::EmailAuthenticationForm {\n            callback_url: url::Url::from_str(\"http://238.115.135.249.201.76.75.130.171/0\")?,\n            email:\n                \"152.81.110.72.35.217.149.186.101@75.52.212.152.128.47.9.245.245.75.108.78.96.116\",\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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

    #[doc = "Listen for callbacks for email verification for users.\n\n```\n/// Listen for callbacks for email verification for users.\nasync fn example_listen_auth_email_callback() -> anyhow::Result<()> {\n    client . hidden () . listen_auth_email_callback (Some (url :: Url :: from_str (\"https://173.0.0.237/0\") ?) , \"71.129.35.235.56.172.81.229.21.129.99.213.53.90.139@99.0.42.1.30.146.10.228.146.62.21.49\" , \"\" . to_string () ,) . await ? ;\n    Ok(())\n}\n\n```"]
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

    #[doc = "This endpoint removes the session cookie for a user.\n\nThis is used in logout scenarios.\n\n```\n/// This endpoint removes the session cookie for a user.\n/// \n/// This is used in logout scenarios.\nasync fn example_logout() -> anyhow::Result<()> {\n    client.hidden().logout().await?;\n    Ok(())\n}\n\n```"]
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
