use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Executor {
    pub client: Client,
}

impl Executor {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Execute a KittyCAD program in a specific language.\n\n**Parameters:**\n\n- `lang: \
             crate::types::CodeLanguage`: The language of the code. (required)\n- `output: \
             Option<String>`: The output file we want to get the contents for (the paths are \
             relative to where in litterbox it is being run). You can denote more than one file \
             with a comma separated list of string paths.\n\n```rust,no_run\nasync fn \
             example_executor_create_file_execution() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::CodeOutput = \
             client\n        .executor()\n        .create_file_execution(\n            \
             kittycad::types::CodeLanguage::Python,\n            \
             Some(\"some-string\".to_string()),\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_file_execution<'a>(
        &'a self,
        lang: crate::types::CodeLanguage,
        output: Option<String>,
        body: &bytes::Bytes,
    ) -> Result<crate::types::CodeOutput, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "file/execute/{lang}".replace("{lang}", &format!("{}", lang))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = output {
            query_params.push(("output", p));
        }

        req = req.query(&query_params);
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

    #[doc = "Create a terminal.\n\nAttach to a docker container to create an interactive \
             terminal.\n\n```rust,no_run\nasync fn example_executor_create_term() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client.executor().create_term().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_term<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "ws/executor/term"),
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
