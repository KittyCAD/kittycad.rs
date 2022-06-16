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

    /**
    * Convert units.
    *
    * This function performs a `POST` to the `/unit/conversion/{src_format}/{output_format}` endpoint.
    *
    * Convert a metric unit value to another metric unit value. This is a nice endpoint to use for helper functions.
    *
    * **Parameters:**
    *
    * * `output_format: crate::types::UnitMetricFormat` -- The valid types of metric unit formats.
    * * `src_format: crate::types::UnitMetricFormat` -- The valid types of metric unit formats.
    * * `value: f64` -- The initial value.
    */
    pub async fn create_conversion(
        &self,
        output_format: crate::types::UnitMetricFormat,
        src_format: crate::types::UnitMetricFormat,
        value: f64,
    ) -> Result<crate::types::UnitConversion> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !value.to_string().is_empty() {
            query_args.push(("value".to_string(), value.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/unit/conversion/{}/{}?{}",
            crate::progenitor_support::encode_path(&src_format.to_string()),
            crate::progenitor_support::encode_path(&output_format.to_string()),
            query_
        );

        self.client.post(&url, None).await
    }
}
