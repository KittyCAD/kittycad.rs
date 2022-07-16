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
        _output_format: crate::types::UnitMetricFormat,
        _src_format: crate::types::UnitMetricFormat,
        _value: f64,
    ) -> Result<crate::types::UnitConversion> {
        todo!()
    }
}
