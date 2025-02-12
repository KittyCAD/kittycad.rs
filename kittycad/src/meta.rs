use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Meta {
    pub client: Client,
}

impl Meta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get OpenAPI schema.\n\n```rust,no_run\nasync fn example_meta_get_schema() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: serde_json::Value = client.meta().get_schema().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_schema<'a>(&'a self) -> Result<serde_json::Value, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, ""),
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

    #[doc = "Get the metadata about our currently running server.\n\nThis includes information on \
             any of our other distributed systems it is connected to.\n\nYou must be a Zoo \
             employee to perform this request.\n\n```rust,no_run\nasync fn \
             example_meta_get_metadata() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::Metadata = \
             client.meta().get_metadata().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_metadata<'a>(
        &'a self,
    ) -> Result<crate::types::Metadata, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "_meta/info"),
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

    #[doc = "Get ip address information.\n\n```rust,no_run\nasync fn example_meta_get_ipinfo() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::IpAddrInfo = client.meta().get_ipinfo().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_ipinfo<'a>(
        &'a self,
    ) -> Result<crate::types::IpAddrInfo, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "_meta/ipinfo"),
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

    #[doc = "Authorize an inbound auth request from our Community page.\n\n**Parameters:**\n\n- \
             `sig: &'astr`: The signature for the given payload (required)\n- `sso: &'astr`: The \
             nonce and redirect URL sent to us by Discourse (required)\n\n```rust,no_run\nasync fn \
             example_meta_community_sso() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .meta()\n        \
             .community_sso(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn community_sso<'a>(
        &'a self,
        sig: &'a str,
        sso: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "community/sso"),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("sig", sig.to_string()), ("sso", sso.to_string())];
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

    #[doc = "Uploads files to public blob storage for debugging purposes.\n\nDo NOT send files \
             here that you don't want to be public.\n\n```rust,no_run\nasync fn \
             example_meta_create_debug_uploads() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: Vec<String> = client\n        \
             .meta()\n        .create_debug_uploads(vec![kittycad::types::multipart::Attachment \
             {\n            name: \"thing\".to_string(),\n            filename: \
             Some(\"myfile.json\".to_string()),\n            content_type: \
             Some(\"application/json\".to_string()),\n            data: \
             std::fs::read(\"myfile.json\").unwrap(),\n        }])\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_debug_uploads<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
    ) -> Result<Vec<String>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "debug/uploads"),
        );
        req = req.bearer_auth(&self.client.token);
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
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

    #[doc = "Creates an internal telemetry event.\n\nWe collect anonymous telemetry data for improving our product.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_meta_create_event() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .meta()\n        .create_event(\n            vec![kittycad::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filename: Some(\"myfile.json\".to_string()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            &kittycad::types::Event {\n                attachment_uri: Some(\"some-string\".to_string()),\n                created_at: chrono::Utc::now(),\n                event_type: kittycad::types::ModelingAppEventType::SuccessfulCompileBeforeClose,\n                last_compiled_at: Some(chrono::Utc::now()),\n                project_description: Some(\"some-string\".to_string()),\n                project_name: \"some-string\".to_string(),\n                source_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                type_: kittycad::types::Type::ModelingAppEvent,\n                user_id: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_event<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        body: &crate::types::Event,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "events"),
        );
        req = req.bearer_auth(&self.client.token);
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        let mut json_part = reqwest::multipart::Part::text(serde_json::to_string(&body)?);
        json_part = json_part.file_name(format!("{}.json", "body"));
        json_part = json_part.mime_str("application/json")?;
        form = form.part("body", json_part);
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
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

    #[doc = "Get an API token for a user by their discord id.\n\nThis endpoint allows us to run \
             API calls from our discord bot on behalf of a user. The user must have a discord \
             account linked to their Zoo Account via oauth2 for this to work.\n\nYou must be a Zoo \
             employee to use this endpoint.\n\n**Parameters:**\n\n- `discord_id: &'astr`: The \
             user's discord ID. (required)\n\n```rust,no_run\nasync fn \
             example_meta_internal_get_api_token_for_discord_user() -> anyhow::Result<()> {\n    \
             let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::ApiToken = client\n        .meta()\n        \
             .internal_get_api_token_for_discord_user(\"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn internal_get_api_token_for_discord_user<'a>(
        &'a self,
        discord_id: &'a str,
    ) -> Result<crate::types::ApiToken, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "internal/discord/api-token/{discord_id}".replace("{discord_id}", discord_id)
            ),
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

    #[doc = "Return pong.\n\n```rust,no_run\nasync fn example_meta_ping() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::Pong = client.meta().ping().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn ping<'a>(&'a self) -> Result<crate::types::Pong, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "ping"),
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

    #[doc = "Get the pricing for our subscriptions.\n\nThis is the ultimate source of truth for the pricing of our subscriptions.\n\n```rust,no_run\nasync fn example_meta_get_pricing_subscriptions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: std::collections::HashMap<String, Vec<kittycad::types::ZooProductSubscription>> =\n        client.meta().get_pricing_subscriptions().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_pricing_subscriptions<'a>(
        &'a self,
    ) -> Result<
        std::collections::HashMap<String, Vec<crate::types::ZooProductSubscription>>,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "pricing/subscriptions"),
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
}
