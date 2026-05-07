use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Store {
    pub client: Client,
}

impl Store {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Create a new store coupon.\n\nThis endpoint requires authentication by a Zoo \
             employee. It creates a new store coupon.\n\n```rust,no_run\nasync fn \
             example_store_create_coupon() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::DiscountCode = \
             client\n        .store()\n        .create_coupon(&kittycad::types::StoreCouponParams \
             {\n            percent_off: 4 as u32,\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_coupon<'a>(
        &'a self,
        body: &crate::types::StoreCouponParams,
    ) -> Result<crate::types::DiscountCode, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "store/coupon"),
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
