use crate::Client;
use anyhow::Result;
pub struct ApiCalls {
    pub client: Client,
}

impl ApiCalls {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get API call metrics.\n\nThis endpoint requires authentication by a KittyCAD employee. The API calls are grouped by the parameter passed."]
    pub async fn get_api_call_metrics(
        &self,
        group_by: crate::types::ApiCallQueryGroupBy,
    ) -> Result<Vec<crate::types::ApiCallQueryGroup>> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "api-call-metrics"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("group_by", format!("{}", group_by)));
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

    #[doc = "List API calls.\n\nThis endpoint requires authentication by a KittyCAD employee. The API calls are returned in order of creation, with the most recently created API calls first."]
    pub async fn list(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiCallWithPriceResultsPage> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "api-calls"),
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

    #[doc = "List API calls.\n\nThis endpoint requires authentication by a KittyCAD employee. The API calls are returned in order of creation, with the most recently created API calls first."]
    pub fn list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::ApiCallWithPrice>> + Unpin + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages()? {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    &format!("{}/{}", self.client.base_url, "api-calls"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                let text = resp.text().await.unwrap_or_default();
                                if status.is_success() {
                                    serde_json::from_str(&text).map_err(|err| {
                                        format_serde_error::SerdeError::new(text.to_string(), err)
                                            .into()
                                    })
                                } else {
                                    Err(anyhow::anyhow!(
                                        "response was not successful `{}` -> `{}`",
                                        status,
                                        text
                                    ))
                                }
                            }
                            .map_ok(|result: crate::types::ApiCallWithPriceResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    result,
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    })
                    .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Get details of an API call.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API call for the user.\nIf the user is not authenticated to view the specified API call, then it is not returned.\nOnly KittyCAD employees can view API calls for other users."]
    pub async fn get_api_call(&self, id: &str) -> Result<crate::types::ApiCallWithPrice> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "api-calls/{id}".replace("{id}", &id)
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

    #[doc = "List async operations.\n\nFor async file conversion operations, this endpoint does not return the contents of converted files (`output`). To get the contents use the `/async/operations/{id}` endpoint.\nThis endpoint requires authentication by a KittyCAD employee."]
    pub async fn list_async_operations(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
        status: Option<crate::types::ApiCallStatus>,
    ) -> Result<crate::types::AsyncApiCallResultsPage> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "async/operations"),
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

        if let Some(p) = status {
            query_params.push(("status", format!("{}", p)));
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

    #[doc = "List async operations.\n\nFor async file conversion operations, this endpoint does not return the contents of converted files (`output`). To get the contents use the `/async/operations/{id}` endpoint.\nThis endpoint requires authentication by a KittyCAD employee."]
    pub fn list_async_operations_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
        status: Option<crate::types::ApiCallStatus>,
    ) -> impl futures::Stream<Item = Result<crate::types::AsyncApiCall>> + Unpin + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list_async_operations(limit, None, sort_by, status)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages()? {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    &format!("{}/{}", self.client.base_url, "async/operations"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                let text = resp.text().await.unwrap_or_default();
                                if status.is_success() {
                                    serde_json::from_str(&text).map_err(|err| {
                                        format_serde_error::SerdeError::new(text.to_string(), err)
                                            .into()
                                    })
                                } else {
                                    Err(anyhow::anyhow!(
                                        "response was not successful `{}` -> `{}`",
                                        status,
                                        text
                                    ))
                                }
                            }
                            .map_ok(|result: crate::types::AsyncApiCallResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    result,
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    })
                    .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Get an async operation.\n\nGet the status and output of an async operation.\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested async operation for the user.\nIf the user is not authenticated to view the specified async operation, then it is not returned.\nOnly KittyCAD employees with the proper access can view async operations for other users."]
    pub async fn get_async_operation(&self, id: &str) -> Result<crate::types::AsyncApiCallOutput> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "async/operations/{id}".replace("{id}", &id)
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

    #[doc = "List API calls for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user.\nThe API calls are returned in order of creation, with the most recently created API calls first."]
    pub async fn user_list(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiCallWithPriceResultsPage> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/api-calls"),
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

    #[doc = "List API calls for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user.\nThe API calls are returned in order of creation, with the most recently created API calls first."]
    pub fn user_list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::ApiCallWithPrice>> + Unpin + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.user_list(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages()? {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    &format!("{}/{}", self.client.base_url, "user/api-calls"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                let text = resp.text().await.unwrap_or_default();
                                if status.is_success() {
                                    serde_json::from_str(&text).map_err(|err| {
                                        format_serde_error::SerdeError::new(text.to_string(), err)
                                            .into()
                                    })
                                } else {
                                    Err(anyhow::anyhow!(
                                        "response was not successful `{}` -> `{}`",
                                        status,
                                        text
                                    ))
                                }
                            }
                            .map_ok(|result: crate::types::ApiCallWithPriceResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    result,
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    })
                    .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Get an API call for a user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API call for the user."]
    pub async fn get_api_call_for_user(&self, id: &str) -> Result<crate::types::ApiCallWithPrice> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/api-calls/{id}".replace("{id}", &id)
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

    #[doc = "List API calls for a user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user if \"me\" is passed as the user id.\nAlternatively, you can use the `/user/api-calls` endpoint to get the API calls for your user.\nIf the authenticated user is a KittyCAD employee, then the API calls are returned for the user specified by the user id.\nThe API calls are returned in order of creation, with the most recently created API calls first."]
    pub async fn list_for_user(
        &self,
        id: &str,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiCallWithPriceResultsPage> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}/api-calls".replace("{id}", &id)
            ),
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

    #[doc = "List API calls for a user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user if \"me\" is passed as the user id.\nAlternatively, you can use the `/user/api-calls` endpoint to get the API calls for your user.\nIf the authenticated user is a KittyCAD employee, then the API calls are returned for the user specified by the user id.\nThe API calls are returned in order of creation, with the most recently created API calls first."]
    pub fn list_for_user_stream<'a>(
        &'a self,
        id: &'a str,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::ApiCallWithPrice>> + Unpin + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list_for_user(id, limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages()? {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    &format!(
                                        "{}/{}",
                                        self.client.base_url,
                                        "users/{id}/api-calls".replace("{id}", &id)
                                    ),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                let text = resp.text().await.unwrap_or_default();
                                if status.is_success() {
                                    serde_json::from_str(&text).map_err(|err| {
                                        format_serde_error::SerdeError::new(text.to_string(), err)
                                            .into()
                                    })
                                } else {
                                    Err(anyhow::anyhow!(
                                        "response was not successful `{}` -> `{}`",
                                        status,
                                        text
                                    ))
                                }
                            }
                            .map_ok(|result: crate::types::ApiCallWithPriceResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    result,
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    })
                    .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }
}
