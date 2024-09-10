use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Oauth2 {
    pub client: Client,
}

impl Oauth2 {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Start an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be \
             accessed from an *unauthenticated* API client. It generates and records a \
             `device_code` and `user_code` which must be verified and confirmed prior to a token \
             being granted.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_oauth2_device_auth_request() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        \
             .device_auth_request(&kittycad::types::DeviceAuthRequestForm {\n            \
             client_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        \
             })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn device_auth_request<'a>(
        &'a self,
        body: &crate::types::DeviceAuthRequestForm,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/auth"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.form(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Confirm an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be \
             accessed by the user agent (browser), not the client requesting the token. So we do \
             not actually return the token here; it will be returned in response to the poll on \
             `/oauth2/device/token`.\n\n```rust,no_run\nasync fn \
             example_oauth2_device_auth_confirm() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        \
             .device_auth_confirm(&kittycad::types::DeviceAuthVerifyParams {\n            \
             user_code: \"some-string\".to_string(),\n        })\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn device_auth_confirm<'a>(
        &'a self,
        body: &crate::types::DeviceAuthVerifyParams,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/confirm"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Request a device access token.\n\nThis endpoint should be polled by the client until the user code is verified and the grant is confirmed.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_oauth2_device_access_token() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        .device_access_token(&kittycad::types::DeviceAccessTokenRequestForm {\n            client_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            device_code: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            grant_type: kittycad::types::Oauth2GrantType::UrnIetfParamsOauthGrantTypeDeviceCode,\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn device_access_token<'a>(
        &'a self,
        body: &crate::types::DeviceAccessTokenRequestForm,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/device/token"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.form(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Verify an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint should be accessed \
             in a full user agent (e.g., a browser). If the user is not logged in, we redirect \
             them to the login page and use the `callback_url` parameter to get them to the UI \
             verification form upon logging in. If they are logged in, we redirect them to the UI \
             verification form on the website.\n\n**Parameters:**\n\n- `user_code: &'astr`: The \
             user code. (required)\n\n```rust,no_run\nasync fn example_oauth2_device_auth_verify() \
             -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client.oauth2().device_auth_verify(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn device_auth_verify<'a>(
        &'a self,
        user_code: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "oauth2/device/verify"),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("user_code", user_code.to_string())];
        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Listen for callbacks for the OAuth 2.0 provider.\n\n**Parameters:**\n\n- `code: Option<String>`: The authorization code.\n- `id_token: Option<String>`: For Apple only, a JSON web token containing the user’s identity information.\n- `provider: crate::types::AccountProvider`: The provider. (required)\n- `state: Option<String>`: The state that we had passed in through the user consent URL.\n- `user: Option<String>`: For Apple only, a JSON string containing the data requested in the scope property. The returned data is in the following format: `{ \"name\": { \"firstName\": string, \"lastName\": string }, \"email\": string }`\n\n```rust,no_run\nasync fn example_oauth2_oauth_2_provider_callback() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        .oauth_2_provider_callback(\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            kittycad::types::AccountProvider::Tencent,\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn oauth_2_provider_callback<'a>(
        &'a self,
        code: Option<String>,
        id_token: Option<String>,
        provider: crate::types::AccountProvider,
        state: Option<String>,
        user: Option<String>,
    ) -> Result<(), crate::types::error::Error> {
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
        let mut query_params = vec![];
        if let Some(p) = code {
            query_params.push(("code", p));
        }

        if let Some(p) = id_token {
            query_params.push(("id_token", p));
        }

        if let Some(p) = state {
            query_params.push(("state", p));
        }

        if let Some(p) = user {
            query_params.push(("user", p));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Listen for callbacks for the OAuth 2.0 provider.\n\nThis specific endpoint listens \
             for posts of form data.\n\n**Parameters:**\n\n- `provider: \
             crate::types::AccountProvider`: The provider. (required)\n\n```rust,no_run\nasync fn \
             example_oauth2_oauth_2_provider_callback_post() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        \
             .oauth_2_provider_callback_post(\n            \
             kittycad::types::AccountProvider::Tencent,\n            \
             &kittycad::types::AuthCallback {\n                code: \
             Some(\"some-string\".to_string()),\n                id_token: \
             Some(\"some-string\".to_string()),\n                state: \
             Some(\"some-string\".to_string()),\n                user: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn oauth_2_provider_callback_post<'a>(
        &'a self,
        provider: crate::types::AccountProvider,
        body: &crate::types::AuthCallback,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "oauth2/provider/{provider}/callback"
                    .replace("{provider}", &format!("{}", provider))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.form(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get the consent URL and other information for the OAuth 2.0 \
             provider.\n\n**Parameters:**\n\n- `callback_url: Option<String>`: The URL to redirect \
             back to after we have authenticated.\n- `provider: crate::types::AccountProvider`: \
             The provider. (required)\n\n```rust,no_run\nasync fn \
             example_oauth2_oauth_2_provider_consent() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::Oauth2ClientInfo \
             = client\n        .oauth2()\n        .oauth_2_provider_consent(\n            \
             Some(\"some-string\".to_string()),\n            \
             kittycad::types::AccountProvider::Tencent,\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn oauth_2_provider_consent<'a>(
        &'a self,
        callback_url: Option<String>,
        provider: crate::types::AccountProvider,
    ) -> Result<crate::types::Oauth2ClientInfo, crate::types::error::Error> {
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
        let mut query_params = vec![];
        if let Some(p) = callback_url {
            query_params.push(("callback_url", p));
        }

        req = req.query(&query_params);
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Revoke an OAuth2 token.\n\nThis endpoint is designed to be accessed from an *unauthenticated* API client.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_oauth2_oauth_2_token_revoke() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        .oauth_2_token_revoke(&kittycad::types::TokenRevokeRequestForm {\n            client_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            client_secret: Some(\"some-string\".to_string()),\n            token: \"some-string\".to_string(),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn oauth_2_token_revoke<'a>(
        &'a self,
        body: &crate::types::TokenRevokeRequestForm,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "oauth2/token/revoke"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.form(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
