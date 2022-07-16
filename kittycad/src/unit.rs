use anyhow::Result;

use crate::Client;

pub struct Unit {
    pub client: Client,
}

impl Unit {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Unit { client }
    }

    #[doc = "Convert units.\n\nConvert a metric unit value to another metric unit value. This is a nice endpoint to use for helper functions."]
    pub async fn create_conversion(
        &self,
        output_format: crate::types::UnitMetricFormat,
        src_format: crate::types::UnitMetricFormat,
        value: f64,
    ) -> Result<crate::types::UnitConversion> {
        let mut rb = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "unit/conversion/{src_format}/{output_format}"
            ),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }
}
