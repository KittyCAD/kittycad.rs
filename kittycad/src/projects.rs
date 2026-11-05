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

    #[doc = "Search current projects shared with the authenticated user's organization.\n\nResults are ranked by similarity to bounded project metadata: title, description, and a capped list of source filenames. Project source contents are not indexed. Projects become searchable asynchronously after saving or changing organization ownership. Personal projects and obsolete versions are excluded. Pass `library_id` to search one named organization library.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_search_org_semantic() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::ProjectSemanticSearchMatch> = client\n        .projects()\n        .search_org_semantic(&kittycad::types::ProjectSemanticSearchRequest {\n            library_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            limit: Some(4 as u32),\n            q: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn search_org_semantic<'a>(
        &'a self,
        body: &crate::types::ProjectSemanticSearchRequest,
    ) -> Result<Vec<crate::types::ProjectSemanticSearchMatch>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "org/projects/search/semantic"
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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
                                                crate::types::error::SerdeError::new(
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

    #[doc = "List publicly visible community projects for the website/gallery.\n\n**Parameters:**\n\n- `category_id: Option<uuid::Uuid>`: Return only projects assigned to this category. Omit to return all categories.\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_list_public() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::PublicProjectResponseResultsPage = client\n        .projects()\n        .list_public(\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_public_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_public_stream(\n        Some(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?),\n        Some(4 as u32),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_public<'a>(
        &'a self,
        category_id: Option<uuid::Uuid>,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::PublicProjectResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "projects/public"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = category_id {
            query_params.push(("category_id", format!("{}", p)));
        }

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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "List publicly visible community projects for the website/gallery.\n\n**Parameters:**\n\n- `category_id: Option<uuid::Uuid>`: Return only projects assigned to this category. Omit to return all categories.\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_list_public() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::PublicProjectResponseResultsPage = client\n        .projects()\n        .list_public(\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_public_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_public_stream(\n        Some(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?),\n        Some(4 as u32),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_public_stream<'a>(
        &'a self,
        category_id: Option<uuid::Uuid>,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::PublicProjectResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("projects/public").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = category_id.as_ref() {
            pagination_query_params.push(("category_id", format!("{}", p)));
        }

        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list_public(category_id, limit, None)
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
                                                crate::types::error::SerdeError::new(
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Download all or selected files from a published public \
             project.\n\n**Parameters:**\n\n- `file_ids: Option<Vec<uuid::Uuid>>`: Optional \
             comma-separated file IDs from this download's version. Omit to download every file. \
             An empty or malformed list returns HTTP 400 with `error_code: InvalidProjectFileIds`. \
             Any ID outside this version returns HTTP 400 with `error_code: \
             InvalidProjectFileSelection` and a `missing_file_ids` array listing the requested IDs \
             absent from this version. The entire selection fails: requesting ten IDs with one \
             missing returns HTTP 400 without an archive. Missing projects or versions still \
             return HTTP 404. Repeated IDs are included once. Selecting one file still returns the \
             requested archive format.\n- `format: Option<crate::types::ProjectArchiveFormat>`: \
             Archive format to return. Defaults to `tar`.\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_download_public() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .projects()\n        \
             .download_public(\n            Some(vec![uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?]),\n            \
             Some(kittycad::types::ProjectArchiveFormat::Zip),\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn download_public<'a>(
        &'a self,
        file_ids: Option<Vec<uuid::Uuid>>,
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
        if let Some(p) = file_ids {
            query_params.push(("file_ids", itertools::join(p, ",")));
        }

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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "List personal libraries and libraries belonging to the active \
             organization.\n\nReturns an object with `items` and `next_page`, ordered by library \
             ID. Request the next page with `?page_token=<next_page>` until `next_page` is null. \
             Keep the same authenticated user and active organization for every page; changing \
             that context requires starting a new list request.\n\nEach library has a stable `id`, \
             a display `name`, either `user_id` or `org_id`, an opaque `revision`, and \
             `can_manage`. Use the ID to track a library across renames. `can_manage` allows \
             renaming/deleting the library, not editing all of its projects. Organization members \
             can discover libraries; only organization admins can manage them. Project permissions \
             remain unchanged.\n\nProjects with `library_id: null` have no named library. They do \
             not appear as a library record here. Use `GET \
             /user/projects?scope=personal&unassigned=true` for the default personal group, or \
             `scope=organization&unassigned=true` for the active organization's unassigned \
             projects.\n\nEach library includes its creation source: `zds`, `factory_portal`, or \
             `org_dataset`. Libraries created before source tracking are returned as `zds`. \
             Clients can filter the results by source; API permissions remain \
             unchanged.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items \
             returned by a single call\n- `page_token: Option<String>`: Token returned by previous \
             call to retrieve the subsequent page\n\n```rust,no_run\nuse \
             futures_util::TryStreamExt;\nasync fn example_projects_list_libraries_stream() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut \
             projects = client.projects();\n    let mut stream = \
             projects.list_libraries_stream(Some(4 as u32));\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_libraries<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::ProjectLibraryResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/project-libraries"),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "List personal libraries and libraries belonging to the active \
             organization.\n\nReturns an object with `items` and `next_page`, ordered by library \
             ID. Request the next page with `?page_token=<next_page>` until `next_page` is null. \
             Keep the same authenticated user and active organization for every page; changing \
             that context requires starting a new list request.\n\nEach library has a stable `id`, \
             a display `name`, either `user_id` or `org_id`, an opaque `revision`, and \
             `can_manage`. Use the ID to track a library across renames. `can_manage` allows \
             renaming/deleting the library, not editing all of its projects. Organization members \
             can discover libraries; only organization admins can manage them. Project permissions \
             remain unchanged.\n\nProjects with `library_id: null` have no named library. They do \
             not appear as a library record here. Use `GET \
             /user/projects?scope=personal&unassigned=true` for the default personal group, or \
             `scope=organization&unassigned=true` for the active organization's unassigned \
             projects.\n\nEach library includes its creation source: `zds`, `factory_portal`, or \
             `org_dataset`. Libraries created before source tracking are returned as `zds`. \
             Clients can filter the results by source; API permissions remain \
             unchanged.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items \
             returned by a single call\n- `page_token: Option<String>`: Token returned by previous \
             call to retrieve the subsequent page\n\n```rust,no_run\nuse \
             futures_util::TryStreamExt;\nasync fn example_projects_list_libraries_stream() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut \
             projects = client.projects();\n    let mut stream = \
             projects.list_libraries_stream(Some(4 as u32));\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_libraries_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ProjectLibraryResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("user/project-libraries").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list_libraries(limit, None)
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
                                                crate::types::error::SerdeError::new(
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
                                    |result: crate::types::ProjectLibraryResponseResultsPage| {
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

    #[doc = "Create a personal library or a library owned by the active organization.\n\nSend JSON \
             with a name and explicit ownership scope:\n\n```ignorejson { \"name\": \"Designs\", \
             \"scope\": \"personal\" } ```ignore\n\nUse `scope: \"organization\"` to create a \
             library for the request's active organization; this requires organization-admin \
             access. There is no organization ID in this body. Personal scope always uses the \
             authenticated user.\n\nNames are trimmed, must contain 1–255 characters, and are \
             unique ignoring case within each user or organization. Display capitalization is \
             preserved. Duplicate names return HTTP 409. The HTTP 201 response contains the \
             library ID, ownership, revision, and `can_manage`. Creating a library does not move \
             existing projects; select its ID when creating a project, or move an existing project \
             with `PUT /user/projects/{id}/library`.\n\nInclude `source` to identify the \
             application that created the library:\n\n```ignorejson { \"name\": \"Factory \
             designs\", \"scope\": \"personal\", \"source\": \"factory_portal\" } \
             ```ignore\n\nAccepted sources are `zds`, `factory_portal`, and `org_dataset`. \
             Omitting source defaults to `zds`. Null and unsupported values return HTTP \
             400.\n\nSource is returned in library responses and cannot be changed after creation. \
             Clients use it to decide how to display libraries; it does not change API \
             permissions. No API source filter is provided at the moment.\n\n```rust,no_run\nasync \
             fn example_projects_create_library() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::ProjectLibraryResponse = client\n        .projects()\n        \
             .create_library(&kittycad::types::CreateProjectLibraryRequest {\n            name: \
             \"some-string\".to_string(),\n            scope: \
             kittycad::types::ProjectLibraryScope::Organization,\n            source: \
             Some(kittycad::types::KclProjectLibrarySource::FactoryPortal),\n        })\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_library<'a>(
        &'a self,
        body: &crate::types::CreateProjectLibraryRequest,
    ) -> Result<crate::types::ProjectLibraryResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/project-libraries"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Delete an empty cloud project library.\n\nSupply the library's last-read `revision` \
             as the required `expected_revision` query parameter. URL-encode this value. Only the \
             personal owner or an admin of the active organization can delete the library. \
             Libraries managed by an organization dataset cannot be deleted independently; those \
             requests return HTTP 409.\n\nMove or unassign all projects first. A nonempty library \
             or stale revision returns HTTP 409. Success returns HTTP 204 without deleting \
             projects or history. Library revisions are separate from project `revision` and \
             `metadata_revision`.\n\n**Parameters:**\n\n- `expected_revision: &'astr`: Revision \
             returned by the last library read. (required)\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_projects_delete_library() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .projects()\n        \
             .delete_library(\n            \"some-string\",\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_library<'a>(
        &'a self,
        expected_revision: &'a str,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/project-libraries/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("expected_revision", expected_revision.to_string())];
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

    #[doc = "Rename a library owned by the user or their active organization.\n\nSend JSON containing `name` and `expected_revision`, using the library's `revision` from the last library list response. The personal owner or an admin of the active organization can rename the library. Names follow the same validation and uniqueness rules as library creation.\n\nHTTP 200 returns the updated library and revision. A stale revision or duplicate name returns HTTP 409; refresh the library list before deciding whether to retry. Renaming keeps the library ID, ownership, and project assignments unchanged.\n\nRenaming doesn't change the library's source.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_rename_library() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectLibraryResponse = client\n        .projects()\n        .rename_library(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::RenameProjectLibraryRequest {\n                expected_revision: \"some-string\".to_string(),\n                name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn rename_library<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::RenameProjectLibraryRequest,
    ) -> Result<crate::types::ProjectLibraryResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/project-libraries/{id}".replace("{id}", &format!("{}", id))
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "List projects in a cloud library, newest first.\n\nReturns project summaries in `items`, with `next_page` for continuation. Pass `?page_token=<next_page>` on the same library endpoint until `next_page` is null. Keep the same authenticated user and active organization for every page. Results are ordered by creation time descending, then project ID ascending.\n\nEach summary includes `library_id`, content `revision`, `metadata_revision`, and the caller's project `access` capabilities. Access to a library does not grant additional project permissions.\n\nProjects can move or disappear while pages load. Even an empty page can have a continuation token; stop only when `next_page` is null.\n\nWhen loading several libraries, a project may appear in library A's results, then move to library B before B is loaded. Combining those results can show the same project twice. Use the project ID to show it once, and fetch its latest details to confirm which library it belongs to. A missing list entry may also mean the project moved; fetch its details before treating it as deleted, and preserve unsynced local edits.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_list_library() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectSummaryResponseResultsPage = client\n        .projects()\n        .list_library(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_library_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_library_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_library<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::ProjectSummaryResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/project-libraries/{id}/projects".replace("{id}", &format!("{}", id))
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "List projects in a cloud library, newest first.\n\nReturns project summaries in `items`, with `next_page` for continuation. Pass `?page_token=<next_page>` on the same library endpoint until `next_page` is null. Keep the same authenticated user and active organization for every page. Results are ordered by creation time descending, then project ID ascending.\n\nEach summary includes `library_id`, content `revision`, `metadata_revision`, and the caller's project `access` capabilities. Access to a library does not grant additional project permissions.\n\nProjects can move or disappear while pages load. Even an empty page can have a continuation token; stop only when `next_page` is null.\n\nWhen loading several libraries, a project may appear in library A's results, then move to library B before B is loaded. Combining those results can show the same project twice. Use the project ID to show it once, and fetch its latest details to confirm which library it belongs to. A missing list entry may also mean the project moved; fetch its details before treating it as deleted, and preserve unsynced local edits.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_list_library() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectSummaryResponseResultsPage = client\n        .projects()\n        .list_library(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_library_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_library_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_library_stream<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ProjectSummaryResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("user/project-libraries/{id}/projects"
            .replace("{id}", &format!("{}", id)))
        .to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list_library(id, limit, None)
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
                                                crate::types::error::SerdeError::new(
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

    #[doc = "List the authenticated user's projects.\n\nReturns a page, including personal projects and projects accessible through the request's active organization. Optional `scope=personal` or `scope=organization` narrows ownership; `unassigned=true` includes only projects whose `library_id` is null. Combine them, for example: `GET /user/projects?scope=personal&unassigned=true`. Omitting both filters keeps returning all accessible projects, including projects in named libraries.\n\nUnassigned is a grouping within the existing ownership scope, not a stored default library. Use `GET /user/project-libraries/{id}/projects` for a named library's paginated project list. Filters do not grant additional access.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `scope: Option<crate::types::ProjectAccessScope>`: Restrict results to personal projects or the active organization's projects. When omitted, include all accessible projects.\n- `unassigned: Option<bool>`: Return only projects that have no named library assigned.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::ProjectAccessScope::Organization),\n        Some(true),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        scope: Option<crate::types::ProjectAccessScope>,
        unassigned: Option<bool>,
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

        if let Some(p) = scope {
            query_params.push(("scope", format!("{}", p)));
        }

        if let Some(p) = unassigned {
            query_params.push(("unassigned", format!("{}", p)));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "List the authenticated user's projects.\n\nReturns a page, including personal projects and projects accessible through the request's active organization. Optional `scope=personal` or `scope=organization` narrows ownership; `unassigned=true` includes only projects whose `library_id` is null. Combine them, for example: `GET /user/projects?scope=personal&unassigned=true`. Omitting both filters keeps returning all accessible projects, including projects in named libraries.\n\nUnassigned is a grouping within the existing ownership scope, not a stored default library. Use `GET /user/project-libraries/{id}/projects` for a named library's paginated project list. Filters do not grant additional access.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `scope: Option<crate::types::ProjectAccessScope>`: Restrict results to personal projects or the active organization's projects. When omitted, include all accessible projects.\n- `unassigned: Option<bool>`: Return only projects that have no named library assigned.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_projects_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut projects = client.projects();\n    let mut stream = projects.list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::ProjectAccessScope::Organization),\n        Some(true),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        scope: Option<crate::types::ProjectAccessScope>,
        unassigned: Option<bool>,
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

        if let Some(p) = scope.as_ref() {
            pagination_query_params.push(("scope", format!("{}", p)));
        }

        if let Some(p) = unassigned.as_ref() {
            pagination_query_params.push(("unassigned", format!("{}", p)));
        }

        let stream = self
            .list(limit, None, scope, unassigned)
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
                                                crate::types::error::SerdeError::new(
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

    #[doc = "Create a project for the authenticated user.\n\nSend multipart data with a JSON `body` part and file parts whose filenames are relative project paths. Include an optional `library_id` in the JSON to create the project in a named library:\n\n```ignorejson { \"title\": \"Bracket\", \"entrypoint_path\": \"main.kcl\", \"library_id\": \"<LIBRARY_ID>\" } ```ignore\n\nA personal library creates a personal project. An accessible library belonging to the active organization creates an organization project with the caller as its creator; creating the project does not require library-management rights. Omitting `library_id` or passing null creates an unassigned personal project, even when the request has an active organization.\n\nThe HTTP 201 response includes `id`, `library_id`, content `revision`, and `metadata_revision`. Later saves preserve membership, even if they include a library field. Use `PUT /user/projects/{id}/library` to move or unassign an existing project instead of uploading its files again.\n\n```rust,no_run\nasync fn example_projects_create() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectResponse = client\n        .projects()\n        .create(vec![kittycad::types::multipart::Attachment {\n            name: \"thing\".to_string(),\n            filepath: Some(\"myfile.json\".into()),\n            content_type: Some(\"application/json\".to_string()),\n            data: std::fs::read(\"myfile.json\").unwrap(),\n        }])\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Replace one of the authenticated user's projects.\n\nContent saves use the project's content `revision` as `expected_revision` and preserve its current library assignment. Omitting `library_id`, setting it to null, or supplying another library ID here does not move the project. Use `PUT /user/projects/{id}/library` with `metadata_revision` for library changes.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_update() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectResponse = client\n        .projects()\n        .update(\n            vec![kittycad::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Download all or selected files from the current version of a project.\n\nUse `file_ids=<UUID>,<UUID>` to select files from the current version. This endpoint resolves the project's current version at request time, so file IDs from before a save may no longer belong to it. To download those files, use `/user/projects/{id}/versions/{version_id}/download` with `version_id` set to the `summary.revision` returned alongside that file list. A single selected file still uses the requested tar or zip archive format.\n\n**Parameters:**\n\n- `file_ids: Option<Vec<uuid::Uuid>>`: Optional comma-separated file IDs from this download's version. Omit to download every file. An empty or malformed list returns HTTP 400 with `error_code: InvalidProjectFileIds`. Any ID outside this version returns HTTP 400 with `error_code: InvalidProjectFileSelection` and a `missing_file_ids` array listing the requested IDs absent from this version. The entire selection fails: requesting ten IDs with one missing returns HTTP 400 without an archive. Missing projects or versions still return HTTP 404. Repeated IDs are included once. Selecting one file still returns the requested archive format.\n- `format: Option<crate::types::ProjectArchiveFormat>`: Archive format to return. Defaults to `tar`.\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_download() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .projects()\n        .download(\n            Some(vec![uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?]),\n            Some(kittycad::types::ProjectArchiveFormat::Zip),\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn download<'a>(
        &'a self,
        file_ids: Option<Vec<uuid::Uuid>>,
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
        if let Some(p) = file_ids {
            query_params.push(("file_ids", itertools::join(p, ",")));
        }

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

    #[doc = "Move a project to a named library or leave it unassigned.\n\nRead the project first, then send JSON using its `metadata_revision` as the required `expected_revision`. Do not use its content `revision` or a library's revision for this operation.\n\n```ignorejson { \"library_id\": \"<DESTINATION_LIBRARY_ID>\", \"expected_revision\": \"<METADATA_REVISION>\" } ```ignore\n\nSave the JSON as `move-project.json`, with values from actual responses:\n\n```ignoresh curl --fail-with-body \\   --request PUT \"${API_BASE_URL}/user/projects/${PROJECT_ID}/library\" \\   --header \"Authorization: Bearer ${API_TOKEN}\" \\   --header 'Content-Type: application/json' \\   --data-binary @move-project.json ```ignore\n\nUse `library_id: null` to unassign the project. The destination must match the project's current ownership: the same user for a personal project, or the same organization for an organization project. Existing project write permission is required; managing the destination library does not grant it. To change ownership first, use `/user/projects/{id}/organization`.\n\nHTTP 200 returns the project with its library assignment and a new `metadata_revision`. A move preserves its ID, files, current content `revision`, version history, publication, and share links; it creates no content version. Ordinary saves preserve membership and cannot be used to move a project.\n\nA stale revision or a destination with different ownership returns HTTP 409. If another save, move, or ownership change conflicts with the move, fetch the latest project before deciding whether to retry. Missing or inaccessible projects/libraries fail their normal existence and permission checks.\n\nIf the connection drops or the request times out, the API may have completed the move even though the client did not receive a response. Fetch the project to check its current library. If library_id matches the requested destination, no further move is needed. Otherwise review its latest state before retrying with its new `metadata_revision`. No idempotency key is used for this operation.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_projects_update_library() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ProjectResponse = client\n        .projects()\n        .update_library(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::UpdateProjectLibraryRequest {\n                expected_revision: \"some-string\".to_string(),\n                library_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_library<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::UpdateProjectLibraryRequest,
    ) -> Result<crate::types::ProjectResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/projects/{id}/library".replace("{id}", &format!("{}", id))
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Transfer one of the authenticated user's projects to their active \
             organization.\n\nThis changes only the project's ownership scope. The project ID, \
             current revision, files, and version history remain unchanged so cloud bindings stay \
             valid across the move.\n\nAn actual ownership change clears `library_id` and changes \
             `metadata_revision`. The project becomes unassigned in the organization. To place it \
             in a named organization library, use `PUT /user/projects/{id}/library` afterwards \
             with the returned metadata revision. An already organization-owned project in the \
             same organization is unchanged by this request.\n\n**Parameters:**\n\n- `id: \
             uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_projects_update_organization() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::ProjectResponse = client\n        .projects()\n        \
             .update_organization(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Transfer an organization project back to its creator's personal \
             ownership.\n\nOrganization administrators may perform this move to revoke \
             organization access. The project ID, current revision, files, and version history \
             remain unchanged.\n\nThe ownership change clears `library_id` and changes \
             `metadata_revision`, leaving the project unassigned under its creator. Success \
             returns HTTP 204. Fetch the project again before assigning it to a personal named \
             library.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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
                                                crate::types::error::SerdeError::new(
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

        let stream = self . list_versions (id , limit , None) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold ((None , result) , move | (prev_page_token , new_result) | { let pagination_url_path = pagination_url_path . clone () ; let pagination_query_params = pagination_query_params . clone () ; async move { if new_result . has_more_pages () && ! new_result . items () . is_empty () && prev_page_token != new_result . next_page_token () { async { let mut req = self . client . client . request (http :: Method :: GET , format ! ("{}/{}" , self . client . base_url , pagination_url_path . clone ()) ,) ; req = req . bearer_auth (& self . client . token) ; let query_params = pagination_query_params . clone () ; req = req . query (& query_params) ; let mut request = req . build () ? ; request = new_result . next_page_with_param (request , "page_token") ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (crate :: types :: error :: SerdeError :: new (text . to_string () , err) , status)) } else { let text = resp . text () . await . unwrap_or_default () ; Err (crate :: types :: error :: Error :: Server { body : text . to_string () , status }) } } . map_ok (| result : crate :: types :: ProjectVersionSummaryResponseResultsPage | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , (new_result . next_page_token () , result) ,)) }) . await } else { Ok (None) } } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () ;
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Download all or selected files saved in one project version.\n\nSelect files with \
             `file_ids=<UUID>,<UUID>`. IDs must belong to this version; omitted files are \
             excluded, including the manifest unless explicitly selected. The response is a tar or \
             zip archive even when selecting one file.\n\n**Parameters:**\n\n- `file_ids: \
             Option<Vec<uuid::Uuid>>`: Optional comma-separated file IDs from this download's \
             version. Omit to download every file. An empty or malformed list returns HTTP 400 \
             with `error_code: InvalidProjectFileIds`. Any ID outside this version returns HTTP \
             400 with `error_code: InvalidProjectFileSelection` and a `missing_file_ids` array \
             listing the requested IDs absent from this version. The entire selection fails: \
             requesting ten IDs with one missing returns HTTP 400 without an archive. Missing \
             projects or versions still return HTTP 404. Repeated IDs are included once. Selecting \
             one file still returns the requested archive format.\n- `format: \
             Option<crate::types::ProjectArchiveFormat>`: Archive format to return. Defaults to \
             `tar`.\n- `id: uuid::Uuid`: Project identifier. (required)\n- `version_id: \
             uuid::Uuid`: Requested version identifier. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_projects_download_version() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .projects()\n        .download_version(\n            \
             Some(vec![uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?]),\n            \
             Some(kittycad::types::ProjectArchiveFormat::Zip),\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn download_version<'a>(
        &'a self,
        file_ids: Option<Vec<uuid::Uuid>>,
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
        if let Some(p) = file_ids {
            query_params.push(("file_ids", itertools::join(p, ",")));
        }

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
