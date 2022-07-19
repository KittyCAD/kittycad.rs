use anyhow::Result;

use crate::Client;
pub struct Unit {
    pub client: Client,
}

impl Unit {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Convert units.\n\nConvert a metric unit value to another metric unit value. This is a \
             nice endpoint to use for helper functions.\n\n```rust,no_run\nasync fn \
             example_unit_create_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitConversion = \
             client\n        .unit()\n        .create_conversion(\n            \
             kittycad::types::UnitMetricFormat::Giga,\n            \
             kittycad::types::UnitMetricFormat::Exa,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    pub async fn create_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMetricFormat,
        src_format: crate::types::UnitMetricFormat,
        value: f64,
    ) -> Result<crate::types::UnitConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
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
