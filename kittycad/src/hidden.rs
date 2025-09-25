use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Hidden {
    pub client: Client,
}

impl Hidden {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Authenticate using an api-key. This is disabled on production but can be used in dev \
             to login without email magic.\n\nThis returns a session \
             token.\n\n```rust,no_run\nasync fn example_hidden_auth_api_key() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::AuthApiKeyResponse = \
             client.hidden().auth_api_key().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn auth_api_key<'a>(
        &'a self,
    ) -> Result<crate::types::AuthApiKeyResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "auth/api-key"),
        );
        req = req.bearer_auth(&self.client.token);
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Create an email verification request for a user.\n\n```rust,no_run\nasync fn example_hidden_auth_email() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::VerificationTokenResponse = client\n        .hidden()\n        .auth_email(&kittycad::types::EmailAuthenticationForm {\n            callback_url: Some(\"https://example.com/foo/bar\".to_string()),\n            email: \"email@example.com\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn auth_email<'a>(
        &'a self,
        body: &crate::types::EmailAuthenticationForm,
    ) -> Result<crate::types::VerificationTokenResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "auth/email"),
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Listen for callbacks for email authentication for users.\n\n**Parameters:**\n\n- `callback_url: Option<String>`: The URL to redirect back to after we have authenticated.\n- `email: &'astr`: The user's email. (required)\n- `token: &'astr`: The verification token. (required)\n\n```rust,no_run\nasync fn example_hidden_auth_email_callback() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .hidden()\n        .auth_email_callback(\n            Some(\"https://example.com/foo/bar\".to_string()),\n            \"email@example.com\",\n            \"some-string\",\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn auth_email_callback<'a>(
        &'a self,
        callback_url: Option<String>,
        email: &'a str,
        token: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "auth/email/callback"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![
            ("email", email.to_string()),
            ("token", token.to_string()),
        ];
        if let Some(p) = callback_url {
            query_params.push(("callback_url", p));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "GET /auth/saml/{org_id}\n\nRedirects the browser straight to the org’s SAML IdP.\n\n**Parameters:**\n\n- `callback_url: Option<String>`: The URL to redirect back to after we have authenticated.\n- `org_id: uuid::Uuid`: The ID of the organisation that owns the IdP. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_hidden_get_auth_saml_by_org() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .hidden()\n        .get_auth_saml_by_org(\n            Some(\"https://example.com/foo/bar\".to_string()),\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_auth_saml_by_org<'a>(
        &'a self,
        callback_url: Option<String>,
        org_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "auth/saml/org/{org_id}/login".replace("{org_id}", &format!("{}", org_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = callback_url {
            query_params.push(("callback_url", p));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get a redirect straight to the SAML IdP.\n\nThe UI uses this to avoid having to ask the API anything about the IdP. It already knows the SAML IdP ID from the path, so it can just link to this path and rely on the API to redirect to the actual IdP.\n\n**Parameters:**\n\n- `callback_url: Option<String>`: The URL to redirect back to after we have authenticated.\n- `provider_id: uuid::Uuid`: The ID of the identity provider. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_hidden_get_auth_saml() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .hidden()\n        .get_auth_saml(\n            Some(\"https://example.com/foo/bar\".to_string()),\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_auth_saml<'a>(
        &'a self,
        callback_url: Option<String>,
        provider_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "auth/saml/provider/{provider_id}/login"
                    .replace("{provider_id}", &format!("{}", provider_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = callback_url {
            query_params.push(("callback_url", p));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Authenticate a user via SAML\n\n**Parameters:**\n\n- `provider_id: uuid::Uuid`: The \
             ID of the identity provider. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_hidden_post_auth_saml() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    client\n        \
             .hidden()\n        .post_auth_saml(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_auth_saml<'a>(
        &'a self,
        provider_id: uuid::Uuid,
        body: &bytes::Bytes,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "auth/saml/provider/{provider_id}/login"
                    .replace("{provider_id}", &format!("{}", provider_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.body(body.clone());
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "This endpoint removes the session cookie for a user.\n\nThis is used in logout \
             scenarios.\n\n```rust,no_run\nasync fn example_hidden_logout() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    \
             client.hidden().logout().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn logout<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "logout"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Redirect the user to the URL for the shortlink.\n\nThis endpoint might require \
             authentication by a Zoo user. It gets the shortlink for the user and redirects them \
             to the URL. If the shortlink is owned by an org, the user must be a member of the \
             org.\n\n**Parameters:**\n\n- `key: &'astr`: The key of the shortlink. \
             (required)\n\n```rust,no_run\nasync fn example_hidden_redirect_user_shortlink() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .hidden()\n        .redirect_user_shortlink(\"some-string\")\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn redirect_user_shortlink<'a>(
        &'a self,
        key: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/shortlinks/{key}".replace("{key}", key)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
