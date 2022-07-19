use crate::Client;
use anyhow::Result;
pub struct ApiTokens {
    pub client: Client,
}

impl ApiTokens {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List API tokens for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API tokens for the authenticated user.\nThe API tokens are returned in order of creation, with the most recently created API tokens first.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_api_tokens_list_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut api_tokens = client.api_tokens();\n    let mut stream = api_tokens.list_for_user_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    pub async fn list_for_user<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ApiTokenResultsPage, crate::types::error::Error> {
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

    #[doc = "List API tokens for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API tokens for the authenticated user.\nThe API tokens are returned in order of creation, with the most recently created API tokens first.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_api_tokens_list_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut api_tokens = client.api_tokens();\n    let mut stream = api_tokens.list_for_user_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    pub fn list_for_user_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::ApiToken, crate::types::error::Error>>
           + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list_for_user(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages() {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    &format!("{}/{}", self.client.base_url, "user/api-tokens"),
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
                            .map_ok(|result: crate::types::ApiTokenResultsPage| {
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

    #[doc = "Create a new API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new API token for the authenticated user.\n\n```rust,no_run\nasync fn example_api_tokens_create_api_token_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiToken = client.api_tokens().create_api_token_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    pub async fn create_api_token_for_user<'a>(
        &'a self,
    ) -> Result<crate::types::ApiToken, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "user/api-tokens"),
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

    #[doc = "Get an API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_api_tokens_get_api_token_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ApiToken = client\n        .api_tokens()\n        .get_api_token_for_user(uuid::Uuid::from_str(\n            \"17dafc3d-6e3c-4aad-81d1-e6cb47fc69ca\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    pub async fn get_api_token_for_user<'a>(
        &'a self,
        token: uuid::Uuid,
    ) -> Result<crate::types::ApiToken, crate::types::error::Error> {
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

    #[doc = "Delete an API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the requested API token for the user.\nThis endpoint does not actually delete the API token from the database. It merely marks the token as invalid. We still want to keep the token in the database for historical purposes.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_api_tokens_delete_api_token_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .api_tokens()\n        .delete_api_token_for_user(uuid::Uuid::from_str(\n            \"3be43f9a-bfd6-41ad-ac2e-3203c36ce805\",\n        )?)\n        .await?;\n    Ok(())\n}\n```"]
    pub async fn delete_api_token_for_user<'a>(
        &'a self,
        token: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
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
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
