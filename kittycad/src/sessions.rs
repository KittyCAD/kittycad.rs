use crate::Client;
use anyhow::Result;
pub struct Sessions {
    pub client: Client,
}

impl Sessions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get a session for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user.\n\n```\n/// Get a session for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user.\nasync fn example_get_session_for_user() -> anyhow::Result<()> {\n    let result: crate::types::Session = client\n        .sessions()\n        .get_session_for_user(uuid::Uuid::from_str(\n            \"4c274bdd-c43b-4ccf-8881-2b2ce95b7459\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
