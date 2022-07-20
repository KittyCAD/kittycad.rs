use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Apps {
    pub client: Client,
}

impl Apps {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Listen for callbacks to GitHub app authentication.\n\nThis is different than OAuth \
             2.0 authentication for users. This endpoint grants access for KittyCAD to access \
             user's repos.\nThe user doesn't need KittyCAD OAuth authorization for this endpoint, \
             this is purely for the GitHub permissions to access repos.\n\n```rust,no_run\nasync \
             fn example_apps_github_callback() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .apps()\n        \
             .github_callback(&serde_json::Value::String(\"some-string\".to_string()))\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn github_callback<'a>(
        &'a self,
        body: &serde_json::Value,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "apps/github/callback"),
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

    #[doc = "Get the consent URL for GitHub app authentication.\n\nThis is different than OAuth \
             2.0 authentication for users. This endpoint grants access for KittyCAD to access \
             user's repos.\nThe user doesn't need KittyCAD OAuth authorization for this endpoint, \
             this is purely for the GitHub permissions to access repos.\n\n```rust,no_run\nasync \
             fn example_apps_github_consent() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::AppClientInfo = \
             client.apps().github_consent().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn github_consent<'a>(
        &'a self,
    ) -> Result<crate::types::AppClientInfo, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "apps/github/consent"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
