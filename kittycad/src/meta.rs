use anyhow::Result;

use crate::Client;

pub struct Meta {
    pub client: Client,
}

impl Meta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Meta { client }
    }

    #[doc = "Get OpenAPI schema."]
    pub fn get_schema(&self) -> Result<serde_json::Value> {
        todo!()
    }

    #[doc = "Get the metadata about our currently running server.\n\nThis includes information on any of our other distributed systems it is connected to.\nYou must be a KittyCAD employee to perform this request."]
    pub fn get_metadata(&self) -> Result<Metadata> {
        todo!()
    }

    #[doc = "Return pong."]
    pub fn ping(&self) -> Result<Pong> {
        todo!()
    }
}
