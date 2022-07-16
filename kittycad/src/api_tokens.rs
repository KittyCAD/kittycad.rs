use anyhow::Result;

use crate::Client;

pub struct ApiTokens {
    pub client: Client,
}

impl ApiTokens {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ApiTokens { client }
    }

    #[doc = "List API tokens for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API tokens for the authenticated user.\nThe API tokens are returned in order of creation, with the most recently created API tokens first."]
    pub async fn list_for_user(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiTokenResultsPage> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/api-tokens"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }
        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }
        if let Some(p) = sort_by {
            query_params.push(("sort_by", format!("{}", p)));
        }
        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }
    #[doc = "List API tokens for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API tokens for the authenticated user.\nThe API tokens are returned in order of creation, with the most recently created API tokens first."]
    pub async fn list_for_user_stream(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiTokenResultsPage> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/api-tokens"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }
        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }
        if let Some(p) = sort_by {
            query_params.push(("sort_by", format!("{}", p)));
        }
        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }
    #[doc = "Create a new API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new API token for the authenticated user."]
    pub async fn create_api_token_for_user(&self) -> Result<crate::types::ApiToken> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "user/api-tokens"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }
    #[doc = "Get an API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user."]
    pub async fn get_api_token_for_user(
        &self,
        token: uuid::Uuid,
    ) -> Result<crate::types::ApiToken> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/api-tokens/{token}".replace("{token}", &format!("{}", token))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }
    #[doc = "Delete an API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the requested API token for the user.\nThis endpoint does not actually delete the API token from the database. It merely marks the token as invalid. We still want to keep the token in the database for historical purposes."]
    pub async fn delete_api_token_for_user(&self, token: uuid::Uuid) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/api-tokens/{token}".replace("{token}", &format!("{}", token))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }
}
