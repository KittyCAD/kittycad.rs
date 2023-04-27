use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Drawing {
    pub client: Client,
}

impl Drawing {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Submit one drawing operation.\n\nResponse depends on which command was submitted, so unfortunately the OpenAPI schema can't generate the right response type.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_drawing_cmd() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: serde_json::Value = client\n        .drawing()\n        .cmd(&kittycad::types::DrawingCmdReq {\n            cmd: kittycad::types::DrawingCmd::DrawingCmd(kittycad::types::DrawingCmd {\n                extrude: kittycad::types::Extrude {\n                    distance: 3.14 as f64,\n                    sketch: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                },\n            }),\n            cmd_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            file_id: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn cmd<'a>(
        &'a self,
        body: &crate::types::DrawingCmdReq,
    ) -> Result<serde_json::Value, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "drawing/cmd"),
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

    #[doc = "Submit many drawing operations.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_drawing_cmd_batch() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::DrawingOutcomes = client\n        .drawing()\n        .cmd_batch(&kittycad::types::DrawingCmdReqBatch {\n            cmds: std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                kittycad::types::DrawingCmdReq {\n                    cmd: kittycad::types::DrawingCmd::DrawingCmd(kittycad::types::DrawingCmd {\n                        extrude: kittycad::types::Extrude {\n                            distance: 3.14 as f64,\n                            sketch: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                        },\n                    }),\n                    cmd_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                    file_id: \"some-string\".to_string(),\n                },\n            )]),\n            file_id: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn cmd_batch<'a>(
        &'a self,
        body: &crate::types::DrawingCmdReqBatch,
    ) -> Result<crate::types::DrawingOutcomes, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "drawing/cmd_batch"),
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
}
