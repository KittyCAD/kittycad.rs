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

    #[doc = "Get API call metrics.\n\nThis endpoint requires authentication by a KittyCAD employee. The API calls are grouped by the parameter passed.\n\n```\nasync fn example_api_calls_get_api_call_metrics() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::ApiCallQueryGroup> = client\n        .api_calls()\n        .get_api_call_metrics(kittycad::types::ApiCallQueryGroupBy::UserId)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_api_call_metrics<'a>(
        &'a self,
        group_by: crate::types::ApiCallQueryGroupBy,
    ) -> Result<Vec<crate::types::ApiCallQueryGroup>, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List API calls.\n\nThis endpoint requires authentication by a KittyCAD employee. The API calls are returned in order of creation, with the most recently created API calls first.\n\n```\nasync fn example_api_calls_list() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiCallWithPriceResultsPage = client\n        .api_calls()\n        .list(\n            Some(4 as u32),\n            Some(\"mtutrk\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::stream::try_stream::TryStreamExt;\nasync fn example_api_calls_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let stream = client.api_calls().list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub async fn list<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiCallWithPriceResultsPage, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List API calls.\n\nThis endpoint requires authentication by a KittyCAD employee. The API calls are returned in order of creation, with the most recently created API calls first.\n\n```\nasync fn example_api_calls_list() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiCallWithPriceResultsPage = client\n        .api_calls()\n        .list(\n            Some(4 as u32),\n            Some(\"mtutrk\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::stream::try_stream::TryStreamExt;\nasync fn example_api_calls_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let stream = client.api_calls().list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub fn list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ApiCallWithPrice, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages() {
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
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
                                            status,
                                        )
                                    })
                                } else {
                                    Err(crate::types::error::Error::UnexpectedResponse(resp))
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

    #[doc = "Get details of an API call.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API call for the user.\nIf the user is not authenticated to view the specified API call, then it is not returned.\nOnly KittyCAD employees can view API calls for other users.\n\n```\nasync fn example_api_calls_get_api_call() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiCallWithPrice = client.api_calls().get_api_call(\"\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_api_call<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::ApiCallWithPrice, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "api-calls/{id}".replace("{id}", id)
            ),
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
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List async operations.\n\nFor async file conversion operations, this endpoint does not return the contents of converted files (`output`). To get the contents use the `/async/operations/{id}` endpoint.\nThis endpoint requires authentication by a KittyCAD employee.\n\n```\nasync fn example_api_calls_list_async_operations() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::AsyncApiCallResultsPage = client\n        .api_calls()\n        .list_async_operations(\n            Some(4 as u32),\n            Some(\"\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n            Some(kittycad::types::ApiCallStatus::Completed),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::stream::try_stream::TryStreamExt;\nasync fn example_api_calls_list_async_operations_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let stream = client.api_calls().list_async_operations_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n        Some(kittycad::types::ApiCallStatus::Completed),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub async fn list_async_operations<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
        status: Option<crate::types::ApiCallStatus>,
    ) -> Result<crate::types::AsyncApiCallResultsPage, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List async operations.\n\nFor async file conversion operations, this endpoint does not return the contents of converted files (`output`). To get the contents use the `/async/operations/{id}` endpoint.\nThis endpoint requires authentication by a KittyCAD employee.\n\n```\nasync fn example_api_calls_list_async_operations() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::AsyncApiCallResultsPage = client\n        .api_calls()\n        .list_async_operations(\n            Some(4 as u32),\n            Some(\"\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n            Some(kittycad::types::ApiCallStatus::Completed),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::stream::try_stream::TryStreamExt;\nasync fn example_api_calls_list_async_operations_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let stream = client.api_calls().list_async_operations_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n        Some(kittycad::types::ApiCallStatus::Completed),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub fn list_async_operations_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
        status: Option<crate::types::ApiCallStatus>,
    ) -> impl futures::Stream<Item = Result<crate::types::AsyncApiCall, crate::types::error::Error>>
           + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list_async_operations(limit, None, sort_by, status)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages() {
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
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
                                            status,
                                        )
                                    })
                                } else {
                                    Err(crate::types::error::Error::UnexpectedResponse(resp))
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

    #[doc = "Get an async operation.\n\nGet the status and output of an async operation.\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested async operation for the user.\nIf the user is not authenticated to view the specified async operation, then it is not returned.\nOnly KittyCAD employees with the proper access can view async operations for other users.\n\n```\nasync fn example_api_calls_get_async_operation() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::AsyncApiCallOutput =\n        client.api_calls().get_async_operation(\"eeogbvfhp\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_async_operation<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::AsyncApiCallOutput, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "async/operations/{id}".replace("{id}", id)
            ),
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
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List API calls for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user.\nThe API calls are returned in order of creation, with the most recently created API calls first.\n\n```\nasync fn example_api_calls_user_list() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiCallWithPriceResultsPage = client\n        .api_calls()\n        .user_list(\n            Some(4 as u32),\n            Some(\"pdeuvuvwe\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::stream::try_stream::TryStreamExt;\nasync fn example_api_calls_user_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let stream = client.api_calls().user_list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub async fn user_list<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiCallWithPriceResultsPage, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List API calls for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user.\nThe API calls are returned in order of creation, with the most recently created API calls first.\n\n```\nasync fn example_api_calls_user_list() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiCallWithPriceResultsPage = client\n        .api_calls()\n        .user_list(\n            Some(4 as u32),\n            Some(\"pdeuvuvwe\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::stream::try_stream::TryStreamExt;\nasync fn example_api_calls_user_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let stream = client.api_calls().user_list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub fn user_list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ApiCallWithPrice, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.user_list(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages() {
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
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
                                            status,
                                        )
                                    })
                                } else {
                                    Err(crate::types::error::Error::UnexpectedResponse(resp))
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

    #[doc = "Get an API call for a user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API call for the user.\n\n```\nasync fn example_api_calls_get_api_call_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiCallWithPrice =\n        client.api_calls().get_api_call_for_user(\"c\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_api_call_for_user<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::ApiCallWithPrice, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/api-calls/{id}".replace("{id}", id)
            ),
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
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List API calls for a user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user if \"me\" is passed as the user id.\nAlternatively, you can use the `/user/api-calls` endpoint to get the API calls for your user.\nIf the authenticated user is a KittyCAD employee, then the API calls are returned for the user specified by the user id.\nThe API calls are returned in order of creation, with the most recently created API calls first.\n\n```\nasync fn example_api_calls_list_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiCallWithPriceResultsPage = client\n        .api_calls()\n        .list_for_user(\n            \"akryyxsy\",\n            Some(4 as u32),\n            Some(\"uq\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::stream::try_stream::TryStreamExt;\nasync fn example_api_calls_list_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let stream = client.api_calls().list_for_user_stream(\n        \"akryyxsy\",\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub async fn list_for_user<'a>(
        &'a self,
        id: &'a str,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiCallWithPriceResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}/api-calls".replace("{id}", id)
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List API calls for a user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user if \"me\" is passed as the user id.\nAlternatively, you can use the `/user/api-calls` endpoint to get the API calls for your user.\nIf the authenticated user is a KittyCAD employee, then the API calls are returned for the user specified by the user id.\nThe API calls are returned in order of creation, with the most recently created API calls first.\n\n```\nasync fn example_api_calls_list_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiCallWithPriceResultsPage = client\n        .api_calls()\n        .list_for_user(\n            \"akryyxsy\",\n            Some(4 as u32),\n            Some(\"uq\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::stream::try_stream::TryStreamExt;\nasync fn example_api_calls_list_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let stream = client.api_calls().list_for_user_stream(\n        \"akryyxsy\",\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtAscending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub fn list_for_user_stream<'a>(
        &'a self,
        id: &'a str,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ApiCallWithPrice, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list_for_user(id, limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages() {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    &format!(
                                        "{}/{}",
                                        self.client.base_url,
                                        "users/{id}/api-calls".replace("{id}", id)
                                    ),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
                                            status,
                                        )
                                    })
                                } else {
                                    Err(crate::types::error::Error::UnexpectedResponse(resp))
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
