use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Modeling {
    pub client: Client,
}

impl Modeling {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Submit one modeling operation.\n\nResponse depends on which command was submitted, so \
             unfortunately the OpenAPI schema can't generate the right response \
             type.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_modeling_cmd() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: serde_json::Value = client\n        .modeling()\n        \
             .cmd(&kittycad::types::ModelingCmdReq {\n            cmd: \
             kittycad::types::ModelingCmd::CameraDragEnd {\n                interaction: \
             kittycad::types::CameraDragInteractionType::Zoom,\n                window: \
             kittycad::types::Point2D {\n                    x: 3.14 as f64,\n                    \
             y: 3.14 as f64,\n                },\n            },\n            cmd_id: \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        })\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn cmd<'a>(
        &'a self,
        body: &crate::types::ModelingCmdReq,
    ) -> Result<serde_json::Value, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "modeling/cmd"),
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

    #[doc = "Submit many modeling operations.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_modeling_cmd_batch() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ModelingOutcomes = client\n        .modeling()\n        .cmd_batch(&kittycad::types::ModelingCmdReqBatch {\n            cmds: std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                kittycad::types::ModelingCmdReq {\n                    cmd: kittycad::types::ModelingCmd::CameraDragEnd {\n                        interaction: kittycad::types::CameraDragInteractionType::Zoom,\n                        window: kittycad::types::Point2D {\n                            x: 3.14 as f64,\n                            y: 3.14 as f64,\n                        },\n                    },\n                    cmd_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                },\n            )]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn cmd_batch<'a>(
        &'a self,
        body: &crate::types::ModelingCmdReqBatch,
    ) -> Result<crate::types::ModelingOutcomes, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "modeling/cmd-batch"),
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

    #[doc = "Open a websocket which accepts modeling commands.\n\nPass those commands to the \
             engine via websocket, and pass responses back to the client. Basically, this is a \
             websocket proxy between the frontend/client and the engine.\n\n**Parameters:**\n\n- \
             `fps: Option<u32>`: Frames per second of the video feed.\n- `unlocked_framerate: \
             Option<bool>`: If true, engine will render video frames as fast as it can.\n- \
             `video_res_height: Option<u32>`: Height of the video feed. Must be a multiple of \
             4.\n- `video_res_width: Option<u32>`: Width of the video feed. Must be a multiple of \
             4."]
    #[tracing::instrument]
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn commands_ws<'a>(
        &'a self,
        fps: Option<u32>,
        unlocked_framerate: Option<bool>,
        video_res_height: Option<u32>,
        video_res_width: Option<u32>,
    ) -> Result<reqwest::Upgraded, crate::types::error::Error> {
        let mut req = self.client.client_http1_only.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "ws/modeling/commands"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = fps {
            query_params.push(("fps", format!("{}", p)));
        }

        if let Some(p) = unlocked_framerate {
            query_params.push(("unlocked_framerate", format!("{}", p)));
        }

        if let Some(p) = video_res_height {
            query_params.push(("video_res_height", format!("{}", p)));
        }

        if let Some(p) = video_res_width {
            query_params.push(("video_res_width", format!("{}", p)));
        }

        req = req.query(&query_params);
        req = req
            .header(reqwest::header::CONNECTION, "Upgrade")
            .header(reqwest::header::UPGRADE, "websocket")
            .header(reqwest::header::SEC_WEBSOCKET_VERSION, "13")
            .header(
                reqwest::header::SEC_WEBSOCKET_KEY,
                base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    rand::random::<[u8; 16]>(),
                ),
            );
        let resp = req.send().await?;
        if resp.status().is_client_error() || resp.status().is_server_error() {
            return Err(crate::types::error::Error::UnexpectedResponse(resp));
        }

        let upgraded = resp
            .upgrade()
            .await
            .map_err(crate::types::error::Error::RequestError)?;
        Ok(upgraded)
    }
}
