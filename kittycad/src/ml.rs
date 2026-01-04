use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug, Default)]
pub struct ListTextToCadPartsForUserParams {
    pub conversation_id: Option<uuid::Uuid>,
    pub limit: Option<u32>,
    pub no_models: Option<bool>,
    pub no_parts: Option<bool>,
    pub page_token: Option<String>,
    pub sort_by: Option<crate::types::CreatedAtSortMode>,
}

#[derive(Clone, Debug)]
pub struct Ml {
    pub client: Client,
}

impl Ml {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Generate a CAD model from text.\n\nBecause our source of truth for the resulting model is a STEP file, you will always have STEP file contents when you list your generated parts. Any other formats you request here will also be returned when you list your generated parts.\n\nThis operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\nOne thing to note, if you hit the cache, this endpoint will return right away. So you only have to wait if the status is not `Completed` or `Failed`.\n\n**Parameters:**\n\n- `kcl: Option<bool>`: If we should output the kcl for the model.\n- `output_format: crate::types::FileExportFormat`: The format the output file should be converted to. (required)\n\n```rust,no_run\nasync fn example_ml_create_text_to_cad() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCad = client\n        .ml()\n        .create_text_to_cad(\n            Some(true),\n            kittycad::types::FileExportFormat::Ply,\n            &kittycad::types::TextToCadCreateBody {\n                kcl_version: Some(\"some-string\".to_string()),\n                model_version: Some(\"some-string\".to_string()),\n                project_name: Some(\"some-string\".to_string()),\n                prompt: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_text_to_cad<'a>(
        &'a self,
        kcl: Option<bool>,
        output_format: crate::types::FileExportFormat,
        body: &crate::types::TextToCadCreateBody,
    ) -> Result<crate::types::TextToCad, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "ai/text-to-cad/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = kcl {
            query_params.push(("kcl", format!("{}", p)));
        }

        req = req.query(&query_params);
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

    #[doc = "List all ML prompts.\n\nFor text-to-cad prompts, this will always return the STEP file contents as well as the format the user originally requested.\n\nThis endpoint requires authentication by a Zoo employee.\n\nThe ML prompts are returned in order of creation, with the most recently created ML prompts first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_ml_list_prompts_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut ml = client.ml();\n    let mut stream = ml.list_prompts_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_prompts<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::MlPromptResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "ml-prompts"),
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

    #[doc = "List all ML prompts.\n\nFor text-to-cad prompts, this will always return the STEP file contents as well as the format the user originally requested.\n\nThis endpoint requires authentication by a Zoo employee.\n\nThe ML prompts are returned in order of creation, with the most recently created ML prompts first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_ml_list_prompts_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut ml = client.ml();\n    let mut stream = ml.list_prompts_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_prompts_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::MlPrompt, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_prompts(limit, None, sort_by)
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
                                    format!("{}/{}", self.client.base_url, "ml-prompts"),
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
                            .map_ok(|result: crate::types::MlPromptResultsPage| {
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

    #[doc = "Get a ML prompt.\n\nThis endpoint requires authentication by a Zoo \
             employee.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The id of the model to give \
             feedback to. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_ml_get_prompt() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::MlPrompt = \
             client\n        .ml()\n        .get_prompt(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_prompt<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::MlPrompt, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "ml-prompts/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "List conversations\n\nThis endpoint requires authentication by any Zoo user. It returns the conversations for the authenticated user.\n\nThe conversations are returned in order of creation, with the most recently created conversations first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_ml_list_conversations_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut ml = client.ml();\n    let mut stream = ml.list_conversations_for_user_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_conversations_for_user<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ConversationResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "ml/conversations"),
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

    #[doc = "List conversations\n\nThis endpoint requires authentication by any Zoo user. It returns the conversations for the authenticated user.\n\nThe conversations are returned in order of creation, with the most recently created conversations first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_ml_list_conversations_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut ml = client.ml();\n    let mut stream = ml.list_conversations_for_user_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_conversations_for_user_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::Conversation, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_conversations_for_user(limit, None, sort_by)
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
                                    format!("{}/{}", self.client.base_url, "ml/conversations"),
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
                            .map_ok(|result: crate::types::ConversationResultsPage| {
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

    #[doc = "Converts a proprietary CAD format to KCL.\n\nThis endpoint is used to convert a \
             proprietary CAD format to KCL. The file passed MUST have feature tree data.\n\nA STEP \
             file does not have feature tree data, so it will not work. A sldprt file does have \
             feature tree data, so it will work.\n\nThis endpoint is designed to work with any \
             native proprietary CAD format, for example: - SolidWorks (.sldprt) - Creo (.prt) - \
             Catia (.catpart) - NX (.prt) - Fusion 360 (.f3d)\n\nThis endpoint is deterministic, \
             it preserves the original design intent by using the feature tree data. This endpoint \
             does not use any machine learning or AI.\n\nThis endpoint is currently in beta, and \
             is only available to users with access to the feature. Please contact support if you \
             are interested in getting access.\n\nThis endpoint might have limitations and bugs, \
             please report any issues you encounter. It will be improved over time.\n\nInput \
             filepaths will be normalized and re-canonicalized to be under the current working \
             directory -- so returned paths may differ from provided paths, and care must be taken \
             when handling user provided paths.\n\n**Parameters:**\n\n- `code_option: \
             Option<crate::types::CodeOption>`: The options to run on the code. By default this is \
             set to `execute`.\n\n```rust,no_run\nasync fn example_ml_create_proprietary_to_kcl() \
             -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::KclModel = client\n        .ml()\n        \
             .create_proprietary_to_kcl(\n            vec![kittycad::types::multipart::Attachment \
             {\n                name: \"thing\".to_string(),\n                filepath: \
             Some(\"myfile.json\".into()),\n                content_type: \
             Some(\"application/json\".to_string()),\n                data: \
             std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            \
             Some(kittycad::types::CodeOption::Cleanup),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_proprietary_to_kcl<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        code_option: Option<crate::types::CodeOption>,
    ) -> Result<crate::types::KclModel, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "ml/convert/proprietary-to-kcl"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = code_option {
            query_params.push(("code_option", format!("{}", p)));
        }

        req = req.query(&query_params);
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

    #[doc = "Create a custom ML model that is backed by one or more org datasets.\n\nDataset \
             readiness is enforced via `OrgDatasetFileConversion::status_counts_for_datasets`: - \
             At least one conversion must have status `success`. - No conversions may remain in \
             `queued`. If even a single file is still queued the dataset is treated as “not ready \
             for training.” - A dataset consisting only of `canceled` or `error_*` entries is \
             rejected because there’s nothing usable.\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_ml_create_custom_model() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::CustomModel = client\n        .ml()\n        \
             .create_custom_model(&kittycad::types::CreateCustomModel {\n            dataset_ids: \
             vec![uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?],\n            name: \
             \"some-string\".to_string(),\n            system_prompt: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_custom_model<'a>(
        &'a self,
        body: &crate::types::CreateCustomModel,
    ) -> Result<crate::types::CustomModel, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "ml/custom/models"),
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

    #[doc = "Retrieve the details of a single custom ML model so long as it belongs to the \
             caller’s organization.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_ml_get_custom_model() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomModel = \
             client\n        .ml()\n        .get_custom_model(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_model<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::CustomModel, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "ml/custom/models/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "Update mutable metadata (name, system prompt) for a custom ML model owned by the \
             caller's organization.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_ml_update_custom_model() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomModel = \
             client\n        .ml()\n        .update_custom_model(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             &kittycad::types::UpdateCustomModel {\n                name: \
             Some(\"some-string\".to_string()),\n                system_prompt: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_custom_model<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::UpdateCustomModel,
    ) -> Result<crate::types::CustomModel, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "ml/custom/models/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "List the org datasets that are currently attached to a custom ML model owned by the caller’s organization.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_ml_list_org_datasets_for_model() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::OrgDataset> = client\n        .ml()\n        .list_org_datasets_for_model(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_org_datasets_for_model<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<Vec<crate::types::OrgDataset>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "ml/custom/models/{id}/datasets".replace("{id}", &format!("{}", id))
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

    #[doc = "Generate code completions for KCL.\n\n```rust,no_run\nasync fn example_ml_create_kcl_code_completions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::KclCodeCompletionResponse = client\n        .ml()\n        .create_kcl_code_completions(&kittycad::types::KclCodeCompletionRequest {\n            extra: Some(kittycad::types::KclCodeCompletionParams {\n                language: Some(\"some-string\".to_string()),\n                next_indent: Some(4 as u8),\n                prompt_tokens: Some(4 as u32),\n                suffix_tokens: Some(4 as u32),\n                trim_by_indentation: true,\n            }),\n            max_tokens: Some(4 as u16),\n            model_version: Some(\"some-string\".to_string()),\n            n: Some(4 as u8),\n            nwo: Some(\"some-string\".to_string()),\n            prompt: Some(\"some-string\".to_string()),\n            stop: Some(vec![\"some-string\".to_string()]),\n            stream: true,\n            suffix: Some(\"some-string\".to_string()),\n            temperature: Some(3.14 as f64),\n            top_p: Some(3.14 as f64),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_kcl_code_completions<'a>(
        &'a self,
        body: &crate::types::KclCodeCompletionRequest,
    ) -> Result<crate::types::KclCodeCompletionResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "ml/kcl/completions"),
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

    #[doc = "Iterate on a CAD model with a prompt.\n\nEven if you give specific ranges to edit, the model might change more than just those in order to make the changes you requested without breaking the code.\n\nYou always get the whole code back, even if you only changed a small part of it.\n\nThis operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\nThis endpoint will soon be deprecated in favor of the `/ml/text-to-cad/multi-file/iteration` endpoint. In that the endpoint path will remain but it will have the same behavior as `ml/text-to-cad/multi-file/iteration`.\n\n**NOTE:** This operation is marked as deprecated.\n\n```rust,no_run\nasync fn example_ml_create_text_to_cad_iteration() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCadIteration = client\n        .ml()\n        .create_text_to_cad_iteration(&kittycad::types::TextToCadIterationBody {\n            kcl_version: Some(\"some-string\".to_string()),\n            original_source_code: \"some-string\".to_string(),\n            project_name: Some(\"some-string\".to_string()),\n            prompt: Some(\"some-string\".to_string()),\n            source_ranges: vec![kittycad::types::SourceRangePrompt {\n                file: Some(\"some-string\".to_string()),\n                prompt: \"some-string\".to_string(),\n                range: kittycad::types::SourceRange {\n                    end: kittycad::types::SourcePosition {\n                        column: 4 as u32,\n                        line: 4 as u32,\n                    },\n                    start: kittycad::types::SourcePosition {\n                        column: 4 as u32,\n                        line: 4 as u32,\n                    },\n                },\n            }],\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_text_to_cad_iteration<'a>(
        &'a self,
        body: &crate::types::TextToCadIterationBody,
    ) -> Result<crate::types::TextToCadIteration, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "ml/text-to-cad/iteration"),
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

    #[doc = "Iterate on a multi-file CAD model with a prompt.\n\nThis endpoint can iterate on multi-file projects.\n\nEven if you give specific ranges to edit, the model might change more than just those in order to make the changes you requested without breaking the code.\n\nYou always get the whole code back, even if you only changed a small part of it. This endpoint will always return all the code back, including files that were not changed. If your original source code imported a stl/gltf/step/etc file, the output will not include that file since the model will never change non-kcl files. The endpoint will only return the kcl files that were changed.\n\nThis operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\nInput filepaths will be normalized and re-canonicalized to be under the current working directory -- so returned paths may differ from provided paths, and care must be taken when handling user provided paths.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_ml_create_text_to_cad_multi_file_iteration() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCadMultiFileIteration = client\n        .ml()\n        .create_text_to_cad_multi_file_iteration(\n            vec![kittycad::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            &kittycad::types::TextToCadMultiFileIterationBody {\n                conversation_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                kcl_version: Some(\"some-string\".to_string()),\n                project_name: Some(\"some-string\".to_string()),\n                prompt: Some(\"some-string\".to_string()),\n                source_ranges: Some(vec![kittycad::types::SourceRangePrompt {\n                    file: Some(\"some-string\".to_string()),\n                    prompt: \"some-string\".to_string(),\n                    range: kittycad::types::SourceRange {\n                        end: kittycad::types::SourcePosition {\n                            column: 4 as u32,\n                            line: 4 as u32,\n                        },\n                        start: kittycad::types::SourcePosition {\n                            column: 4 as u32,\n                            line: 4 as u32,\n                        },\n                    },\n                }]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_text_to_cad_multi_file_iteration<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        body: &crate::types::TextToCadMultiFileIterationBody,
    ) -> Result<crate::types::TextToCadMultiFileIteration, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "ml/text-to-cad/multi-file/iteration"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        let mut json_part = reqwest::multipart::Part::text(serde_json::to_string(&body)?);
        json_part = json_part.file_name(format!("{}.json", "body"));
        json_part = json_part.mime_str("application/json")?;
        form = form.part("body", json_part);
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

    #[doc = "List text-to-CAD parts you've generated.\n\nThis will always return the STEP file contents as well as the format the user originally requested.\n\nThis endpoint requires authentication by any Zoo user. It returns the text-to-CAD parts for the authenticated user.\n\nThe text-to-CAD parts are returned in order of creation, with the most recently created text-to-CAD parts first.\n\n**Parameters:**\n\n- `conversation_id: Option<uuid::Uuid>`: If specified, only return the prompts for the conversation id given.\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `no_models: Option<bool>`: DEPRECATED: This is the same as `no_parts`, and will be dropped in a future release. Please do not use this.\n- `no_parts: Option<bool>`: If we should return the part contents or just the metadata.\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_ml_list_text_to_cad_parts_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCadResponseResultsPage = client\n        .ml()\n        .list_text_to_cad_parts_for_user(kittycad::ml::ListTextToCadPartsForUserParams {\n            conversation_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            limit: Some(4 as u32),\n            no_models: Some(true),\n            no_parts: Some(true),\n            page_token: Some(\"some-string\".to_string()),\n            sort_by: Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_ml_list_text_to_cad_parts_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut ml = client.ml();\n    let mut stream =\n        ml.list_text_to_cad_parts_for_user_stream(kittycad::ml::ListTextToCadPartsForUserParams {\n            conversation_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            limit: Some(4 as u32),\n            no_models: Some(true),\n            no_parts: Some(true),\n            page_token: Some(\"some-string\".to_string()),\n            sort_by: Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        });\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_text_to_cad_parts_for_user<'a>(
        &'a self,
        params: ListTextToCadPartsForUserParams,
    ) -> Result<crate::types::TextToCadResponseResultsPage, crate::types::error::Error> {
        let ListTextToCadPartsForUserParams {
            conversation_id,
            limit,
            no_models,
            no_parts,
            page_token,
            sort_by,
        } = params;
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/text-to-cad"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = conversation_id {
            query_params.push(("conversation_id", format!("{}", p)));
        }

        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = no_models {
            query_params.push(("no_models", format!("{}", p)));
        }

        if let Some(p) = no_parts {
            query_params.push(("no_parts", format!("{}", p)));
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

    #[doc = "List text-to-CAD parts you've generated.\n\nThis will always return the STEP file contents as well as the format the user originally requested.\n\nThis endpoint requires authentication by any Zoo user. It returns the text-to-CAD parts for the authenticated user.\n\nThe text-to-CAD parts are returned in order of creation, with the most recently created text-to-CAD parts first.\n\n**Parameters:**\n\n- `conversation_id: Option<uuid::Uuid>`: If specified, only return the prompts for the conversation id given.\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `no_models: Option<bool>`: DEPRECATED: This is the same as `no_parts`, and will be dropped in a future release. Please do not use this.\n- `no_parts: Option<bool>`: If we should return the part contents or just the metadata.\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_ml_list_text_to_cad_parts_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCadResponseResultsPage = client\n        .ml()\n        .list_text_to_cad_parts_for_user(kittycad::ml::ListTextToCadPartsForUserParams {\n            conversation_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            limit: Some(4 as u32),\n            no_models: Some(true),\n            no_parts: Some(true),\n            page_token: Some(\"some-string\".to_string()),\n            sort_by: Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_ml_list_text_to_cad_parts_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut ml = client.ml();\n    let mut stream =\n        ml.list_text_to_cad_parts_for_user_stream(kittycad::ml::ListTextToCadPartsForUserParams {\n            conversation_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            limit: Some(4 as u32),\n            no_models: Some(true),\n            no_parts: Some(true),\n            page_token: Some(\"some-string\".to_string()),\n            sort_by: Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n        });\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_text_to_cad_parts_for_user_stream<'a>(
        &'a self,
        params: ListTextToCadPartsForUserParams,
    ) -> impl futures::Stream<
        Item = Result<crate::types::TextToCadResponse, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let mut params = params;
        params.page_token = Default::default();
        let params_for_call = params.clone();
        self.list_text_to_cad_parts_for_user(params_for_call)
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
                                    format!("{}/{}", self.client.base_url, "user/text-to-cad"),
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
                            .map_ok(|result: crate::types::TextToCadResponseResultsPage| {
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

    #[doc = "Get a text-to-CAD response.\n\nThis endpoint requires authentication by any Zoo user. \
             The user must be the owner of the text-to-CAD model.\n\n**Parameters:**\n\n- `id: \
             uuid::Uuid`: The id of the model to give feedback to. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_ml_get_text_to_cad_part_for_user() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCadResponse \
             = client\n        .ml()\n        \
             .get_text_to_cad_part_for_user(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_text_to_cad_part_for_user<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::TextToCadResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/text-to-cad/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "Give feedback to a specific ML response.\n\nThis can be a text-to-CAD creation or \
             iteration.\n\nThis endpoint requires authentication by any Zoo user. The user must be \
             the owner of the ML response, in order to give feedback.\n\n**Parameters:**\n\n- \
             `feedback: crate::types::MlFeedback`: The feedback. (required)\n- `id: uuid::Uuid`: \
             The id of the model to give feedback to. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_ml_create_text_to_cad_part_feedback() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .ml()\n        .create_text_to_cad_part_feedback(\n            \
             kittycad::types::MlFeedback::Accepted,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_text_to_cad_part_feedback<'a>(
        &'a self,
        feedback: crate::types::MlFeedback,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/text-to-cad/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("feedback", format!("{}", feedback))];
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

    #[doc = "Open a websocket to prompt the ML copilot.\n\nThis endpoint accepts typed query \
             parameters via `MlCopilotQuery`. See the field documentation on that struct for \
             details, including replay behavior and wire format.\n\n**Parameters:**\n\n- \
             `conversation_id: Option<uuid::Uuid>`: Conversation to replay (UUID). Required when \
             `replay` is `true`.\n- `pr: Option<u64>`: Optional Pull Request number to route \
             traffic.\n- `replay: Option<bool>`: If `true`, emit MsgPack Replay for the specified \
             conversation and continue."]
    #[tracing::instrument]
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn copilot_ws<'a>(
        &'a self,
        conversation_id: Option<uuid::Uuid>,
        pr: Option<u64>,
        replay: Option<bool>,
    ) -> Result<(reqwest::Upgraded, http::HeaderMap), crate::types::error::Error> {
        let mut req = self.client.client_http1_only.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "ws/ml/copilot"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = conversation_id {
            query_params.push(("conversation_id", format!("{}", p)));
        }

        if let Some(p) = pr {
            query_params.push(("pr", format!("{}", p)));
        }

        if let Some(p) = replay {
            query_params.push(("replay", format!("{}", p)));
        }

        req = req.query(&query_params);
        req = req
            .header(reqwest::header::CONNECTION, "Upgrade")
            .header(reqwest::header::UPGRADE, "websocket")
            .header(reqwest::header::SEC_WEBSOCKET_VERSION, "13")
            .header(
                reqwest::header::SEC_WEBSOCKET_KEY,
                base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    rand::random::<[u8; 16]>(),
                ),
            );
        let resp = req.send().await?;
        if resp.status().is_client_error() || resp.status().is_server_error() {
            return Err(crate::types::error::Error::UnexpectedResponse(resp));
        }

        let headers = resp.headers().clone();
        let upgraded = resp
            .upgrade()
            .await
            .map_err(crate::types::error::Error::RequestError)?;
        Ok((upgraded, headers))
    }

    #[doc = "Open a websocket to prompt the ML copilot.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: \
             The ID of the async operation. (required)"]
    #[tracing::instrument]
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn reasoning_ws<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<(reqwest::Upgraded, http::HeaderMap), crate::types::error::Error> {
        let mut req = self.client.client_http1_only.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "ws/ml/reasoning/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req
            .header(reqwest::header::CONNECTION, "Upgrade")
            .header(reqwest::header::UPGRADE, "websocket")
            .header(reqwest::header::SEC_WEBSOCKET_VERSION, "13")
            .header(
                reqwest::header::SEC_WEBSOCKET_KEY,
                base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    rand::random::<[u8; 16]>(),
                ),
            );
        let resp = req.send().await?;
        if resp.status().is_client_error() || resp.status().is_server_error() {
            return Err(crate::types::error::Error::UnexpectedResponse(resp));
        }

        let headers = resp.headers().clone();
        let upgraded = resp
            .upgrade()
            .await
            .map_err(crate::types::error::Error::RequestError)?;
        Ok((upgraded, headers))
    }
}
