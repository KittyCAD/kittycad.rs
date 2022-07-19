use anyhow::Result;

use crate::Client;
pub struct Sessions {
    pub client: Client,
}

impl Sessions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get a session for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_sessions_get_session_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Session = client\n        .sessions()\n        .get_session_for_user(uuid::Uuid::from_str(\n            \"eee99d0c-cd5d-4cc1-8e5c-f37a94090927\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    pub async fn get_session_for_user<'a>(
        &'a self,
        token: uuid::Uuid,
    ) -> Result<crate::types::Session, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/session/{token}".replace("{token}", &format!("{}", token))
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
