use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Ai {
    pub client: Client,
}

impl Ai {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Generate a 3D model from an image.\n\nThis is an alpha endpoint. It will change in the future. The current output is honestly pretty bad. So if you find this endpoint, you get what you pay for, which currently is nothing. But in the future will be made a lot better.\n\n**Parameters:**\n\n- `input_format: crate::types::ImageType`: The format of the image being converted. (required)\n- `output_format: crate::types::FileExportFormat`: The format the output file should be converted to. (required)\n\n```rust,no_run\nasync fn example_ai_create_image_to_3d() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Mesh = client\n        .ai()\n        .create_image_to_3d(\n            kittycad::types::ImageType::Png,\n            kittycad::types::FileExportFormat::Dae,\n            &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_image_to_3d<'a>(
        &'a self,
        input_format: crate::types::ImageType,
        output_format: crate::types::FileExportFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::Mesh, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "ai/image-to-3d/{input_format}/{output_format}"
                    .replace("{input_format}", &format!("{}", input_format))
                    .replace("{output_format}", &format!("{}", output_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.body(body.clone());
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

    #[doc = "Generate a 3D model from text.\n\nThis is an alpha endpoint. It will change in the future. The current output is honestly pretty bad. So if you find this endpoint, you get what you pay for, which currently is nothing. But in the future will be made a lot better.\n\n**Parameters:**\n\n- `output_format: crate::types::FileExportFormat`: The format the output file should be converted to. (required)\n- `prompt: &'astr`: The prompt for the model. (required)\n\n```rust,no_run\nasync fn example_ai_create_text_to_3d() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Mesh = client\n        .ai()\n        .create_text_to_3d(kittycad::types::FileExportFormat::Fbxb, \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_text_to_3d<'a>(
        &'a self,
        output_format: crate::types::FileExportFormat,
        prompt: &'a str,
    ) -> Result<crate::types::Mesh, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "ai/text-to-3d/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("prompt", prompt.to_string())];
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
