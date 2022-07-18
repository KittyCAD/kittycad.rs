use crate::Client;
use anyhow::Result;
pub struct Meta {
    pub client: Client,
}

impl Meta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get OpenAPI schema.\n\n```\n/// Get OpenAPI schema.\nasync fn example_get_schema() -> anyhow::Result<()> {\n    let result: serde_json::Value = client.meta().get_schema().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_schema<'a>(&'a self) -> Result<serde_json::Value, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, ""),
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get the metadata about our currently running server.\n\nThis includes information on any of our other distributed systems it is connected to.\nYou must be a KittyCAD employee to perform this request.\n\n```\n/// Get the metadata about our currently running server.\n/// \n/// This includes information on any of our other distributed systems it is connected to.\n/// You must be a KittyCAD employee to perform this request.\nasync fn example_get_metadata() -> anyhow::Result<()> {\n    let result: crate::types::Metadata = client.meta().get_metadata().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_metadata<'a>(
        &'a self,
    ) -> Result<crate::types::Metadata, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "_meta/info"),
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Return pong.\n\n```\n/// Return pong.\nasync fn example_ping() -> anyhow::Result<()> {\n    let result: crate::types::Pong = client.meta().ping().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn ping<'a>(&'a self) -> Result<crate::types::Pong, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "ping"),
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
