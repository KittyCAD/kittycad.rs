use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Projects {
    pub client: Client,
}

impl Projects {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List the active categories available for project submissions.\n\n**Parameters:**\n\n- \
             `limit: Option<u32>`: Maximum number of items returned by a single call\n- \
             `page_token: Option<String>`: Token returned by previous call to retrieve the \
             subsequent page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_projects_list_categories_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let \
             mut stream = projects.list_categories_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_categories<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::ProjectCategoryResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "projects/categories"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List the active categories available for project submissions.\n\n**Parameters:**\n\n- \
             `limit: Option<u32>`: Maximum number of items returned by a single call\n- \
             `page_token: Option<String>`: Token returned by previous call to retrieve the \
             subsequent page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_projects_list_categories_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let \
             mut stream = projects.list_categories_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_categories_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ProjectCategoryResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("projects/categories").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list_categories(limit, None)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| {
                        let pagination_url_path = pagination_url_path.clone();
                        let pagination_query_params = pagination_query_params.clone();
                        async move {
                            if new_result.has_more_pages()
                                && !new_result.items().is_empty()
                                && prev_page_token != new_result.next_page_token()
                            {
                                async {
                                    let mut req = self.client.client.request(
                                        http::Method::GET,
                                        format!(
                                            "{}/{}",
                                            self.client.base_url,
                                            pagination_url_path.clone()
                                        ),
                                    );
                                    req = req.bearer_auth(&self.client.token);
                                    let query_params = pagination_query_params.clone();
                                    req = req.query(&query_params);
                                    let mut request = req.build()?;
                                    request =
                                        new_result.next_page_with_param(request, "page_token")?;
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
                                        let text = resp.text().await.unwrap_or_default();
                                        Err(crate::types::error::Error::Server {
                                            body: text.to_string(),
                                            status,
                                        })
                                    }
                                }
                                .map_ok(
                                    |result: crate::types::ProjectCategoryResponseResultsPage| {
                                        Some((
                                            futures::stream::iter(
                                                result.items().into_iter().map(Ok),
                                            ),
                                            (new_result.next_page_token(), result),
                                        ))
                                    },
                                )
                                .await
                            } else {
                                Ok(None)
                            }
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream();
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
        }
    }

    #[doc = "List publicly visible community projects for the \
             website/gallery.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of \
             items returned by a single call\n- `page_token: Option<String>`: Token returned by \
             previous call to retrieve the subsequent page\n\n```rust,no_run\nuse \
             futures_util::TryStreamExt;\nasync fn example_projects_list_public_stream() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut \
             projects = client.projects();\n    let mut stream = \
             projects.list_public_stream(Some(4 as u32));\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_public<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::PublicProjectResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "projects/public"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List publicly visible community projects for the \
             website/gallery.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of \
             items returned by a single call\n- `page_token: Option<String>`: Token returned by \
             previous call to retrieve the subsequent page\n\n```rust,no_run\nuse \
             futures_util::TryStreamExt;\nasync fn example_projects_list_public_stream() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut \
             projects = client.projects();\n    let mut stream = \
             projects.list_public_stream(Some(4 as u32));\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_public_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::PublicProjectResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("projects/public").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list_public(limit, None)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| {
                        let pagination_url_path = pagination_url_path.clone();
                        let pagination_query_params = pagination_query_params.clone();
                        async move {
                            if new_result.has_more_pages()
                                && !new_result.items().is_empty()
                                && prev_page_token != new_result.next_page_token()
                            {
                                async {
                                    let mut req = self.client.client.request(
                                        http::Method::GET,
                                        format!(
                                            "{}/{}",
                                            self.client.base_url,
                                            pagination_url_path.clone()
                                        ),
                                    );
                                    req = req.bearer_auth(&self.client.token);
                                    let query_params = pagination_query_params.clone();
                                    req = req.query(&query_params);
                                    let mut request = req.build()?;
                                    request =
                                        new_result.next_page_with_param(request, "page_token")?;
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
                                        let text = resp.text().await.unwrap_or_default();
                                        Err(crate::types::error::Error::Server {
                                            body: text.to_string(),
                                            status,
                                        })
                                    }
                                }
                                .map_ok(|result: crate::types::PublicProjectResponseResultsPage| {
                                    Some((
                                        futures::stream::iter(result.items().into_iter().map(Ok)),
                                        (new_result.next_page_token(), result),
                                    ))
                                })
                                .await
                            } else {
                                Ok(None)
                            }
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream();
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
        }
    }

    #[doc = "Get one publicly visible community project.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: \
             The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_get_public() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::PublicProjectResponse = client\n        .projects()\n        \
             .get_public(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_public<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::PublicProjectResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "projects/public/{id}".replace("{id}", &format!("{}", id))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Download a published public project as a tar archive.\n\n**Parameters:**\n\n- `format: Option<crate::types::ProjectArchiveFormat>`: Archive format to return. Defaults to `tar`.\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_download_public() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .projects()\n        .download_public(\n            Some(kittycad::types::ProjectArchiveFormat::Zip),\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn download_public<'a>(
        &'a self,
        format: Option<crate::types::ProjectArchiveFormat>,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "projects/public/{id}/download".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = format {
            query_params.push(("format", format!("{}", p)));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Fetch the public thumbnail for a published project.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_get_public_thumbnail() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .projects()\n        .get_public_thumbnail(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_public_thumbnail<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "projects/public/{id}/thumbnail".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Add the authenticated user's upvote to a published community \
             project.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_create_public_vote() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::PublicProjectVoteResponse = client\n        .projects()\n        \
             .create_public_vote(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_public_vote<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::PublicProjectVoteResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "projects/public/{id}/vote".replace("{id}", &format!("{}", id))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Remove the authenticated user's upvote from a published community \
             project.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_delete_public_vote() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::PublicProjectVoteResponse = client\n        .projects()\n        \
             .delete_public_vote(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_public_vote<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::PublicProjectVoteResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "projects/public/{id}/vote".replace("{id}", &format!("{}", id))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List the authenticated user's projects.\n\n**Parameters:**\n\n- `limit: Option<u32>`: \
             Maximum number of items returned by a single call\n- `page_token: Option<String>`: \
             Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_projects_list_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let \
             mut stream = projects.list_stream(Some(4 as u32));\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::ProjectSummaryResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/projects"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List the authenticated user's projects.\n\n**Parameters:**\n\n- `limit: Option<u32>`: \
             Maximum number of items returned by a single call\n- `page_token: Option<String>`: \
             Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_projects_list_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let \
             mut stream = projects.list_stream(Some(4 as u32));\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ProjectSummaryResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("user/projects").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list(limit, None)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| {
                        let pagination_url_path = pagination_url_path.clone();
                        let pagination_query_params = pagination_query_params.clone();
                        async move {
                            if new_result.has_more_pages()
                                && !new_result.items().is_empty()
                                && prev_page_token != new_result.next_page_token()
                            {
                                async {
                                    let mut req = self.client.client.request(
                                        http::Method::GET,
                                        format!(
                                            "{}/{}",
                                            self.client.base_url,
                                            pagination_url_path.clone()
                                        ),
                                    );
                                    req = req.bearer_auth(&self.client.token);
                                    let query_params = pagination_query_params.clone();
                                    req = req.query(&query_params);
                                    let mut request = req.build()?;
                                    request =
                                        new_result.next_page_with_param(request, "page_token")?;
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
                                        let text = resp.text().await.unwrap_or_default();
                                        Err(crate::types::error::Error::Server {
                                            body: text.to_string(),
                                            status,
                                        })
                                    }
                                }
                                .map_ok(
                                    |result: crate::types::ProjectSummaryResponseResultsPage| {
                                        Some((
                                            futures::stream::iter(
                                                result.items().into_iter().map(Ok),
                                            ),
                                            (new_result.next_page_token(), result),
                                        ))
                                    },
                                )
                                .await
                            } else {
                                Ok(None)
                            }
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream();
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
        }
    }

    #[doc = "Create a draft project for the authenticated user.\n\n```rust,no_run\nasync fn example_projects_create() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectResponse = client\n        .projects()\n        .create(vec![kittycad::types::multipart::Attachment {\n            name: \"thing\".to_string(),\n            filepath: Some(\"myfile.json\".into()),\n            content_type: Some(\"application/json\".to_string()),\n            data: std::fs::read(\"myfile.json\").unwrap(),\n        }])\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
    ) -> Result<crate::types::ProjectResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/projects"),
        );
        req = req.bearer_auth(&self.client.token);
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get one of the authenticated user's projects.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_get() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectResponse = client\n        .projects()\n        .get(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::ProjectResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}".replace("{id}", &format!("{}", id))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Replace one of the authenticated user's projects.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_update() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectResponse = client\n        .projects()\n        .update(\n            vec![kittycad::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        id: uuid::Uuid,
    ) -> Result<crate::types::ProjectResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Delete one of the authenticated user's projects.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_delete() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .projects()\n        .delete(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(&'a self, id: uuid::Uuid) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Download one of the authenticated user's projects as a tar archive.\n\n**Parameters:**\n\n- `format: Option<crate::types::ProjectArchiveFormat>`: Archive format to return. Defaults to `tar`.\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_download() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .projects()\n        .download(\n            Some(kittycad::types::ProjectArchiveFormat::Zip),\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn download<'a>(
        &'a self,
        format: Option<crate::types::ProjectArchiveFormat>,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/download".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = format {
            query_params.push(("format", format!("{}", p)));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Move one of the authenticated user's projects into their active organization library.\n\nThis changes only the project's ownership scope. The project ID, current revision, files, and version history remain unchanged so cloud bindings stay valid across the move.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_update_organization() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectResponse = client\n        .projects()\n        .update_organization(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_organization<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::ProjectResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/organization".replace("{id}", &format!("{}", id))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Move an organization project back to its creator's personal library.\n\nOrganization \
             administrators may perform this move to revoke organization access. The project ID, \
             current revision, files, and version history remain \
             unchanged.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_delete_organization() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .projects()\n        \
             .delete_organization(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_organization<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/organization".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Submit one of the authenticated user's projects for public \
             review.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_publish() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectResponse = \
             client\n        .projects()\n        .publish(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn publish<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::ProjectResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/publish".replace("{id}", &format!("{}", id))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List share links for one of the authenticated user's projects.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_list_share_links() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectShareLinkResponseResultsPage = client\n        .projects()\n        .list_share_links(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_share_links_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_share_links_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_share_links<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::ProjectShareLinkResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/share-links".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List share links for one of the authenticated user's projects.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_list_share_links() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectShareLinkResponseResultsPage = client\n        .projects()\n        .list_share_links(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_share_links_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_share_links_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_share_links_stream<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ProjectShareLinkResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path =
            ("user/projects/{id}/share-links".replace("{id}", &format!("{}", id))).to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list_share_links(id, limit, None)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| {
                        let pagination_url_path = pagination_url_path.clone();
                        let pagination_query_params = pagination_query_params.clone();
                        async move {
                            if new_result.has_more_pages()
                                && !new_result.items().is_empty()
                                && prev_page_token != new_result.next_page_token()
                            {
                                async {
                                    let mut req = self.client.client.request(
                                        http::Method::GET,
                                        format!(
                                            "{}/{}",
                                            self.client.base_url,
                                            pagination_url_path.clone()
                                        ),
                                    );
                                    req = req.bearer_auth(&self.client.token);
                                    let query_params = pagination_query_params.clone();
                                    req = req.query(&query_params);
                                    let mut request = req.build()?;
                                    request =
                                        new_result.next_page_with_param(request, "page_token")?;
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
                                        let text = resp.text().await.unwrap_or_default();
                                        Err(crate::types::error::Error::Server {
                                            body: text.to_string(),
                                            status,
                                        })
                                    }
                                }
                                .map_ok(
                                    |result: crate::types::ProjectShareLinkResponseResultsPage| {
                                        Some((
                                            futures::stream::iter(
                                                result.items().into_iter().map(Ok),
                                            ),
                                            (new_result.next_page_token(), result),
                                        ))
                                    },
                                )
                                .await
                            } else {
                                Ok(None)
                            }
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream();
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
        }
    }

    #[doc = "Create a share link for one of the authenticated user's \
             projects.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_create_share_link() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::ProjectShareLinkResponse = client\n        .projects()\n        \
             .create_share_link(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             &kittycad::types::CreateProjectShareLinkRequest {\n                access_mode: \
             Some(kittycad::types::KclProjectShareLinkAccessMode::OrganizationOnly),\n            \
             },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_share_link<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::CreateProjectShareLinkRequest,
    ) -> Result<crate::types::ProjectShareLinkResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/share-links".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Delete one share link for one of the authenticated user's \
             projects.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: Project identifier. (required)\n- \
             `key: &'astr`: Share-link key. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_projects_delete_share_link() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .projects()\n        .delete_share_link(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             \"some-string\",\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_share_link<'a>(
        &'a self,
        id: uuid::Uuid,
        key: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/share-links/{key}"
                    .replace("{id}", &format!("{}", id))
                    .replace("{key}", key)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Fetch the authenticated owner's current project thumbnail.\n\n**Parameters:**\n\n- \
             `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_projects_get_thumbnail() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    client\n        \
             .projects()\n        .get_thumbnail(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_thumbnail<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/thumbnail".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List a project's saved versions, newest first.\n\nRequires access to the project. Public visibility or link sharing will not grant access to history.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_list_versions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectVersionSummaryResponseResultsPage = client\n        .projects()\n        .list_versions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_versions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_versions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_versions<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::ProjectVersionSummaryResponseResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/versions".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List a project's saved versions, newest first.\n\nRequires access to the project. Public visibility or link sharing will not grant access to history.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_list_versions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectVersionSummaryResponseResultsPage = client\n        .projects()\n        .list_versions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_versions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_versions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_versions_stream<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ProjectVersionSummaryResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path =
            ("user/projects/{id}/versions".replace("{id}", &format!("{}", id))).to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self . list_versions (id , limit , None) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold ((None , result) , move | (prev_page_token , new_result) | { let pagination_url_path = pagination_url_path . clone () ; let pagination_query_params = pagination_query_params . clone () ; async move { if new_result . has_more_pages () && ! new_result . items () . is_empty () && prev_page_token != new_result . next_page_token () { async { let mut req = self . client . client . request (http :: Method :: GET , format ! ("{}/{}" , self . client . base_url , pagination_url_path . clone ()) ,) ; req = req . bearer_auth (& self . client . token) ; let query_params = pagination_query_params . clone () ; req = req . query (& query_params) ; let mut request = req . build () ? ; request = new_result . next_page_with_param (request , "page_token") ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (format_serde_error :: SerdeError :: new (text . to_string () , err) , status)) } else { let text = resp . text () . await . unwrap_or_default () ; Err (crate :: types :: error :: Error :: Server { body : text . to_string () , status }) } } . map_ok (| result : crate :: types :: ProjectVersionSummaryResponseResultsPage | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , (new_result . next_page_token () , result) ,)) }) . await } else { Ok (None) } } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () ;
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
        }
    }

    #[doc = "Save an alternate project version without changing the current version.\n\nFor a history A -> B -> C with C current, saving D with B as its parent creates a second child of B. C stays current. Publications and share links keep pointing to their existing versions.\n\nSend a multipart request with a JSON `body` part and file parts. Upload the complete replacement snapshot, including unchanged files. Each uploaded filename must be its relative project path.\n\nExample JSON for the `body` part (replace the parent placeholder with B's UUID):\n\n```ignorejson {   \"parent_version_id\": \"<B_VERSION_ID>\",   \"title\": \"Alternative design\",   \"description\": \"Trying another shape\",   \"entrypoint_path\": \"main.kcl\",   \"deleted_paths\": [\"obsolete.kcl\"] } ```ignore\n\n`parent_version_id` and `title` are required. Description defaults to an empty string, and the entrypoint defaults to `main.kcl`. When supplying `deleted_paths`, list all files removed from the chosen parent B, regardless of the files in current C. An empty list declares that no files were removed; omitting the field skips this deletion-intent check.\n\nSave the JSON as `save-metadata.json`. With D's files in the working directory, set `API_BASE_URL`, `API_TOKEN`, and `PROJECT_ID`, then generate `SAVE_KEY` once for this save (for example, using `uuidgen`):\n\n```ignoresh curl --fail-with-body \\   --request POST \"${API_BASE_URL}/user/projects/${PROJECT_ID}/versions\" \\   --header \"Authorization: Bearer ${API_TOKEN}\" \\   --header \"Idempotency-Key: ${SAVE_KEY}\" \\   --form 'body=<save-metadata.json;type=application/json' \\   --form 'file-0=@project.toml;filename=project.toml' \\   --form 'file-1=@main.kcl;filename=main.kcl' \\   --form 'file-2=@part.kcl;filename=part.kcl' ```ignore\n\nThe HTTP 200 response contains `version_id` (D) and `current_version_id` (C, or the current version when the response is prepared). Read D through `GET /user/projects/{id}/versions/{version_id}` and download it through `GET /user/projects/{id}/versions/{version_id}/download`. Downloads default to TAR; use `?format=zip` for ZIP.\n\n`Idempotency-Key` is optional for all clients. Use a unique key for each save to avoid duplicate versions when retrying. Retain the key, metadata, and submitted file contents across app restarts until the save's outcome is known. Within 24 hours of a successful save, retrying with the same key and contents returns the same version. Changed contents require a new key; reusing an unexpired key with different contents returns HTTP 409 with `IdempotencyConflict`. Without a key, or after its window expires, resending the request can create another version.\n\nWrite access to the project is required, including for retries. A public listing or share link does not grant access to private version history. There is no endpoint to promote an existing alternate version directly to current.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_create_version() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CreateProjectVersionResponse = client\n        .projects()\n        .create_version(\n            vec![kittycad::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_version<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        id: uuid::Uuid,
    ) -> Result<crate::types::CreateProjectVersionResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/versions".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get metadata and files for a single saved project version.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: Project identifier. (required)\n- `version_id: uuid::Uuid`: Requested version identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_get_version() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectVersionDetailResponse = client\n        .projects()\n        .get_version(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_version<'a>(
        &'a self,
        id: uuid::Uuid,
        version_id: uuid::Uuid,
    ) -> Result<crate::types::ProjectVersionDetailResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/versions/{version_id}"
                    .replace("{id}", &format!("{}", id))
                    .replace("{version_id}", &format!("{}", version_id))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Download the files saved in one project version.\n\n**Parameters:**\n\n- `format: \
             Option<crate::types::ProjectArchiveFormat>`: Archive format to return. Defaults to \
             `tar`.\n- `id: uuid::Uuid`: Project identifier. (required)\n- `version_id: \
             uuid::Uuid`: Requested version identifier. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_projects_download_version() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .projects()\n        .download_version(\n            \
             Some(kittycad::types::ProjectArchiveFormat::Zip),\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn download_version<'a>(
        &'a self,
        format: Option<crate::types::ProjectArchiveFormat>,
        id: uuid::Uuid,
        version_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/versions/{version_id}/download"
                    .replace("{id}", &format!("{}", id))
                    .replace("{version_id}", &format!("{}", version_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = format {
            query_params.push(("format", format!("{}", p)));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Fetch the thumbnail for a single saved project version.\n\n**Parameters:**\n\n- `id: \
             uuid::Uuid`: Project identifier. (required)\n- `version_id: uuid::Uuid`: Requested \
             version identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_get_version_thumbnail() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .projects()\n        \
             .get_version_thumbnail(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_version_thumbnail<'a>(
        &'a self,
        id: uuid::Uuid,
        version_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/versions/{version_id}/thumbnail"
                    .replace("{id}", &format!("{}", id))
                    .replace("{version_id}", &format!("{}", version_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
