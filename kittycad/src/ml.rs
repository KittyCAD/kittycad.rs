use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Ml {
    pub client: Client,
}

impl Ml {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Generate a CAD model from text.\n\nBecause our source of truth for the resulting model is a STEP file, you will always have STEP file contents when you list your generated models. Any other formats you request here will also be returned when you list your generated models.\n\nThis operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\nOne thing to note, if you hit the cache, this endpoint will return right away. So you only have to wait if the status is not `Completed` or `Failed`.\n\n**Parameters:**\n\n- `kcl: Option<bool>`: If we should output the kcl for the model.\n- `output_format: crate::types::FileExportFormat`: The format the output file should be converted to. (required)\n\n```rust,no_run\nasync fn example_ml_create_text_to_cad() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCad = client\n        .ml()\n        .create_text_to_cad(\n            Some(true),\n            kittycad::types::FileExportFormat::Ply,\n            &kittycad::types::TextToCadCreateBody {\n                kcl_version: Some(\"some-string\".to_string()),\n                project_name: Some(\"some-string\".to_string()),\n                prompt: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Generate code completions for KCL.\n\n```rust,no_run\nasync fn example_ml_create_kcl_code_completions() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::KclCodeCompletionResponse = client\n        .ml()\n        .create_kcl_code_completions(&kittycad::types::KclCodeCompletionRequest {\n            extra: Some(kittycad::types::KclCodeCompletionParams {\n                language: Some(\"some-string\".to_string()),\n                next_indent: Some(4 as u8),\n                prompt_tokens: Some(4 as u32),\n                suffix_tokens: Some(4 as u32),\n                trim_by_indentation: true,\n            }),\n            max_tokens: Some(4 as u16),\n            n: Some(4 as u8),\n            nwo: Some(\"some-string\".to_string()),\n            prompt: Some(\"some-string\".to_string()),\n            stop: Some(vec![\"some-string\".to_string()]),\n            stream: true,\n            suffix: Some(\"some-string\".to_string()),\n            temperature: Some(3.14 as f64),\n            top_p: Some(3.14 as f64),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Iterate on a CAD model with a prompt.\n\nEven if you give specific ranges to edit, the model might change more than just those in order to make the changes you requested without breaking the code.\n\nYou always get the whole code back, even if you only changed a small part of it.\n\nThis operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\nThis endpoint will soon be deprecated in favor of the `/ml/text-to-cad/multi-file/iteration` endpoint. In that the endpoint path will remain but it will have the same behavior as `ml/text-to-cad/multi-file/iteration`.\n\n```rust,no_run\nasync fn example_ml_create_text_to_cad_iteration() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCadIteration = client\n        .ml()\n        .create_text_to_cad_iteration(&kittycad::types::TextToCadIterationBody {\n            kcl_version: Some(\"some-string\".to_string()),\n            original_source_code: \"some-string\".to_string(),\n            project_name: Some(\"some-string\".to_string()),\n            prompt: Some(\"some-string\".to_string()),\n            source_ranges: vec![kittycad::types::SourceRangePrompt {\n                file: Some(\"some-string\".to_string()),\n                prompt: \"some-string\".to_string(),\n                range: kittycad::types::SourceRange {\n                    end: kittycad::types::SourcePosition {\n                        column: 4 as u32,\n                        line: 4 as u32,\n                    },\n                    start: kittycad::types::SourcePosition {\n                        column: 4 as u32,\n                        line: 4 as u32,\n                    },\n                },\n            }],\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Iterate on a multi-file CAD model with a prompt.\n\nThis endpoint can iterate on multi-file models.\n\nEven if you give specific ranges to edit, the model might change more than just those in order to make the changes you requested without breaking the code.\n\nYou always get the whole code back, even if you only changed a small part of it. This endpoint will always return all the code back, including files that were not changed. If your original source code imported a stl/gltf/step/etc file, the output will not include that file since the model will never change non-kcl files. The endpoint will only return the kcl files that were changed.\n\nThis operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\n```rust,no_run\nasync fn example_ml_create_text_to_cad_multi_file_iteration() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCadMultiFileIteration = client\n        .ml()\n        .create_text_to_cad_multi_file_iteration(\n            vec![kittycad::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filename: Some(\"myfile.json\".to_string()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            &kittycad::types::TextToCadMultiFileIterationBody {\n                kcl_version: Some(\"some-string\".to_string()),\n                project_name: Some(\"some-string\".to_string()),\n                prompt: Some(\"some-string\".to_string()),\n                source_ranges: Some(vec![kittycad::types::SourceRangePrompt {\n                    file: Some(\"some-string\".to_string()),\n                    prompt: \"some-string\".to_string(),\n                    range: kittycad::types::SourceRange {\n                        end: kittycad::types::SourcePosition {\n                            column: 4 as u32,\n                            line: 4 as u32,\n                        },\n                        start: kittycad::types::SourcePosition {\n                            column: 4 as u32,\n                            line: 4 as u32,\n                        },\n                    },\n                }]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "List text-to-CAD models you've generated.\n\nThis will always return the STEP file contents as well as the format the user originally requested.\n\nThis endpoint requires authentication by any Zoo user. It returns the text-to-CAD models for the authenticated user.\n\nThe text-to-CAD models are returned in order of creation, with the most recently created text-to-CAD models first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `no_models: Option<bool>`: If we should return the model file contents or just the metadata.\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_ml_list_text_to_cad_models_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut ml = client.ml();\n    let mut stream = ml.list_text_to_cad_models_for_user_stream(\n        Some(4 as u32),\n        Some(true),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_text_to_cad_models_for_user<'a>(
        &'a self,
        limit: Option<u32>,
        no_models: Option<bool>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::TextToCadResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/text-to-cad"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = no_models {
            query_params.push(("no_models", format!("{}", p)));
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

    #[doc = "List text-to-CAD models you've generated.\n\nThis will always return the STEP file contents as well as the format the user originally requested.\n\nThis endpoint requires authentication by any Zoo user. It returns the text-to-CAD models for the authenticated user.\n\nThe text-to-CAD models are returned in order of creation, with the most recently created text-to-CAD models first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `no_models: Option<bool>`: If we should return the model file contents or just the metadata.\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_ml_list_text_to_cad_models_for_user_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut ml = client.ml();\n    let mut stream = ml.list_text_to_cad_models_for_user_stream(\n        Some(4 as u32),\n        Some(true),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_text_to_cad_models_for_user_stream<'a>(
        &'a self,
        limit: Option<u32>,
        no_models: Option<bool>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::TextToCad, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_text_to_cad_models_for_user(limit, no_models, None, sort_by)
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
                            .map_ok(|result: crate::types::TextToCadResultsPage| {
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
             example_ml_get_text_to_cad_model_for_user() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::TextToCad = \
             client\n        .ml()\n        \
             .get_text_to_cad_model_for_user(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_text_to_cad_model_for_user<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::TextToCad, crate::types::error::Error> {
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
             std::str::FromStr;\nasync fn example_ml_create_text_to_cad_model_feedback() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .ml()\n        .create_text_to_cad_model_feedback(\n            \
             kittycad::types::MlFeedback::Accepted,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_text_to_cad_model_feedback<'a>(
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
}
