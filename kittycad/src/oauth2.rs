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

    #[doc = "Start an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be accessed from an *unauthenticated* API client. It generates and records a `device_code` and `user_code` which must be verified and confirmed prior to a token being granted.\n\n```rust,no_run\n\nuse std::str::FromStr;\nasync fn example_oauth2_device_auth_request() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        .device_auth_request(&kittycad::types::DeviceAuthRequestForm {\n            client_id: uuid::Uuid::from_str(\"3f5b0abb-f1fc-4529-a662-e72b62f663f7\")?,\n        })\n        .await?;\n    Ok(())\n}\n\n```"]
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Confirm an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be accessed by the user agent (browser), not the client requesting the token. So we do not actually return the token here; it will be returned in response to the poll on `/oauth2/device/token`.\n\n```rust,no_run\nasync fn example_oauth2_device_auth_confirm() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        .device_auth_confirm(&kittycad::types::DeviceAuthVerifyParams {\n            user_code: \"mufw\".to_string(),\n        })\n        .await?;\n    Ok(())\n}\n\n```"]
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Request a device access token.\n\nThis endpoint should be polled by the client until the user code is verified and the grant is confirmed.\n\n```rust,no_run\n\nuse std::str::FromStr;\nasync fn example_oauth2_device_access_token() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        .device_access_token(&kittycad::types::DeviceAccessTokenRequestForm {\n            client_id: uuid::Uuid::from_str(\"f49d5e87-ad38-400e-8fb5-7deb29d2d2a5\")?,\n            device_code: uuid::Uuid::from_str(\"81b928b4-1c30-47cf-9432-8fd5b0c224c7\")?,\n            grant_type:\n                kittycad::types::CrateTypesOauth2GrantType::UrnIetfParamsOauthGrantTypeDeviceCode,\n        })\n        .await?;\n    Ok(())\n}\n\n```"]
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Verify an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint should be accessed in a full user agent (e.g., a browser). If the user is not logged in, we redirect them to the login page and use the `callback_url` parameter to get them to the UI verification form upon logging in. If they are logged in, we redirect them to the UI verification form on the website.\n\n```rust,no_run\nasync fn example_oauth2_device_auth_verify() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client.oauth2().device_auth_verify(\"tdsqm\").await?;\n    Ok(())\n}\n\n```"]
    pub async fn device_auth_verify<'a>(
        &'a self,
        user_code: &'a str,
    ) -> Result<(), crate::types::error::Error> {
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
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Listen for callbacks for the OAuth 2.0 provider.\n\n```rust,no_run\nasync fn example_oauth2_listen_provider_callback() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .oauth2()\n        .listen_provider_callback(\n            Some(\"dyutbbry\".to_string()),\n            kittycad::types::AccountProvider::Google,\n            Some(\"mtivfmq\".to_string()),\n        )\n        .await?;\n    Ok(())\n}\n\n```"]
    pub async fn listen_provider_callback<'a>(
        &'a self,
        code: Option<String>,
        provider: crate::types::AccountProvider,
        state: Option<String>,
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
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get the consent URL and other information for the OAuth 2.0 provider.\n\n```rust,no_run\nasync fn example_oauth2_listen_provider_consent() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Oauth2ClientInfo = client\n        .oauth2()\n        .listen_provider_consent(Some(\"i\".to_string()), kittycad::types::AccountProvider::Github)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn listen_provider_consent<'a>(
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
        let mut query_params = Vec::new();
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
