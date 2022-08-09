use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Constant {
    pub client: Client,
}

impl Constant {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get a physics constant.\n\n**Parameters:**\n\n- `constant: crate::types::PhysicsConstantName`: The constant to get. (required)\n\n```rust,no_run\nasync fn example_constant_get_physics() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::PhysicsConstant = client\n        .constant()\n        .get_physics(kittycad::types::PhysicsConstantName::MolarGasConst)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_physics<'a>(
        &'a self,
        constant: crate::types::PhysicsConstantName,
    ) -> Result<crate::types::PhysicsConstant, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "constant/physics/{constant}".replace("{constant}", &format!("{}", constant))
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
