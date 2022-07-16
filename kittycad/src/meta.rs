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
    pub async fn get_schema(&self) -> Result<serde_json::Value> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, ""),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Get the metadata about our currently running server.\n\nThis includes information on any of our other distributed systems it is connected to.\nYou must be a KittyCAD employee to perform this request."]
    pub async fn get_metadata(&self) -> Result<crate::types::Metadata> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "_meta/info"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Return pong."]
    pub async fn ping(&self) -> Result<crate::types::Pong> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "ping"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }
}
