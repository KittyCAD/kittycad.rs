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

    /**
    * List API tokens for your user.
    *
    * This function performs a `GET` to the `/user/api-tokens` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns the API tokens for the authenticated user.
    * The API tokens are returned in order of creation, with the most recently created API tokens first.
    *
    * **Parameters:**
    *
    * * `limit: u32` -- Maximum number of items returned by a single call.
    * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
    * * `sort_by: crate::types::CreatedAtSortMode` -- Supported set of sort modes for scanning by created_at only.
    *  
    *  Currently, we only support scanning in ascending order.
    */
    pub async fn list_tokens_for_user(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ApiToken>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.to_string().is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/user/api-tokens?{}", query_);

        let resp: crate::types::ApiTokenResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
    * List API tokens for your user.
    *
    * This function performs a `GET` to the `/user/api-tokens` endpoint.
    *
    * As opposed to `list_tokens_for_user`, this function returns all the pages of the request at once.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns the API tokens for the authenticated user.
    * The API tokens are returned in order of creation, with the most recently created API tokens first.
    */
    pub async fn list_all_tokens_for_user(
        &self,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ApiToken>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/user/api-tokens?{}", query_);

        let mut resp: crate::types::ApiTokenResultsPage = self.client.get(&url, None).await?;

        let mut items = resp.items;
        let mut page = resp.next_page;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?page={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&page={}", url, page), None)
                    .await?;
            }

            items.append(&mut resp.items);

            if !resp.next_page.is_empty() && resp.next_page != page {
                page = resp.next_page.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(items)
    }

    /**
    * Create a new API token for your user.
    *
    * This function performs a `POST` to the `/user/api-tokens` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It creates a new API token for the authenticated user.
    */
    pub async fn create_token_for_user(&self) -> Result<crate::types::ApiToken> {
        let url = "/user/api-tokens".to_string();
        self.client.post(&url, None).await
    }

    /**
    * Get an API token for your user.
    *
    * This function performs a `GET` to the `/user/api-tokens/{token}` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user.
    *
    * **Parameters:**
    *
    * * `token: &str` -- The API token.
    */
    pub async fn get_token_for_user(&self, token: &str) -> Result<crate::types::ApiToken> {
        let url = format!(
            "/user/api-tokens/{}",
            crate::progenitor_support::encode_path(&token.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete an API token for your user.
    *
    * This function performs a `DELETE` to the `/user/api-tokens/{token}` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It deletes the requested API token for the user.
    * This endpoint does not actually delete the API token from the database. It merely marks the token as invalid. We still want to keep the token in the database for historical purposes.
    *
    * **Parameters:**
    *
    * * `token: &str` -- The API token.
    */
    pub async fn delete_token_for_user(&self, token: &str) -> Result<()> {
        let url = format!(
            "/user/api-tokens/{}",
            crate::progenitor_support::encode_path(&token.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
