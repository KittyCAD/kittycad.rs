use anyhow::Result;

use crate::Client;

pub struct ApiCalls {
    pub client: Client,
}

impl ApiCalls {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ApiCalls { client }
    }

    /**
    * Get API call metrics.
    *
    * This function performs a `GET` to the `/api-call-metrics` endpoint.
    *
    * This endpoint requires authentication by a KittyCAD employee. The API calls are grouped by the parameter passed.
    *
    * **Parameters:**
    *
    * * `group_by: crate::types::ApiCallQueryGroupBy` -- The field of an API call to group by.
    */
    pub async fn get_call_metrics(
        &self,
        group_by: crate::types::ApiCallQueryGroupBy,
    ) -> Result<Vec<crate::types::ApiCallQueryGroup>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !group_by.to_string().is_empty() {
            query_args.push(("group_by".to_string(), group_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/api-call-metrics?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * List API calls.
    *
    * This function performs a `GET` to the `/api-calls` endpoint.
    *
    * This endpoint requires authentication by a KittyCAD employee. The API calls are returned in order of creation, with the most recently created API calls first.
    *
    * **Parameters:**
    *
    * * `limit: u32` -- Maximum number of items returned by a single call.
    * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
    * * `sort_by: crate::types::CreatedAtSortMode` -- Supported set of sort modes for scanning by created_at only.
    *  
    *  Currently, we only support scanning in ascending order.
    */
    pub async fn list_calls(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ApiCallWithPrice>> {
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
        let url = format!("/api-calls?{}", query_);

        let resp: crate::types::ApiCallWithPriceResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
    * List API calls.
    *
    * This function performs a `GET` to the `/api-calls` endpoint.
    *
    * As opposed to `list_calls`, this function returns all the pages of the request at once.
    *
    * This endpoint requires authentication by a KittyCAD employee. The API calls are returned in order of creation, with the most recently created API calls first.
    */
    pub async fn list_all_calls(
        &self,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ApiCallWithPrice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/api-calls?{}", query_);

        let mut resp: crate::types::ApiCallWithPriceResultsPage =
            self.client.get(&url, None).await?;

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
    * Get details of an API call.
    *
    * This function performs a `GET` to the `/api-calls/{id}` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns details of the requested API call for the user.
    * If the user is not authenticated to view the specified API call, then it is not returned.
    * Only KittyCAD employees can view API calls for other users.
    *
    * **Parameters:**
    *
    * * `id: &str` -- The ID of the API call.
    */
    pub async fn get_call(&self, id: &str) -> Result<crate::types::ApiCallWithPrice> {
        let url = format!(
            "/api-calls/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * List async operations.
    *
    * This function performs a `GET` to the `/async/operations` endpoint.
    *
    * For async file conversion operations, this endpoint does not return the contents of converted files (`output`). To get the contents use the `/async/operations/{id}` endpoint.
    * This endpoint requires authentication by a KittyCAD employee.
    *
    * **Parameters:**
    *
    * * `limit: u32` -- Maximum number of items returned by a single call.
    * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
    * * `sort_by: crate::types::CreatedAtSortMode` -- Supported set of sort modes for scanning by created_at only.
    *  
    *  Currently, we only support scanning in ascending order.
    * * `status: crate::types::ApiCallStatus` -- The status of an async API call.
    */
    pub async fn list_async_operations(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::CreatedAtSortMode,
        status: crate::types::ApiCallStatus,
    ) -> Result<Vec<crate::types::AsyncApiCall>> {
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
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/async/operations?{}", query_);

        let resp: crate::types::AsyncApiCallResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
    * List async operations.
    *
    * This function performs a `GET` to the `/async/operations` endpoint.
    *
    * As opposed to `list_async_operations`, this function returns all the pages of the request at once.
    *
    * For async file conversion operations, this endpoint does not return the contents of converted files (`output`). To get the contents use the `/async/operations/{id}` endpoint.
    * This endpoint requires authentication by a KittyCAD employee.
    */
    pub async fn list_all_async_operations(
        &self,
        sort_by: crate::types::CreatedAtSortMode,
        status: crate::types::ApiCallStatus,
    ) -> Result<Vec<crate::types::AsyncApiCall>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/async/operations?{}", query_);

        let mut resp: crate::types::AsyncApiCallResultsPage = self.client.get(&url, None).await?;

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
    * Get an async operation.
    *
    * This function performs a `GET` to the `/async/operations/{id}` endpoint.
    *
    * Get the status and output of an async operation.
    * This endpoint requires authentication by any KittyCAD user. It returns details of the requested async operation for the user.
    * If the user is not authenticated to view the specified async operation, then it is not returned.
    * Only KittyCAD employees with the proper access can view async operations for other users.
    *
    * **Parameters:**
    *
    * * `id: &str` -- The ID of the async operation.
    */
    pub async fn get_async_operation(&self, id: &str) -> Result<crate::types::AsyncApiCallOutput> {
        let url = format!(
            "/async/operations/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * List API calls for your user.
    *
    * This function performs a `GET` to the `/user/api-calls` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user.
    * The API calls are returned in order of creation, with the most recently created API calls first.
    *
    * **Parameters:**
    *
    * * `limit: u32` -- Maximum number of items returned by a single call.
    * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
    * * `sort_by: crate::types::CreatedAtSortMode` -- Supported set of sort modes for scanning by created_at only.
    *  
    *  Currently, we only support scanning in ascending order.
    */
    pub async fn user_list_calls(
        &self,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ApiCallWithPrice>> {
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
        let url = format!("/user/api-calls?{}", query_);

        let resp: crate::types::ApiCallWithPriceResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
    * List API calls for your user.
    *
    * This function performs a `GET` to the `/user/api-calls` endpoint.
    *
    * As opposed to `user_list_calls`, this function returns all the pages of the request at once.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user.
    * The API calls are returned in order of creation, with the most recently created API calls first.
    */
    pub async fn user_list_all_calls(
        &self,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ApiCallWithPrice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/user/api-calls?{}", query_);

        let mut resp: crate::types::ApiCallWithPriceResultsPage =
            self.client.get(&url, None).await?;

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
    * Get an API call for a user.
    *
    * This function performs a `GET` to the `/user/api-calls/{id}` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns details of the requested API call for the user.
    *
    * **Parameters:**
    *
    * * `id: &str` -- The ID of the API call.
    */
    pub async fn get_call_for_user(&self, id: &str) -> Result<crate::types::ApiCallWithPrice> {
        let url = format!(
            "/user/api-calls/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * List API calls for a user.
    *
    * This function performs a `GET` to the `/users/{id}/api-calls` endpoint.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user if "me" is passed as the user id.
    * Alternatively, you can use the `/user/api-calls` endpoint to get the API calls for your user.
    * If the authenticated user is a KittyCAD employee, then the API calls are returned for the user specified by the user id.
    * The API calls are returned in order of creation, with the most recently created API calls first.
    *
    * **Parameters:**
    *
    * * `id: &str` -- The user ID.
    * * `limit: u32` -- Maximum number of items returned by a single call.
    * * `page_token: &str` -- Token returned by previous call to retrieve the subsequent page.
    * * `sort_by: crate::types::CreatedAtSortMode` -- Supported set of sort modes for scanning by created_at only.
    *  
    *  Currently, we only support scanning in ascending order.
    */
    pub async fn list_calls_for_user(
        &self,
        id: &str,
        limit: u32,
        page_token: &str,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ApiCallWithPrice>> {
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
        let url = format!(
            "/users/{}/api-calls?{}",
            crate::progenitor_support::encode_path(&id.to_string()),
            query_
        );

        let resp: crate::types::ApiCallWithPriceResultsPage = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.items)
    }

    /**
    * List API calls for a user.
    *
    * This function performs a `GET` to the `/users/{id}/api-calls` endpoint.
    *
    * As opposed to `list_calls_for_user`, this function returns all the pages of the request at once.
    *
    * This endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user if "me" is passed as the user id.
    * Alternatively, you can use the `/user/api-calls` endpoint to get the API calls for your user.
    * If the authenticated user is a KittyCAD employee, then the API calls are returned for the user specified by the user id.
    * The API calls are returned in order of creation, with the most recently created API calls first.
    */
    pub async fn list_all_calls_for_user(
        &self,
        id: &str,
        sort_by: crate::types::CreatedAtSortMode,
    ) -> Result<Vec<crate::types::ApiCallWithPrice>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/api-calls?{}",
            crate::progenitor_support::encode_path(&id.to_string()),
            query_
        );

        let mut resp: crate::types::ApiCallWithPriceResultsPage =
            self.client.get(&url, None).await?;

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
}
