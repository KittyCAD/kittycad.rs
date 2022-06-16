use anyhow::Result;

use crate::Client;

pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
    * Get your user.
    *
    * This function performs a `GET` to the `/user` endpoint.
    *
    * Get the user information for the authenticated user.
    * Alternatively, you can also use the `/users/me` endpoint.
    */
    pub async fn get_self(&self) -> Result<crate::types::User> {
        let url = "/user".to_string();
        self.client.get(&url, None).await
    }

    /**
    * Update your user.
    *
    * This function performs a `PUT` to the `/user` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It updates information about the authenticated user.
    */
    pub async fn update_self(&self, body: &crate::types::UpdateUser) -> Result<crate::types::User> {
        let url = "/user".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get extended information about your user.
    *
    * This function performs a `GET` to the `/user/extended` endpoint.
    *
    * Get the user information for the authenticated user.
    * Alternatively, you can also use the `/users/me` endpoint.
    */
    pub async fn get_self_extended(&self) -> Result<crate::types::ExtendedUser> {
        let url = "/user/extended".to_string();
        self.client.get(&url, None).await
    }

    /**
    * List users.
    *
    * This function performs a `GET` to the `/users` endpoint.
    *
    * This endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.
    *
    * **Parameters:**
    *
    * * `limit: u32` -- Maximum number of items returned by a single call.
    * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
    * * `sort_by: crate::types::CreatedAtSortMode` -- Supported set of sort modes for scanning by created_at only.
    *  
    *  Currently, we only support scanning in ascending order.
    */
    pub async fn list(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::User>> {
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
        let url = format!("/users?{}", query_);

        let resp: crate::types::UserResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
    * List users.
    *
    * This function performs a `GET` to the `/users` endpoint.
    *
    * As opposed to `list`, this function returns all the pages of the request at once.
    *
    * This endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.
    */
    pub async fn list_all(
        &self,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users?{}", query_);

        let mut resp: crate::types::UserResultsPage = self.client.get(&url, None).await?;

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
    * List users with extended information.
    *
    * This function performs a `GET` to the `/users-extended` endpoint.
    *
    * This endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.
    *
    * **Parameters:**
    *
    * * `limit: u32` -- Maximum number of items returned by a single call.
    * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
    * * `sort_by: crate::types::CreatedAtSortMode` -- Supported set of sort modes for scanning by created_at only.
    *  
    *  Currently, we only support scanning in ascending order.
    */
    pub async fn list_extended(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ExtendedUser>> {
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
        let url = format!("/users-extended?{}", query_);

        let resp: crate::types::ExtendedUserResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
    * List users with extended information.
    *
    * This function performs a `GET` to the `/users-extended` endpoint.
    *
    * As opposed to `list_extended`, this function returns all the pages of the request at once.
    *
    * This endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.
    */
    pub async fn list_all_extended(
        &self,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ExtendedUser>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users-extended?{}", query_);

        let mut resp: crate::types::ExtendedUserResultsPage = self.client.get(&url, None).await?;

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
    * Get extended information about a user.
    *
    * This function performs a `GET` to the `/users-extended/{id}` endpoint.
    *
    * To get information about yourself, use `/users-extended/me` as the endpoint. By doing so you will get the user information for the authenticated user.
    * Alternatively, to get information about the authenticated user, use `/user/extended` endpoint.
    * To get information about any KittyCAD user, you must be a KittyCAD employee.
    *
    * **Parameters:**
    *
    * * `id: &str` -- The user ID.
    */
    pub async fn get_extended(&self, id: &str) -> Result<crate::types::ExtendedUser> {
        let url = format!(
            "/users-extended/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Get a user.
    *
    * This function performs a `GET` to the `/users/{id}` endpoint.
    *
    * To get information about yourself, use `/users/me` as the endpoint. By doing so you will get the user information for the authenticated user.
    * Alternatively, to get information about the authenticated user, use `/user` endpoint.
    * To get information about any KittyCAD user, you must be a KittyCAD employee.
    *
    * **Parameters:**
    *
    * * `id: &str` -- The user ID.
    */
    pub async fn get(&self, id: &str) -> Result<crate::types::User> {
        let url = format!(
            "/users/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
