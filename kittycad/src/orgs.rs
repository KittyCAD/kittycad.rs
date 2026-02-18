use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Orgs {
    pub client: Client,
}

impl Orgs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get an org.\n\nThis endpoint requires authentication by an org admin. It gets the \
             authenticated user's org.\n\n```rust,no_run\nasync fn example_orgs_get() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::Org = client.orgs().get().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(&'a self) -> Result<crate::types::Org, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org"),
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

    #[doc = "Update an org.\n\nThis endpoint requires authentication by an org admin. It updates the authenticated user's org.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_update() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Org = client\n        .orgs()\n        .update(&kittycad::types::OrgDetails {\n            allow_users_in_domain_to_auto_join: Some(true),\n            billing_email: Some(\"email@example.com\".to_string()),\n            domain: Some(\"some-string\".to_string()),\n            image: Some(\"https://example.com/foo/bar\".to_string()),\n            name: Some(\"some-string\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        body: &crate::types::OrgDetails,
    ) -> Result<crate::types::Org, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "org"),
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

    #[doc = "Create an org.\n\nThis endpoint requires authentication by a Zoo user that is not already in an org. It creates a new org for the authenticated user and makes them an admin.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_create() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Org = client\n        .orgs()\n        .create(&kittycad::types::OrgDetails {\n            allow_users_in_domain_to_auto_join: Some(true),\n            billing_email: Some(\"email@example.com\".to_string()),\n            domain: Some(\"some-string\".to_string()),\n            image: Some(\"https://example.com/foo/bar\".to_string()),\n            name: Some(\"some-string\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::OrgDetails,
    ) -> Result<crate::types::Org, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org"),
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

    #[doc = "Delete an org.\n\nIn order to delete an org, you must first delete all of its \
             members, except yourself.\n\nYou must also have no outstanding invoices or unpaid \
             balances.\n\nThis endpoint requires authentication by an org admin. It deletes the \
             authenticated user's org.\n\n```rust,no_run\nasync fn example_orgs_delete() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client.orgs().delete().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!("{}/{}", self.client.base_url, "org"),
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

    #[doc = "Return the IAM policies customers should apply when onboarding an S3 \
             dataset.\n\n**Parameters:**\n\n- `role_arn: &'astr`: IAM role ARN customers expect \
             Zoo to assume when reading the dataset. (required)\n- `uri: &'astr`: Dataset URI used \
             to scope generated IAM policies. (required)\n\n```rust,no_run\nasync fn \
             example_orgs_dataset_s_3_policies() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::DatasetS3Policies \
             = client\n        .orgs()\n        .dataset_s_3_policies(\"some-string\", \
             \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn dataset_s_3_policies<'a>(
        &'a self,
        role_arn: &'a str,
        uri: &'a str,
    ) -> Result<crate::types::DatasetS3Policies, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/dataset/s3/policies"),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![
            ("role_arn", role_arn.to_string()),
            ("uri", uri.to_string()),
        ];
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

    #[doc = "List every dataset that belongs to the caller's organization.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_datasets_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_datasets_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_datasets<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::OrgDatasetResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/datasets"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List every dataset that belongs to the caller's organization.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_datasets_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_datasets_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_datasets_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::OrgDataset, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_datasets(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "org/datasets"),
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
                                    let text = resp.text().await.unwrap_or_default();
                                    Err(crate::types::error::Error::Server {
                                        body: text.to_string(),
                                        status,
                                    })
                                }
                            }
                            .map_ok(|result: crate::types::OrgDatasetResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Register a new org dataset.\n\nIf the dataset lives in S3, call `/org/dataset/s3/policies` first so you can generate the trust, permission, and bucket policies scoped to your dataset before invoking this endpoint.\n\n```rust,no_run\nasync fn example_orgs_create_dataset() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgDataset = client\n        .orgs()\n        .create_dataset(&kittycad::types::CreateOrgDataset {\n            name: \"some-string\".to_string(),\n            source: kittycad::types::OrgDatasetSource {\n                access_role_arn: Some(\"some-string\".to_string()),\n                provider: kittycad::types::StorageProvider::ZooManaged,\n                uri: Some(\"some-string\".to_string()),\n            },\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_dataset<'a>(
        &'a self,
        body: &crate::types::CreateOrgDataset,
    ) -> Result<crate::types::OrgDataset, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/datasets"),
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

    #[doc = "Fetch a single dataset by id so long as it belongs to the authenticated \
             org.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_get_dataset() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgDataset = \
             client\n        .orgs()\n        .get_dataset(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_dataset<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::OrgDataset, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "Update dataset metadata or storage credentials for the caller's organization.\n\nIMPORTANT: Use this endpoint to fix connectivity to the same underlying storage location (e.g. rotating credentials or correcting a typo). Do not repoint an existing dataset at a completely different bucket or provider—create a new dataset instead so conversions in flight keep their original source. This warning applies to every storage backend, not just S3.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_update_dataset() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgDataset = client\n        .orgs()\n        .update_dataset(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::UpdateOrgDataset {\n                name: Some(\"some-string\".to_string()),\n                source: Some(kittycad::types::UpdateOrgDatasetSource {\n                    access_role_arn: Some(\"some-string\".to_string()),\n                    provider: Some(kittycad::types::StorageProvider::ZooManaged),\n                    uri: Some(\"some-string\".to_string()),\n                }),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_dataset<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::UpdateOrgDataset,
    ) -> Result<crate::types::OrgDataset, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "Delete a dataset owned by the caller's organization.\n\nThis is a destructive \
             operation that: - requires org admin authentication and the dataset must belong to \
             the caller's org. - fails with a 409 Conflict if the dataset is still attached to any \
             custom model. - deletes Zoo-managed artifacts for this dataset (converted outputs and \
             embeddings). - does **not** delete or modify the customer's source \
             bucket/prefix.\n\nAll internal artifact deletions are strict; if any cleanup fails, \
             the request fails.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_delete_dataset() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .orgs()\n        \
             .delete_dataset(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_dataset<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "List the file conversions that have been processed for a given dataset owned by the caller's org.\n\nThis endpoint returns lightweight conversion summaries only (including `phase`), and intentionally omits converted KCL output and snapshot image payloads for speed.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::ConversionSortMode>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_list_dataset_conversions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgDatasetFileConversionSummaryResultsPage = client\n        .orgs()\n        .list_dataset_conversions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n            Some(kittycad::types::ConversionSortMode::StatusDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_dataset_conversions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_dataset_conversions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n        Some(kittycad::types::ConversionSortMode::StatusDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_dataset_conversions<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::ConversionSortMode>,
    ) -> Result<crate::types::OrgDatasetFileConversionSummaryResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}/conversions".replace("{id}", &format!("{}", id))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List the file conversions that have been processed for a given dataset owned by the caller's org.\n\nThis endpoint returns lightweight conversion summaries only (including `phase`), and intentionally omits converted KCL output and snapshot image payloads for speed.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::ConversionSortMode>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_list_dataset_conversions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgDatasetFileConversionSummaryResultsPage = client\n        .orgs()\n        .list_dataset_conversions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n            Some(kittycad::types::ConversionSortMode::StatusDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_dataset_conversions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_dataset_conversions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n        Some(kittycad::types::ConversionSortMode::StatusDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_dataset_conversions_stream<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
        sort_by: Option<crate::types::ConversionSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::OrgDatasetFileConversionSummary, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self . list_dataset_conversions (id , limit , None , sort_by) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold ((None , result) , move | (prev_page_token , new_result) | async move { if new_result . has_more_pages () && ! new_result . items () . is_empty () && prev_page_token != new_result . next_page_token () { async { let mut req = self . client . client . request (http :: Method :: GET , format ! ("{}/{}" , self . client . base_url , "org/datasets/{id}/conversions" . replace ("{id}" , & format ! ("{}" , id))) ,) ; req = req . bearer_auth (& self . client . token) ; let mut request = req . build () ? ; request = new_result . next_page (request) ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (format_serde_error :: SerdeError :: new (text . to_string () , err) , status)) } else { let text = resp . text () . await . unwrap_or_default () ; Err (crate :: types :: error :: Error :: Server { body : text . to_string () , status }) } } . map_ok (| result : crate :: types :: OrgDatasetFileConversionSummaryResultsPage | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , (new_result . next_page_token () , result) ,)) }) . await } else { Ok (None) } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () . boxed ()
    }

    #[doc = "Fetch the metadata and converted output for a single dataset conversion.\n\nUnlike list/search endpoints, this returns the full conversion payload: latest output text plus decoded snapshot image payloads for original, raw-KCL, and salon-KCL stages.\n\n**Parameters:**\n\n- `conversion_id: uuid::Uuid`: Conversion identifier. (required)\n- `id: uuid::Uuid`: Dataset identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_get_dataset_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgDatasetFileConversionDetails = client\n        .orgs()\n        .get_dataset_conversion(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_dataset_conversion<'a>(
        &'a self,
        conversion_id: uuid::Uuid,
        id: uuid::Uuid,
    ) -> Result<crate::types::OrgDatasetFileConversionDetails, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}/conversions/{conversion_id}"
                    .replace("{conversion_id}", &format!("{}", conversion_id))
                    .replace("{id}", &format!("{}", id))
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

    #[doc = "Retrigger a specific dataset conversion for the caller's org.\n\n**Parameters:**\n\n- \
             `conversion_id: uuid::Uuid`: Conversion identifier. (required)\n- `id: uuid::Uuid`: \
             Dataset identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_retrigger_dataset_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .orgs()\n        \
             .retrigger_dataset_conversion(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn retrigger_dataset_conversion<'a>(
        &'a self,
        conversion_id: uuid::Uuid,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}/conversions/{conversion_id}/retrigger"
                    .replace("{conversion_id}", &format!("{}", conversion_id))
                    .replace("{id}", &format!("{}", id))
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

    #[doc = "Request a retrigger of conversions for a dataset that belongs to the caller's \
             org.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- \
             `statuses: Option<String>`: Optional comma-separated set of conversion statuses to \
             retrigger.\n\nExample: `statuses=success,in_progress` If omitted, we retrigger all \
             non-success conversions, but only retrigger `in_progress` conversions that have been \
             running for more than 5 minutes.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_retrigger_dataset() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .orgs()\n        \
             .retrigger_dataset(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             Some(\"some-string\".to_string()),\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn retrigger_dataset<'a>(
        &'a self,
        id: uuid::Uuid,
        statuses: Option<String>,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}/retrigger".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = statuses {
            query_params.push(("statuses", p));
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

    #[doc = "Search dataset conversions by conversion ID or file path.\n\nSupports exact conversion-ID matching and fuzzy file-path matching.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `q: Option<String>`: Search text matched against conversion id or file path.\n- `sort_by: Option<crate::types::ConversionSortMode>`: Requested sort mode for matched conversions.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_search_dataset_conversions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgDatasetFileConversionSummaryResultsPage = client\n        .orgs()\n        .search_dataset_conversions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(kittycad::types::ConversionSortMode::StatusDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_orgs_search_dataset_conversions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.search_dataset_conversions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n        Some(\"some-string\".to_string()),\n        Some(kittycad::types::ConversionSortMode::StatusDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn search_dataset_conversions<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
        page_token: Option<String>,
        q: Option<String>,
        sort_by: Option<crate::types::ConversionSortMode>,
    ) -> Result<crate::types::OrgDatasetFileConversionSummaryResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}/search/conversions".replace("{id}", &format!("{}", id))
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

        if let Some(p) = q {
            query_params.push(("q", p));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Search dataset conversions by conversion ID or file path.\n\nSupports exact conversion-ID matching and fuzzy file-path matching.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `q: Option<String>`: Search text matched against conversion id or file path.\n- `sort_by: Option<crate::types::ConversionSortMode>`: Requested sort mode for matched conversions.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_search_dataset_conversions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgDatasetFileConversionSummaryResultsPage = client\n        .orgs()\n        .search_dataset_conversions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(kittycad::types::ConversionSortMode::StatusDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_orgs_search_dataset_conversions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.search_dataset_conversions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n        Some(\"some-string\".to_string()),\n        Some(kittycad::types::ConversionSortMode::StatusDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn search_dataset_conversions_stream<'a>(
        &'a self,
        id: uuid::Uuid,
        limit: Option<u32>,
        q: Option<String>,
        sort_by: Option<crate::types::ConversionSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::OrgDatasetFileConversionSummary, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self . search_dataset_conversions (id , limit , None , q , sort_by) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold ((None , result) , move | (prev_page_token , new_result) | async move { if new_result . has_more_pages () && ! new_result . items () . is_empty () && prev_page_token != new_result . next_page_token () { async { let mut req = self . client . client . request (http :: Method :: GET , format ! ("{}/{}" , self . client . base_url , "org/datasets/{id}/search/conversions" . replace ("{id}" , & format ! ("{}" , id))) ,) ; req = req . bearer_auth (& self . client . token) ; let mut request = req . build () ? ; request = new_result . next_page (request) ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (format_serde_error :: SerdeError :: new (text . to_string () , err) , status)) } else { let text = resp . text () . await . unwrap_or_default () ; Err (crate :: types :: error :: Error :: Server { body : text . to_string () , status }) } } . map_ok (| result : crate :: types :: OrgDatasetFileConversionSummaryResultsPage | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , (new_result . next_page_token () , result) ,)) }) . await } else { Ok (None) } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () . boxed ()
    }

    #[doc = "Return aggregate conversion stats for a dataset owned by the caller's \
             org.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_get_dataset_conversion_stats() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::OrgDatasetConversionStatsResponse = client\n        .orgs()\n        \
             .get_dataset_conversion_stats(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_dataset_conversion_stats<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::OrgDatasetConversionStatsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}/stats".replace("{id}", &format!("{}", id))
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

    #[doc = "Upload source files into a Zoo-managed dataset.\n\nThis endpoint accepts `multipart/form-data` where each file part becomes a source object in the dataset. Paths are normalized and must be relative.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_upload_dataset_files() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UploadOrgDatasetFilesResponse = client\n        .orgs()\n        .upload_dataset_files(\n            vec![kittycad::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn upload_dataset_files<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        id: uuid::Uuid,
    ) -> Result<crate::types::UploadOrgDatasetFilesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/datasets/{id}/uploads".replace("{id}", &format!("{}", id))
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

    #[doc = "List members of your org.\n\nThis endpoint requires authentication by an org admin. It lists the members of the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `role: Option<crate::types::UserOrgRole>`: The organization role to filter by.\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_members_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_members_stream(\n        Some(4 as u32),\n        Some(kittycad::types::UserOrgRole::Member),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_members<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        role: Option<crate::types::UserOrgRole>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::OrgMemberResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/members"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        if let Some(p) = role {
            query_params.push(("role", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List members of your org.\n\nThis endpoint requires authentication by an org admin. It lists the members of the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `role: Option<crate::types::UserOrgRole>`: The organization role to filter by.\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_members_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_members_stream(\n        Some(4 as u32),\n        Some(kittycad::types::UserOrgRole::Member),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_members_stream<'a>(
        &'a self,
        limit: Option<u32>,
        role: Option<crate::types::UserOrgRole>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::OrgMember, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_members(limit, None, role, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "org/members"),
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
                                    let text = resp.text().await.unwrap_or_default();
                                    Err(crate::types::error::Error::Server {
                                        body: text.to_string(),
                                        status,
                                    })
                                }
                            }
                            .map_ok(|result: crate::types::OrgMemberResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Add a member to your org.\n\nIf the user exists, this will add them to your org. If \
             they do not exist, this will create a new user and add them to your org.\n\nIn both \
             cases the user gets an email that they have been added to the org.\n\nIf the user is \
             already in your org, this will return a 400 and a message.\n\nIf the user is already \
             in a different org, this will return a 400 and a message.\n\nThis endpoint requires \
             authentication by an org admin. It adds the specified member to the authenticated \
             user's org.\n\n```rust,no_run\nasync fn example_orgs_create_member() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::OrgMember = client\n        .orgs()\n        \
             .create_member(&kittycad::types::AddOrgMember {\n            email: \
             \"email@example.com\".to_string(),\n            role: \
             kittycad::types::UserOrgRole::Member,\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_member<'a>(
        &'a self,
        body: &crate::types::AddOrgMember,
    ) -> Result<crate::types::OrgMember, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/members"),
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

    #[doc = "Get a member of your org.\n\nThis endpoint requires authentication by an org admin. \
             It gets the specified member of the authenticated user's org.\n\n**Parameters:**\n\n- \
             `user_id: uuid::Uuid`: The user id of the org member. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_get_member() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgMember = \
             client\n        .orgs()\n        .get_member(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_member<'a>(
        &'a self,
        user_id: uuid::Uuid,
    ) -> Result<crate::types::OrgMember, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/members/{user_id}".replace("{user_id}", &format!("{}", user_id))
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

    #[doc = "Update a member of your org.\n\nThis endpoint requires authentication by an org admin. It updates the specified member of the authenticated user's org.\n\n**Parameters:**\n\n- `user_id: uuid::Uuid`: The user id of the org member. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_update_member() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgMember = client\n        .orgs()\n        .update_member(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::UpdateMemberToOrgBody {\n                role: kittycad::types::UserOrgRole::Member,\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_member<'a>(
        &'a self,
        user_id: uuid::Uuid,
        body: &crate::types::UpdateMemberToOrgBody,
    ) -> Result<crate::types::OrgMember, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/members/{user_id}".replace("{user_id}", &format!("{}", user_id))
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

    #[doc = "Remove a member from your org.\n\nThis endpoint requires authentication by an org \
             admin. It removes the specified member from the authenticated user's \
             org.\n\n**Parameters:**\n\n- `user_id: uuid::Uuid`: The user id of the org member. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_delete_member() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .orgs()\n        \
             .delete_member(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_member<'a>(
        &'a self,
        user_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/members/{user_id}".replace("{user_id}", &format!("{}", user_id))
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

    #[doc = "Get the privacy settings for an org.\n\nThis endpoint requires authentication by an \
             org admin. It gets the privacy settings for the authenticated user's \
             org.\n\n```rust,no_run\nasync fn example_orgs_get_privacy_settings() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::PrivacySettings = \
             client.orgs().get_privacy_settings().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_privacy_settings<'a>(
        &'a self,
    ) -> Result<crate::types::PrivacySettings, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/privacy"),
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

    #[doc = "Update the privacy settings for an org.\n\nThis endpoint requires authentication by \
             an org admin. It updates the privacy settings for the authenticated user's \
             org.\n\n```rust,no_run\nasync fn example_orgs_update_privacy_settings() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::PrivacySettings = client\n        .orgs()\n        \
             .update_privacy_settings(&kittycad::types::PrivacySettings {\n            \
             can_train_on_data: true,\n        })\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_privacy_settings<'a>(
        &'a self,
        body: &crate::types::PrivacySettings,
    ) -> Result<crate::types::PrivacySettings, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "org/privacy"),
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

    #[doc = "Get the SAML identity provider.\n\nThis endpoint requires authentication by an org \
             admin.\n\n```rust,no_run\nasync fn example_orgs_get_saml_idp() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::SamlIdentityProvider = client.orgs().get_saml_idp().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_saml_idp<'a>(
        &'a self,
    ) -> Result<crate::types::SamlIdentityProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/saml/idp"),
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

    #[doc = "Update the SAML identity provider.\n\nThis endpoint requires authentication by an org admin.\n\n```rust,no_run\nasync fn example_orgs_update_saml_idp() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::SamlIdentityProvider = client\n        .orgs()\n        .update_saml_idp(&kittycad::types::SamlIdentityProviderCreate {\n            idp_entity_id: Some(\"some-string\".to_string()),\n            idp_metadata_source: kittycad::types::IdpMetadataSource::Base64EncodedXml {\n                data: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n            },\n            signing_keypair: Some(kittycad::types::DerEncodedKeyPair {\n                private_key: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n                public_cert: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n            }),\n            technical_contact_email: Some(\"email@example.com\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_saml_idp<'a>(
        &'a self,
        body: &crate::types::SamlIdentityProviderCreate,
    ) -> Result<crate::types::SamlIdentityProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "org/saml/idp"),
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

    #[doc = "Create a SAML identity provider.\n\nThis endpoint requires authentication by an org admin.\n\n```rust,no_run\nasync fn example_orgs_create_saml_idp() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::SamlIdentityProvider = client\n        .orgs()\n        .create_saml_idp(&kittycad::types::SamlIdentityProviderCreate {\n            idp_entity_id: Some(\"some-string\".to_string()),\n            idp_metadata_source: kittycad::types::IdpMetadataSource::Base64EncodedXml {\n                data: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n            },\n            signing_keypair: Some(kittycad::types::DerEncodedKeyPair {\n                private_key: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n                public_cert: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n            }),\n            technical_contact_email: Some(\"email@example.com\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_saml_idp<'a>(
        &'a self,
        body: &crate::types::SamlIdentityProviderCreate,
    ) -> Result<crate::types::SamlIdentityProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/saml/idp"),
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

    #[doc = "Delete an SAML identity provider.\n\nThis endpoint requires authentication by an org \
             admin.\n\n```rust,no_run\nasync fn example_orgs_delete_saml_idp() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client.orgs().delete_saml_idp().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_saml_idp<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!("{}/{}", self.client.base_url, "org/saml/idp"),
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

    #[doc = "Get the shortlinks for an org.\n\nThis endpoint requires authentication by an org admin. It gets the shortlinks for the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_get_shortlinks_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.get_shortlinks_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_shortlinks<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ShortlinkResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/shortlinks"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get the shortlinks for an org.\n\nThis endpoint requires authentication by an org admin. It gets the shortlinks for the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_get_shortlinks_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.get_shortlinks_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn get_shortlinks_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::Shortlink, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.get_shortlinks(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "org/shortlinks"),
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
                                    let text = resp.text().await.unwrap_or_default();
                                    Err(crate::types::error::Error::Server {
                                        body: text.to_string(),
                                        status,
                                    })
                                }
                            }
                            .map_ok(|result: crate::types::ShortlinkResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "List orgs.\n\nThis endpoint requires authentication by a Zoo employee. The orgs are returned in order of creation, with the most recently created orgs first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::OrgResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "orgs"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List orgs.\n\nThis endpoint requires authentication by a Zoo employee. The orgs are returned in order of creation, with the most recently created orgs first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::Org, crate::types::error::Error>> + Unpin + '_
    {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "orgs"),
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
                                    let text = resp.text().await.unwrap_or_default();
                                    Err(crate::types::error::Error::Server {
                                        body: text.to_string(),
                                        status,
                                    })
                                }
                            }
                            .map_ok(|result: crate::types::OrgResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Get an org.\n\nThis endpoint requires authentication by a Zoo employee. It gets the information for the specified org.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The organization ID. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_get_any() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Org = client\n        .orgs()\n        .get_any(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_any<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::Org, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "orgs/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "Get admin-only details for an organization.\n\nZoo admins can retrieve extended information about any organization, while non-admins receive a 404 to avoid leaking existence.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The organization ID. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_admin_details_get() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgAdminDetails = client\n        .orgs()\n        .admin_details_get(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn admin_details_get<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::OrgAdminDetails, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "orgs/{id}/admin/details".replace("{id}", &format!("{}", id))
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

    #[doc = "Get a user's org.\n\nThis endpoint requires authentication by any Zoo user. It gets \
             the authenticated user's org.\n\nIf the user is not a member of an org, this endpoint \
             will return a 404.\n\n```rust,no_run\nasync fn example_orgs_get_user() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::UserOrgInfo = client.orgs().get_user().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user<'a>(
        &'a self,
    ) -> Result<crate::types::UserOrgInfo, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/org"),
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
}
