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
    pub fn create_conversion(&self) -> Result<()> {
        Ok(())
    }
}
