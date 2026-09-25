use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Factory {
    pub client: Client,
}

impl Factory {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List Factory jobs owned by your organization.\n\nAny current organization member can list its jobs, including archived jobs. Ownership uses the job's stored organization, so a submitter leaving or deleting their account does not move the job. Former members lose access. Results are paginated, newest first by default, with the job id breaking ties. Internal communication, financial details, and file storage locations are omitted.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_factory_list_org_jobs_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let mut stream = factory.list_org_jobs_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_org_jobs<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::FactoryCustomerJobSummaryResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/factory/jobs"),
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

    #[doc = "List Factory jobs owned by your organization.\n\nAny current organization member can list its jobs, including archived jobs. Ownership uses the job's stored organization, so a submitter leaving or deleting their account does not move the job. Former members lose access. Results are paginated, newest first by default, with the job id breaking ties. Internal communication, financial details, and file storage locations are omitted.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_factory_list_org_jobs_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let mut stream = factory.list_org_jobs_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_org_jobs_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::FactoryCustomerJobSummary, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("org/factory/jobs").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = sort_by.as_ref() {
            pagination_query_params.push(("sort_by", format!("{}", p)));
        }

        let stream = self
            .list_org_jobs(limit, None, sort_by)
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
                                    |result: crate::types::FactoryCustomerJobSummaryResultsPage| {
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

    #[doc = "Get an organization-owned Factory job for any current member.\n\n**Parameters:**\n\n- \
             `job_id: uuid::Uuid`: The requested job's identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_get_org_job() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryCustomerJobDetail = client\n        .factory()\n        \
             .get_org_job(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_org_job<'a>(
        &'a self,
        job_id: uuid::Uuid,
    ) -> Result<crate::types::FactoryCustomerJobDetail, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}".replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "List finishes currently available for customer Factory submissions.\n\nInternal-only \
             entries are omitted. Clients should refetch this endpoint after a catalog validation \
             error before asking the customer to choose again.\n\n```rust,no_run\nasync fn \
             example_factory_get_user_finishes() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             Vec<kittycad::types::FactoryCustomerCatalogOption> =\n        \
             client.factory().get_user_finishes().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_finishes<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::FactoryCustomerCatalogOption>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/factory/finishes"),
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

    #[doc = "List your personal Factory jobs.\n\nReturns jobs owned by your account, including archived jobs. Jobs with an organization owner belong to that organization, even when your account is also associated with them; use `GET /org/factory/jobs` to list those jobs. Results are paginated, newest first by default, with the job id breaking ties. Internal communication, financial details, and file storage locations are omitted.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_factory_list_user_jobs_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let mut stream = factory.list_user_jobs_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_user_jobs<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::FactoryCustomerJobSummaryResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/factory/jobs"),
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

    #[doc = "List your personal Factory jobs.\n\nReturns jobs owned by your account, including archived jobs. Jobs with an organization owner belong to that organization, even when your account is also associated with them; use `GET /org/factory/jobs` to list those jobs. Results are paginated, newest first by default, with the job id breaking ties. Internal communication, financial details, and file storage locations are omitted.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_factory_list_user_jobs_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let mut stream = factory.list_user_jobs_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_user_jobs_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::FactoryCustomerJobSummary, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("user/factory/jobs").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = sort_by.as_ref() {
            pagination_query_params.push(("sort_by", format!("{}", p)));
        }

        let stream = self
            .list_user_jobs(limit, None, sort_by)
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
                                    |result: crate::types::FactoryCustomerJobSummaryResultsPage| {
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

    #[doc = "Submit a part for manufacturing. Requires a signed-in Zoo account.\n\nThe request is `multipart/form-data`: - one JSON part named `body` (`FactoryIntakeForm`) whose `fields` object holds   intake data (material, finish, quantity, notes, …). Material and finish   are required customer-visible catalog names; all other fields are stored   verbatim so they can be added or renamed without an API change. - one or more file parts (any part name). At least one file is required.\n\nThe submitter's identity (email, name, user id) comes from the authenticated account, not the form.\n\nFetch `GET /user/factory/materials` and `GET /user/factory/finishes`, then send the returned exact `material` and `finish` names. The server rejects missing, non-string, unknown, deleted, and internal-only choices with these stable field-specific `error_code` values: - `factory_material_input_missing` - `factory_material_input_invalid_type` - `factory_material_not_found` - `factory_material_not_customer_visible` - `factory_finish_input_missing` - `factory_finish_input_invalid_type` - `factory_finish_not_found` - `factory_finish_not_customer_visible` - `quantity`: a positive integer.\n\nExample `body` part: ```ignorejson { \"fields\": { \"material\": \"6061 Aluminum\", \"finish\": \"Anodized\", \"quantity\": 10, \"notes\": \"deburr all edges\" } } ```ignore\n\nExample request (curl): ```ignore curl -X POST https://api.zoo.dev/user/factory/jobs \\   -H \"Authorization: Bearer $ZOO_API_TOKEN\" \\   -F 'body={\"fields\":{\"material\":\"6061 Aluminum\",\"finish\":\"Anodized\",\"quantity\":10}};type=application/json' \\   -F 'file=@bracket.step' ```ignore\n\nReturns `201` with the created job (`FactoryJobResponse`).\n\n```rust,no_run\nasync fn example_factory_create_user_job() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryJobResponse = client\n        .factory()\n        .create_user_job(vec![kittycad::types::multipart::Attachment {\n            name: \"thing\".to_string(),\n            filepath: Some(\"myfile.json\".into()),\n            content_type: Some(\"application/json\".to_string()),\n            data: std::fs::read(\"myfile.json\").unwrap(),\n        }])\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_user_job<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
    ) -> Result<crate::types::FactoryJobResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/factory/jobs"),
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

    #[doc = "Get a personal Factory job and its current customer-visible \
             specifications.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The requested job's \
             identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_get_user_job() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryCustomerJobDetail = client\n        .factory()\n        \
             .get_user_job(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_job<'a>(
        &'a self,
        job_id: uuid::Uuid,
    ) -> Result<crate::types::FactoryCustomerJobDetail, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}".replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "List materials currently available for customer Factory submissions.\n\nInternal-only \
             entries are omitted. Clients should refetch this endpoint after a catalog validation \
             error before asking the customer to choose again.\n\n```rust,no_run\nasync fn \
             example_factory_get_user_materials() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             Vec<kittycad::types::FactoryCustomerCatalogOption> =\n        \
             client.factory().get_user_materials().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_materials<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::FactoryCustomerCatalogOption>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/factory/materials"),
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
