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

    #[doc = "Cancel your organization's Factory job before payment starts. Requires organization \
             membership.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The requested job's \
             identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_cancel_org_job() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryCustomerJobSummary = client\n        .factory()\n        \
             .cancel_org_job(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn cancel_org_job<'a>(
        &'a self,
        job_id: uuid::Uuid,
    ) -> Result<crate::types::FactoryCustomerJobSummary, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}/cancel".replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "Download an exact submitted manufacturing input for a current organization \
             member.\n\n**Parameters:**\n\n- `file_id: uuid::Uuid`: Submitted file identifier. \
             (required)\n- `job_id: uuid::Uuid`: Stable job identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_get_org_job_file() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .factory()\n        \
             .get_org_job_file(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_org_job_file<'a>(
        &'a self,
        file_id: uuid::Uuid,
        job_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}/files/{file_id}"
                    .replace("{file_id}", &format!("{}", file_id))
                    .replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "Get your organization's published Factory quote and current checkout amounts. \
             Requires organization membership.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The \
             manufacturing job being purchased. (required)\n- `payment_attempt_id: uuid::Uuid`: \
             Identity reused by the corresponding acceptance and its transport retries. \
             (required)\n- `quote_id: uuid::Uuid`: The exact published quote accepted by the \
             customer. (required)\n- `use_credits: Option<bool>`: Include available Zoo usage \
             credits in the checkout estimate.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_get_org_quote() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryQuoteDetail = client\n        .factory()\n        \
             .get_org_quote(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             Some(true),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_org_quote<'a>(
        &'a self,
        job_id: uuid::Uuid,
        payment_attempt_id: uuid::Uuid,
        quote_id: uuid::Uuid,
        use_credits: Option<bool>,
    ) -> Result<crate::types::FactoryQuoteDetail, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}/quotes/{quote_id}"
                    .replace("{job_id}", &format!("{}", job_id))
                    .replace("{quote_id}", &format!("{}", quote_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![("payment_attempt_id", format!("{}", payment_attempt_id))];
        if let Some(p) = use_credits {
            query_params.push(("use_credits", format!("{}", p)));
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

    #[doc = "Accept your organization's current Factory quote and charge its default card. \
             Requires organization membership.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The \
             manufacturing job being purchased. (required)\n- `max_amount_due_cents: Option<i64>`: \
             Maximum cash payment confirmed by the customer from the checkout preview. If the \
             amount increases, refresh the preview before retrying acceptance.\n- \
             `payment_attempt_id: uuid::Uuid`: Generate once for an explicit acceptance and reuse \
             for every transport retry. A new attempt after a confirmed decline requires a new \
             UUID. (required)\n- `quote_id: uuid::Uuid`: The exact published quote accepted by the \
             customer. (required)\n- `use_credits: Option<bool>`: Apply available Zoo usage \
             credits to this purchase before charging the default card.\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_factory_accept_org_quote() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryQuoteAcceptance = client\n        .factory()\n        \
             .accept_org_quote(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 \
             as i64),\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             Some(true),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn accept_org_quote<'a>(
        &'a self,
        job_id: uuid::Uuid,
        max_amount_due_cents: Option<i64>,
        payment_attempt_id: uuid::Uuid,
        quote_id: uuid::Uuid,
        use_credits: Option<bool>,
    ) -> Result<crate::types::FactoryQuoteAcceptance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}/quotes/{quote_id}/accept"
                    .replace("{job_id}", &format!("{}", job_id))
                    .replace("{quote_id}", &format!("{}", quote_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![("payment_attempt_id", format!("{}", payment_attempt_id))];
        if let Some(p) = max_amount_due_cents {
            query_params.push(("max_amount_due_cents", format!("{}", p)));
        }

        if let Some(p) = use_credits {
            query_params.push(("use_credits", format!("{}", p)));
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

    #[doc = "Decline your organization's current Factory quote and close the job. Requires organization membership.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The manufacturing job being purchased. (required)\n- `quote_id: uuid::Uuid`: The exact published quote accepted by the customer. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_factory_decline_org_quote() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryCustomerJobSummary = client\n        .factory()\n        .decline_org_quote(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn decline_org_quote<'a>(
        &'a self,
        job_id: uuid::Uuid,
        quote_id: uuid::Uuid,
    ) -> Result<crate::types::FactoryCustomerJobSummary, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}/quotes/{quote_id}/decline"
                    .replace("{job_id}", &format!("{}", job_id))
                    .replace("{quote_id}", &format!("{}", quote_id))
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

    #[doc = "List an organization's submitted revision history for a current member.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The requested job's identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_factory_list_org_job_versions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryCustomerJobVersionResultsPage = client\n        .factory()\n        .list_org_job_versions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_factory_list_org_job_versions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let mut stream = factory.list_org_job_versions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_org_job_versions<'a>(
        &'a self,
        job_id: uuid::Uuid,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::FactoryCustomerJobVersionResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}/versions".replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "List an organization's submitted revision history for a current member.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The requested job's identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_factory_list_org_job_versions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryCustomerJobVersionResultsPage = client\n        .factory()\n        .list_org_job_versions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_factory_list_org_job_versions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let mut stream = factory.list_org_job_versions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_org_job_versions_stream<'a>(
        &'a self,
        job_id: uuid::Uuid,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::FactoryCustomerJobVersion, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("org/factory/jobs/{job_id}/versions"
            .replace("{job_id}", &format!("{}", job_id)))
        .to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = sort_by.as_ref() {
            pagination_query_params.push(("sort_by", format!("{}", p)));
        }

        let stream = self
            .list_org_job_versions(job_id, limit, None, sort_by)
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
                                    |result: crate::types::FactoryCustomerJobVersionResultsPage| {
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

    #[doc = "Submit a complete new revision as a current human member of the owning organization.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The requested job's identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_factory_create_org_job_version() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryJobResponse = client\n        .factory()\n        .create_org_job_version(\n            vec![kittycad::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_org_job_version<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        job_id: uuid::Uuid,
    ) -> Result<crate::types::FactoryJobResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}/versions".replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "Fetch the private thumbnail pinned by an organization Factory \
             revision.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: Stable job identifier. \
             (required)\n- `version_id: uuid::Uuid`: Factory revision identifier, not a Project \
             version identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_get_org_job_thumbnail() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .factory()\n        \
             .get_org_job_thumbnail(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_org_job_thumbnail<'a>(
        &'a self,
        job_id: uuid::Uuid,
        version_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/factory/jobs/{job_id}/versions/{version_id}/thumbnail"
                    .replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "List finishes currently available for customer Factory submissions.\n\nInternal-only \
             entries are omitted. Results are ordered alphabetically, ignoring case, with \
             \"Other\" last. Clients should refetch this endpoint after a catalog validation error \
             before asking the customer to choose again.\n\n**Parameters:**\n\n- `limit: \
             Option<u32>`: Maximum number of items returned by a single call\n- `page_token: \
             Option<String>`: Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_factory_get_user_finishes_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let \
             mut stream = factory.get_user_finishes_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_finishes<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::FactoryCustomerCatalogOptionResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/factory/finishes"),
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

    #[doc = "List finishes currently available for customer Factory submissions.\n\nInternal-only \
             entries are omitted. Results are ordered alphabetically, ignoring case, with \
             \"Other\" last. Clients should refetch this endpoint after a catalog validation error \
             before asking the customer to choose again.\n\n**Parameters:**\n\n- `limit: \
             Option<u32>`: Maximum number of items returned by a single call\n- `page_token: \
             Option<String>`: Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_factory_get_user_finishes_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let \
             mut stream = factory.get_user_finishes_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn get_user_finishes_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::FactoryCustomerCatalogOption, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("user/factory/finishes").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self . get_user_finishes (limit , None) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold ((None , result) , move | (prev_page_token , new_result) | { let pagination_url_path = pagination_url_path . clone () ; let pagination_query_params = pagination_query_params . clone () ; async move { if new_result . has_more_pages () && ! new_result . items () . is_empty () && prev_page_token != new_result . next_page_token () { async { let mut req = self . client . client . request (http :: Method :: GET , format ! ("{}/{}" , self . client . base_url , pagination_url_path . clone ()) ,) ; req = req . bearer_auth (& self . client . token) ; let query_params = pagination_query_params . clone () ; req = req . query (& query_params) ; let mut request = req . build () ? ; request = new_result . next_page_with_param (request , "page_token") ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (crate :: types :: error :: SerdeError :: new (text . to_string () , err) , status)) } else { let text = resp . text () . await . unwrap_or_default () ; Err (crate :: types :: error :: Error :: Server { body : text . to_string () , status }) } } . map_ok (| result : crate :: types :: FactoryCustomerCatalogOptionResultsPage | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , (new_result . next_page_token () , result) ,)) }) . await } else { Ok (None) } } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () ;
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
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

    #[doc = "Submit a part for manufacturing. Requires a signed-in Zoo account.\n\nThe request is `multipart/form-data`: - one JSON part named `body` (`FactoryIntakeForm`) whose `fields` object holds   intake data (material, finish, quantity, notes, …). Material and finish   are required customer-visible catalog names; all other fields are stored   verbatim so they can be added or renamed without an API change.   The same JSON part requires `shipping_address_revision_id` and `is_expedited`.   First create a delivery address through `/user/addresses` or `/org/addresses`   for the job's owner, then submit its `revision_id`, not its stable card `id`.   All delivery fields except company and street2 are required for Factory.   `is_expedited` is required and accepts booleans or the exact strings \"true\"/\"false\".   The job pins that revision without changing billing or following later address-book edits. - one or more file parts (any part name). At least one file is required.\n\nThe submitter's identity (email, name, user id) comes from the authenticated account, not the form.\n\nFetch `GET /user/factory/materials` and `GET /user/factory/finishes`, then send the returned exact `material` and `finish` names. The server rejects missing, non-string, unknown, deleted, and internal-only choices with these stable field-specific `error_code` values: - `factory_material_input_missing` - `factory_material_input_invalid_type` - `factory_material_not_found` - `factory_material_not_customer_visible` - `factory_finish_input_missing` - `factory_finish_input_invalid_type` - `factory_finish_not_found` - `factory_finish_not_customer_visible` - `quantity`: a positive integer.\n\nExample `body` part: ```ignorejson { \"fields\": { \"material\": \"6061 Aluminum\", \"finish\": \"Anodized\", \"quantity\": 10 }, \"shipping_address_revision_id\": \"00000000-0000-4000-8000-000000000001\", \"is_expedited\": false } ```ignore\n\nExample request (curl): ```ignore curl -X POST https://api.zoo.dev/user/factory/jobs \\   -H \"Authorization: Bearer $ZOO_API_TOKEN\" \\   -F 'body={\"fields\":{\"material\":\"6061 Aluminum\",\"finish\":\"Anodized\",\"quantity\":10},\"shipping_address_revision_id\":\"00000000-0000-4000-8000-000000000001\",\"is_expedited\":false};type=application/json' \\   -F 'file=@bracket.step' ```ignore\n\nReturns `201` with the created job (`FactoryJobResponse`).\n\n```rust,no_run\nasync fn example_factory_create_user_job() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryJobResponse = client\n        .factory()\n        .create_user_job(vec![kittycad::types::multipart::Attachment {\n            name: \"thing\".to_string(),\n            filepath: Some(\"myfile.json\".into()),\n            content_type: Some(\"application/json\".to_string()),\n            data: std::fs::read(\"myfile.json\").unwrap(),\n        }])\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Cancel your Factory job before payment starts, even if no quote \
             exists.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The requested job's identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_cancel_user_job() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryCustomerJobSummary = client\n        .factory()\n        \
             .cancel_user_job(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn cancel_user_job<'a>(
        &'a self,
        job_id: uuid::Uuid,
    ) -> Result<crate::types::FactoryCustomerJobSummary, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}/cancel".replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "Download an exact submitted manufacturing input of a personal Factory \
             job.\n\n**Parameters:**\n\n- `file_id: uuid::Uuid`: Submitted file identifier. \
             (required)\n- `job_id: uuid::Uuid`: Stable job identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_get_user_job_file() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .factory()\n        \
             .get_user_job_file(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_job_file<'a>(
        &'a self,
        file_id: uuid::Uuid,
        job_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}/files/{file_id}"
                    .replace("{file_id}", &format!("{}", file_id))
                    .replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "Get your published Factory quote with its current shipping, tax, credits, and \
             checkout total.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The manufacturing job \
             being purchased. (required)\n- `payment_attempt_id: uuid::Uuid`: Identity reused by \
             the corresponding acceptance and its transport retries. (required)\n- `quote_id: \
             uuid::Uuid`: The exact published quote accepted by the customer. (required)\n- \
             `use_credits: Option<bool>`: Include available Zoo usage credits in the checkout \
             estimate.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_get_user_quote() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryQuoteDetail = client\n        .factory()\n        \
             .get_user_quote(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             Some(true),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_quote<'a>(
        &'a self,
        job_id: uuid::Uuid,
        payment_attempt_id: uuid::Uuid,
        quote_id: uuid::Uuid,
        use_credits: Option<bool>,
    ) -> Result<crate::types::FactoryQuoteDetail, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}/quotes/{quote_id}"
                    .replace("{job_id}", &format!("{}", job_id))
                    .replace("{quote_id}", &format!("{}", quote_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![("payment_attempt_id", format!("{}", payment_attempt_id))];
        if let Some(p) = use_credits {
            query_params.push(("use_credits", format!("{}", p)));
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

    #[doc = "Accept your current Factory quote and immediately charge your default card. \
             Production starts only after successful payment.\n\n**Parameters:**\n\n- `job_id: \
             uuid::Uuid`: The manufacturing job being purchased. (required)\n- \
             `max_amount_due_cents: Option<i64>`: Maximum cash payment confirmed by the customer \
             from the checkout preview. If the amount increases, refresh the preview before \
             retrying acceptance.\n- `payment_attempt_id: uuid::Uuid`: Generate once for an \
             explicit acceptance and reuse for every transport retry. A new attempt after a \
             confirmed decline requires a new UUID. (required)\n- `quote_id: uuid::Uuid`: The \
             exact published quote accepted by the customer. (required)\n- `use_credits: \
             Option<bool>`: Apply available Zoo usage credits to this purchase before charging the \
             default card.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_accept_user_quote() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryQuoteAcceptance = client\n        .factory()\n        \
             .accept_user_quote(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 \
             as i64),\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             Some(true),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn accept_user_quote<'a>(
        &'a self,
        job_id: uuid::Uuid,
        max_amount_due_cents: Option<i64>,
        payment_attempt_id: uuid::Uuid,
        quote_id: uuid::Uuid,
        use_credits: Option<bool>,
    ) -> Result<crate::types::FactoryQuoteAcceptance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}/quotes/{quote_id}/accept"
                    .replace("{job_id}", &format!("{}", job_id))
                    .replace("{quote_id}", &format!("{}", quote_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![("payment_attempt_id", format!("{}", payment_attempt_id))];
        if let Some(p) = max_amount_due_cents {
            query_params.push(("max_amount_due_cents", format!("{}", p)));
        }

        if let Some(p) = use_credits {
            query_params.push(("use_credits", format!("{}", p)));
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

    #[doc = "Decline your current Factory quote and close the job. No payment is \
             created.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The manufacturing job being \
             purchased. (required)\n- `quote_id: uuid::Uuid`: The exact published quote accepted \
             by the customer. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_decline_user_quote() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryCustomerJobSummary = client\n        .factory()\n        \
             .decline_user_quote(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn decline_user_quote<'a>(
        &'a self,
        job_id: uuid::Uuid,
        quote_id: uuid::Uuid,
    ) -> Result<crate::types::FactoryCustomerJobSummary, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}/quotes/{quote_id}/decline"
                    .replace("{job_id}", &format!("{}", job_id))
                    .replace("{quote_id}", &format!("{}", quote_id))
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

    #[doc = "List a personal Factory job's submitted revision history, newest first by default.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The requested job's identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_factory_list_user_job_versions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryCustomerJobVersionResultsPage = client\n        .factory()\n        .list_user_job_versions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_factory_list_user_job_versions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let mut stream = factory.list_user_job_versions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_user_job_versions<'a>(
        &'a self,
        job_id: uuid::Uuid,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::FactoryCustomerJobVersionResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}/versions".replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "List a personal Factory job's submitted revision history, newest first by default.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: The requested job's identifier. (required)\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_factory_list_user_job_versions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryCustomerJobVersionResultsPage = client\n        .factory()\n        .list_user_job_versions(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(4 as u32),\n            Some(\"some-string\".to_string()),\n            Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_factory_list_user_job_versions_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let mut stream = factory.list_user_job_versions_stream(\n        uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_user_job_versions_stream<'a>(
        &'a self,
        job_id: uuid::Uuid,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::FactoryCustomerJobVersion, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("user/factory/jobs/{job_id}/versions"
            .replace("{job_id}", &format!("{}", job_id)))
        .to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = sort_by.as_ref() {
            pagination_query_params.push(("sort_by", format!("{}", p)));
        }

        let stream = self
            .list_user_job_versions(job_id, limit, None, sort_by)
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
                                    |result: crate::types::FactoryCustomerJobVersionResultsPage| {
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

    #[doc = "Submit a complete new revision of a personal Factory job.\n\n**Parameters:**\n\n- \
             `job_id: uuid::Uuid`: The requested job's identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_create_user_job_version() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::FactoryJobResponse = client\n        .factory()\n        \
             .create_user_job_version(\n            vec![kittycad::types::multipart::Attachment \
             {\n                name: \"thing\".to_string(),\n                filepath: \
             Some(\"myfile.json\".into()),\n                content_type: \
             Some(\"application/json\".to_string()),\n                data: \
             std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_user_job_version<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        job_id: uuid::Uuid,
    ) -> Result<crate::types::FactoryJobResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}/versions".replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "Fetch the private thumbnail pinned by a personal Factory \
             revision.\n\n**Parameters:**\n\n- `job_id: uuid::Uuid`: Stable job identifier. \
             (required)\n- `version_id: uuid::Uuid`: Factory revision identifier, not a Project \
             version identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_factory_get_user_job_thumbnail() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .factory()\n        \
             .get_user_job_thumbnail(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_job_thumbnail<'a>(
        &'a self,
        job_id: uuid::Uuid,
        version_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/factory/jobs/{job_id}/versions/{version_id}/thumbnail"
                    .replace("{job_id}", &format!("{}", job_id))
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

    #[doc = "List materials currently available for customer Factory submissions.\n\nInternal-only \
             entries are omitted. Results are ordered alphabetically, ignoring case, with \
             \"Other\" last. Clients should refetch this endpoint after a catalog validation error \
             before asking the customer to choose again.\n\n**Parameters:**\n\n- `limit: \
             Option<u32>`: Maximum number of items returned by a single call\n- `page_token: \
             Option<String>`: Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_factory_get_user_materials_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let \
             mut stream = factory.get_user_materials_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_materials<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::FactoryCustomerCatalogOptionResultsPage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/factory/materials"),
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

    #[doc = "List materials currently available for customer Factory submissions.\n\nInternal-only \
             entries are omitted. Results are ordered alphabetically, ignoring case, with \
             \"Other\" last. Clients should refetch this endpoint after a catalog validation error \
             before asking the customer to choose again.\n\n**Parameters:**\n\n- `limit: \
             Option<u32>`: Maximum number of items returned by a single call\n- `page_token: \
             Option<String>`: Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_factory_get_user_materials_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut factory = client.factory();\n    let \
             mut stream = factory.get_user_materials_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn get_user_materials_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::FactoryCustomerCatalogOption, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("user/factory/materials").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self . get_user_materials (limit , None) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold ((None , result) , move | (prev_page_token , new_result) | { let pagination_url_path = pagination_url_path . clone () ; let pagination_query_params = pagination_query_params . clone () ; async move { if new_result . has_more_pages () && ! new_result . items () . is_empty () && prev_page_token != new_result . next_page_token () { async { let mut req = self . client . client . request (http :: Method :: GET , format ! ("{}/{}" , self . client . base_url , pagination_url_path . clone ()) ,) ; req = req . bearer_auth (& self . client . token) ; let query_params = pagination_query_params . clone () ; req = req . query (& query_params) ; let mut request = req . build () ? ; request = new_result . next_page_with_param (request , "page_token") ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (crate :: types :: error :: SerdeError :: new (text . to_string () , err) , status)) } else { let text = resp . text () . await . unwrap_or_default () ; Err (crate :: types :: error :: Error :: Server { body : text . to_string () , status }) } } . map_ok (| result : crate :: types :: FactoryCustomerCatalogOptionResultsPage | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , (new_result . next_page_token () , result) ,)) }) . await } else { Ok (None) } } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () ;
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
        }
    }
}
