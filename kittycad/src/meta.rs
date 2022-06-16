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

    /**
     * Get OpenAPI schema.
     *
     * This function performs a `GET` to the `/` endpoint.
     */
    pub async fn get_schema(&self) -> Result<()> {
        let url = "".to_string();

        self.client.get(&url, None).await
    }

    /**
     * Get the metadata about our currently running server.
     *
     * This function performs a `GET` to the `/_meta/info` endpoint.
     *
     * This includes information on any of our other distributed systems it is connected to.
     * You must be a KittyCAD employee to perform this request.
     */
    pub async fn get_data(&self) -> Result<crate::types::Metadata> {
        let url = "/_meta/info".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Return pong.
     *
     * This function performs a `GET` to the `/ping` endpoint.
     */
    pub async fn ping(&self) -> Result<crate::types::Pong> {
        let url = "/ping".to_string();
        self.client.get(&url, None).await
    }
}
