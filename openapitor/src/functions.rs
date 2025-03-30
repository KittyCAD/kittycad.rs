//! Utilities for generating rust functions from an OpenAPI spec.

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Write as _,
};

use anyhow::Result;
use proc_macro2::TokenStream;

use crate::types::{
    exts::{
        OperationExt, ParameterSchemaOrContentExt, ReferenceOrExt, SchemaRenderExt, StatusCodeExt,
        TokenStreamExt,
    },
    sanitize_indents,
};

/// Returns example
fn generate_websocket_fn(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
    opts: &crate::Opts,
) -> Result<(TokenStream, HashMap<String, String>)> {
    let docs = generate_docs(type_space, name, method, op, global_params)?;
    // Get the function name.
    let fn_name = op.get_fn_name()?;
    let fn_name_ident = format_ident!("{}", fn_name);
    let response_type = quote!((reqwest::Upgraded, http::HeaderMap));
    // Get the function args.
    let raw_args = get_args(name, method, type_space, op, global_params)?;
    // Make sure if we have args, we start with a comma.
    let args = if raw_args.is_empty() {
        quote!()
    } else {
        let a = raw_args.iter().map(|(k, v)| {
            let n = format_ident!("{}", crate::types::clean_property_name(k));
            quote!(#n: #v)
        });
        quote!(,#(#a),*)
    };

    let url_path = name.trim_start_matches('/');
    let method_ident = format_ident!("{}", method.to_string());

    // Let's get the path parameters.
    let path_params = get_path_params(type_space, op, global_params)?;
    let clean_url = clean_url_from(&path_params)?;

    // Let's get the query parameters.
    let query_params = get_query_params(type_space, op, global_params)?;
    let query_params_code = gen_query_params_code(&query_params, false)?;

    let auth_code = generate_auth_code(opts)?;

    let websocket_headers = quote! {
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
    };

    let function_body = quote! {
        let mut req = self.client.client_http1_only.request(
            http::Method::#method_ident,
            format!("{}/{}", self.client.base_url, #url_path #clean_url),
        );

        #auth_code

        #query_params_code

        #websocket_headers

        let resp = req.send().await?;
        if resp.status().is_client_error() || resp.status().is_server_error() {
            return Err(crate::types::error::Error::UnexpectedResponse(resp));
        }

        let headers = resp.headers().clone();
        // TODO: This isn't really a request error, but the response was already consumed.
        // So we can't use Error::UnexpectedResponse.
        let upgraded = resp.upgrade().await.map_err(crate::types::error::Error::RequestError)?;
        Ok((upgraded, headers))
    };

    let function = quote! {
        #[doc = #docs]
        #[tracing::instrument]
        #[cfg(not(target_arch = "wasm32"))]
        pub async fn #fn_name_ident<'a>(&'a self #args) -> Result<#response_type, crate::types::error::Error> {
            #function_body
        }
    };

    // TODO: Build actual example
    Ok((function, Default::default()))
}

/// Generate functions for each path operation.
pub fn generate_files(
    type_space: &mut crate::types::TypeSpace,
    opts: &crate::Opts,
) -> Result<(
    BTreeMap<String, proc_macro2::TokenStream>,
    openapiv3::OpenAPI,
)> {
    let mut tag_files: BTreeMap<String, proc_macro2::TokenStream> = Default::default();

    // Make a spec we can modify for the docs.
    let mut new_spec = type_space.spec.clone();

    for (name, path) in type_space.clone().spec.paths.iter() {
        let op = path.item()?;

        let mut new_path = op.clone();

        let mut gen = |name: &str,
                       method: &http::Method,
                       op: Option<&openapiv3::Operation>,
                       global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>]|
         -> Result<()> {
            // Ensure we have an operation for this path and method, otherwise return early.
            let op = if let Some(op) = op {
                op
            } else {
                return Ok(());
            };

            let tag = op.get_tag()?;
            let example = if op.extensions.contains_key("x-dropshot-websocket") {
                let (function, example) =
                    generate_websocket_fn(type_space, name, method, op, global_params, opts)?;
                add_fn_to_tag(&mut tag_files, &tag, &function)?;
                example
            } else {
                // Get the docs.
                let docs = generate_docs(type_space, name, method, op, global_params)?;

                // Get the function name.
                let fn_name = op.get_fn_name()?;
                let fn_name_ident = format_ident!("{}", fn_name);

                // Get the response for the function.
                let response_type =
                    if let Some(response) = get_response_type(type_space, name, method, op)? {
                        let t = response.type_name;
                        quote!(#t)
                    } else {
                        // We don't have a response, so we'll return `()`.
                        quote!(())
                    };

                // Get the function args.
                let raw_args = get_args(name, method, type_space, op, global_params)?;
                // Make sure if we have args, we start with a comma.
                let args = if raw_args.is_empty() {
                    quote!()
                } else {
                    let a = raw_args.iter().map(|(k, v)| {
                        let n = format_ident!("{}", crate::types::clean_property_name(k));
                        quote!(#n: #v)
                    });
                    quote!(,#(#a),*)
                };

                // Get the request body for the function if there is one.
                let request_body = if let Some(rb) = get_request_body(type_space, name, method, op)?
                {
                    let t = rb.type_name;

                    if is_multipart(type_space, name, method, op)? && !multipart_has_body(&t)? {
                        // We don't have a request body, so we'll return nothing.
                        quote!()
                    } else {
                        // We add the comma at the front, so it works.
                        quote!(, body: &#t)
                    }
                } else {
                    // We don't have a request body, so we'll return nothing.
                    quote!()
                };

                // Get the function body.
                let function_body =
                    get_function_body(type_space, name, method, op, false, opts, global_params)?;

                let example_code_fn = generate_example_code_fn(
                    type_space,
                    name,
                    method,
                    &tag,
                    op,
                    opts,
                    global_params,
                )?;
                // For the rust docs example code we want to trim the doc string since it is
                // repetitive.
                let rust_doc_example_code_fn = &example_code_fn[example_code_fn
                    .find("\nuse ")
                    .unwrap_or_else(|| example_code_fn.find("async fn example_").unwrap_or(0))
                    ..example_code_fn.len()]
                    .trim();

                // Add our example code to our docs.
                // This way we can test the examples compile by running `rust doc`.
                // We want the code to comile but not be run as there are functions that
                // would delete our user etc.
                let docs = format!(
                    r#"{}

```rust,no_run
{}
```"#,
                    docs, rust_doc_example_code_fn
                );

                let function = quote! {
                    #[doc = #docs]
                    #[tracing::instrument]
                    pub async fn #fn_name_ident<'a>(&'a self #args #request_body) -> Result<#response_type, crate::types::error::Error> {
                        #function_body
                    }
                };

                add_fn_to_tag(&mut tag_files, &tag, &function)?;

                // Let's pause here and update our spec with the new function.
                // Add the docs to our spec.
                // let new_operation = op.clone();
                let mut example: HashMap<String, String> = HashMap::new();

                example.insert("example".to_string(), example_code_fn);

                example.insert(
                    "libDocsLink".to_string(),
                    format!(
                        "https://docs.rs/{}/latest/{}/{}/struct.{}.html#method.{}",
                        opts.name,
                        opts.name,
                        tag,
                        crate::types::proper_name(&tag),
                        fn_name
                    ),
                );

                // Let's check if this function can be paginated.
                let pagination_properties =
                    get_pagination_properties(name, method, op, &type_space.spec)?;
                if pagination_properties.can_paginate() {
                    // If we can paginate we should generate a paginated stream function.
                    let stream_fn_name_ident = format_ident!("{}_stream", fn_name);

                    // Get the inner args for the function.
                    let page_param_str = pagination_properties.page_param_str()?;

                    // Make sure if we have args, we start with a comma.
                    // Get the args again without the page param.
                    let min_args = if raw_args.is_empty() {
                        quote!()
                    } else {
                        let mut a = Vec::new();
                        for (k, v) in raw_args.iter() {
                            // Skip the next page arg.
                            if k != &page_param_str {
                                let n = format_ident!("{}", k);
                                a.push(quote!(#n: #v))
                            }
                        }
                        quote!(,#(#a),*)
                    };

                    let inner_args = if raw_args.is_empty() {
                        quote!()
                    } else {
                        let mut a = Vec::new();
                        for (k, _v) in raw_args.iter() {
                            // Skip the next page arg.
                            if k != &page_param_str {
                                let n = format_ident!("{}", k);
                                a.push(quote!(#n))
                            } else {
                                // Make the arg none for our page parameter.
                                a.push(quote!(None))
                            }
                        }
                        quote!(#(#a),*)
                    };

                    // Check if we have a body as an arg.
                    let body_arg = if request_body.is_empty() {
                        quote!()
                    } else {
                        quote!(,body)
                    };

                    let paginated_function_body =
                        get_function_body(type_space, name, method, op, true, opts, global_params)?;

                    let item_type = pagination_properties.item_type(false)?;

                    let function = quote! {
                        #[doc = #docs]
                        #[tracing::instrument]
                        #[cfg(not(feature = "js"))]
                        pub fn #stream_fn_name_ident<'a>(&'a self #min_args #request_body) -> impl futures::Stream<Item = Result<#item_type, crate::types::error::Error>> + Unpin + '_  {
                            use futures::{StreamExt, TryFutureExt, TryStreamExt};
                            use crate::types::paginate::Pagination;

                            // Get the result from our main function.
                            self.#fn_name_ident(#inner_args #body_arg)
                                .map_ok(move |result| {
                                    let items = futures::stream::iter(result.items().into_iter().map(Ok));

                                    // Get the next pages.
                                    let next_pages = futures::stream::try_unfold(
                                        (None, result),
                                        move |(prev_page_token, new_result)| async move {
                                            if new_result.has_more_pages() && !new_result.items().is_empty() && prev_page_token != new_result.next_page_token() {
                                                // Get the next page, we modify the request directly,
                                                // so that if we want to generate an API that uses
                                                // Link headers or any other weird shit it works.
                                                async {
                                                    #paginated_function_body
                                                }.map_ok(|result: #response_type| {
                                                    Some((futures::stream::iter(
                                                            result.items().into_iter().map(Ok),
                                                        ),
                                                        ( new_result.next_page_token(), result),
                                                    ))
                                                })
                                                .await
                                            } else {
                                                // We have no more pages.
                                                Ok(None)
                                            }
                                        }
                                    )
                                    .try_flatten();

                                    items.chain(next_pages)
                                })
                                .try_flatten_stream()
                                .boxed()
                            }
                    };

                    add_fn_to_tag(&mut tag_files, &tag, &function)?;
                }
                example
            };

            // Update our api spec with the new functions.
            let mut new_operation = op.clone();
            new_operation
                .extensions
                .insert("x-rust".to_string(), serde_json::json!(example));
            match method.clone() {
                http::Method::GET => {
                    new_path.get = Some(new_operation);
                }
                http::Method::POST => {
                    new_path.post = Some(new_operation);
                }
                http::Method::PUT => {
                    new_path.put = Some(new_operation);
                }
                http::Method::PATCH => {
                    new_path.patch = Some(new_operation);
                }
                http::Method::DELETE => {
                    new_path.delete = Some(new_operation);
                }
                _ => {}
            }
            new_spec.paths.paths.insert(
                name.to_string(),
                openapiv3::ReferenceOr::Item(new_path.clone()),
            );

            Ok(())
        };

        gen(
            name.as_str(),
            &http::Method::GET,
            op.get.as_ref(),
            &op.parameters,
        )?;
        gen(
            name.as_str(),
            &http::Method::PUT,
            op.put.as_ref(),
            &op.parameters,
        )?;
        gen(
            name.as_str(),
            &http::Method::POST,
            op.post.as_ref(),
            &op.parameters,
        )?;
        gen(
            name.as_str(),
            &http::Method::DELETE,
            op.delete.as_ref(),
            &op.parameters,
        )?;
        gen(
            name.as_str(),
            &http::Method::HEAD,
            op.head.as_ref(),
            &op.parameters,
        )?;
        gen(
            name.as_str(),
            &http::Method::PATCH,
            op.patch.as_ref(),
            &op.parameters,
        )?;
        gen(
            name.as_str(),
            &http::Method::TRACE,
            op.trace.as_ref(),
            &op.parameters,
        )?;
    }

    Ok((tag_files, new_spec))
}

/// Generate the docs for the given operation.
fn generate_docs(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<String> {
    let mut docs = if let Some(summary) = &op.summary {
        summary.to_string()
    } else {
        format!("Perform a `{}` request to `{}`.", method, name)
    };

    if let Some(description) = &op.description {
        docs.push_str("\n\n");
        let description_sanitized = sanitize_indents(description, name.to_string());
        docs.push_str(&description_sanitized.replace("```", "```ignore"));
    }

    // Document the params.
    let mut params = get_path_params_schema(op, &type_space.spec, global_params)?;
    params.append(&mut get_query_params_schema(
        op,
        &type_space.spec,
        global_params,
    )?);

    let params_types = get_args(name, method, type_space, op, global_params)?;

    if !params.is_empty() {
        docs.push_str("\n\n**Parameters:**\n");
    }
    for (name, (_schema, parameter_data)) in params {
        // Get the type of the param.
        let param_type = params_types.get(&name).ok_or_else(|| {
            // This should not happen since both call the same functions.
            anyhow::anyhow!(
                "Could not find type for param `{}` in operation `{}`",
                name,
                name
            )
        })?;
        let mut param_docs = format!(
            "- `{}: {}`",
            crate::types::clean_property_name(&name),
            param_type.rendered()?
        );
        if let Some(description) = &parameter_data.description {
            if !description.trim().is_empty() {
                param_docs.push_str(": ");
                let description_sanitized = sanitize_indents(description, "".to_string());
                param_docs.push_str(&description_sanitized);
            }
        }
        if parameter_data.required {
            param_docs.push_str(" (required)");
        }
        docs.push('\n');
        docs.push_str(&param_docs);
    }

    if op.deprecated {
        docs.push_str("\n\n");
        docs.push_str("**NOTE:** This operation is marked as deprecated.");
    }

    if let Some(external_docs) = &op.external_docs {
        docs.push_str("\n\n");
        if let Some(description) = &external_docs.description {
            write!(
                docs,
                "See <{}|{}> for more information.",
                external_docs.url, description
            )?;
        } else {
            write!(docs, "See <{}> for more information.", external_docs.url)?;
        }
    }

    Ok(docs)
}

struct RequestOrResponse {
    media_type: String,
    type_name: proc_macro2::TokenStream,
}

/// Return the response type for the operation.
fn get_response_type(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
) -> Result<Option<RequestOrResponse>> {
    for (status_code, response) in &op.responses.responses {
        // We only care if the response is a success since this is for the function
        // to return upon success.
        if status_code.is_success() {
            // Then let's get the type for the response.
            let response = response.expand(&type_space.spec)?;

            // Iterate over all the media types and return the first response.
            for (media_type, content) in &response.content {
                if let Some(s) = &content.schema {
                    let t = match s {
                        openapiv3::ReferenceOr::Reference { .. } => {
                            crate::types::get_type_name_from_reference(
                                &s.reference()?,
                                &type_space.spec,
                                false,
                            )?
                        }
                        openapiv3::ReferenceOr::Item(s) => {
                            let on_the_fly_type = crate::types::get_type_name_for_schema(
                                &generate_name_for_fn_schema(name, method, s, op, "Response"),
                                s,
                                &type_space.spec,
                                false,
                            )?;

                            // Make sure we generate the object.
                            type_space.render_schema(&on_the_fly_type.to_string(), s)?;

                            on_the_fly_type
                        }
                    };

                    // Return early since we found the type.
                    return Ok(Some(RequestOrResponse {
                        media_type: media_type.to_string(),
                        type_name: t,
                    }));
                }
            }
        }
    }

    // We couldn't find a type for the response.
    Ok(None)
}

/// Return the schema name for the type.
/// We use this for populating the name of the type, if there is not one.
fn generate_name_for_fn_schema(
    name: &str,
    method: &http::Method,
    schema: &openapiv3::Schema,
    op: &openapiv3::Operation,
    addition: &str,
) -> String {
    if let Some(title) = &schema.schema_data.title {
        return title.to_string();
    }

    if let Some(operation_id) = &op.operation_id {
        return format!("{} {}", operation_id, addition);
    }

    format!("{} {} {}", name, method, addition)
}

/// Return the function arguments for the operation.
fn get_args(
    name: &str,
    method: &http::Method,
    type_space: &mut crate::types::TypeSpace,
    op: &openapiv3::Operation,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<BTreeMap<String, proc_macro2::TokenStream>> {
    let path_params = get_path_params(type_space, op, global_params)?;
    let query_params = get_query_params(type_space, op, global_params)?;

    let mut args: BTreeMap<String, proc_macro2::TokenStream> =
        path_params.into_iter().chain(query_params).collect();

    // Add attachments if we have a multipart request.
    if is_multipart(type_space, name, method, op)? {
        args.insert(
            "attachments".to_string(),
            quote!(Vec<crate::types::multipart::Attachment>),
        );
    }

    Ok(args)
}

fn is_multipart(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
) -> Result<bool> {
    if let Some(request_body) = get_request_body(type_space, name, method, op)? {
        if request_body.media_type.as_str() == "multipart/form-data" {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Return the request body type for the operation.
fn get_request_body(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
) -> Result<Option<RequestOrResponse>> {
    if let Some(request_body) = &op.request_body {
        // Then let's get the type for the response.
        let request_body = request_body.expand(&type_space.spec)?;

        // Iterate over all the media types and return the first request.
        for (media_type, content) in &request_body.content {
            if let Some(s) = &content.schema {
                let t = match s {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        crate::types::get_type_name_from_reference(
                            &s.reference()?,
                            &type_space.spec,
                            false,
                        )?
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        let fly_request = crate::types::get_type_name_for_schema(
                            &generate_name_for_fn_schema(name, method, s, op, "Request Body"),
                            s,
                            &type_space.spec,
                            false,
                        )?;

                        // Make sure we generate the object.
                        type_space.render_schema(&fly_request.to_string(), s)?;

                        fly_request
                    }
                };

                // Return early since we found the type.
                // We start with a comma here so it's not weird.
                return Ok(Some(RequestOrResponse {
                    media_type: media_type.to_string(),
                    type_name: t,
                }));
            }
        }
    }

    // We don't have a request body.
    // So we return nothing.
    Ok(None)
}

/// Return the request body type example for the operation.
fn get_request_body_example(
    type_space: &crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
) -> Result<Option<RequestOrResponse>> {
    if let Some(request_body) = &op.request_body {
        // Then let's get the type for the response.
        let request_body = request_body.expand(&type_space.spec)?;

        // Iterate over all the media types and return the first request.
        for (media_type, content) in &request_body.content {
            if let Some(s) = &content.schema {
                let t = match s {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        let name = crate::types::get_type_name_from_reference(
                            &s.reference()?,
                            &type_space.spec,
                            true,
                        )?;
                        crate::types::example::generate_example_rust_from_schema(
                            type_space,
                            &name.rendered()?,
                            &s.expand(&type_space.spec)?,
                            false,
                        )?
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        crate::types::example::generate_example_rust_from_schema(
                            type_space,
                            &generate_name_for_fn_schema(name, method, s, op, "Request Body"),
                            s,
                            false,
                        )?
                    }
                };

                // Return early since we found the type.
                // We start with a comma here so it's not weird.
                return Ok(Some(RequestOrResponse {
                    media_type: media_type.to_string(),
                    type_name: t,
                }));
            }
        }
    }

    // We don't have a request body.
    // So we return nothing.
    Ok(None)
}

/// Return the function arguments for the operation.
fn get_example_args(
    name: &str,
    method: &http::Method,
    type_space: &crate::types::TypeSpace,
    op: &openapiv3::Operation,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<BTreeMap<String, proc_macro2::TokenStream>> {
    let mut params = get_path_params_schema(op, &type_space.spec, global_params)?;
    params.append(&mut get_query_params_schema(
        op,
        &type_space.spec,
        global_params,
    )?);

    let mut new_params: BTreeMap<String, proc_macro2::TokenStream> = Default::default();

    for (name, (schema, parameter_data)) in params {
        // Get the type for the parameter.
        let t = match &schema {
            openapiv3::ReferenceOr::Reference { .. } => crate::types::get_type_name_from_reference(
                &schema.reference()?,
                &type_space.spec,
                true,
            )?,
            openapiv3::ReferenceOr::Item(s) => {
                let mut t_name =
                    crate::types::get_type_name_for_schema(&name, s, &type_space.spec, true)?;
                // Check if we should render the schema.
                if schema.should_render()? {
                    // Check if we already have a type with this name.
                    if let Some(rendered) = type_space
                        .types
                        .get(&t_name.strip_option()?.strip_vec()?.rendered()?)
                    {
                        if rendered != s {
                            // Update the name of the type.
                            t_name = crate::types::get_type_name_for_schema(
                                &format!("{} {}", op.get_fn_name()?, name),
                                s,
                                &type_space.spec,
                                true,
                            )?;
                        }
                    }
                }

                t_name
            }
        };

        // Let's get the example rust code for the schema.
        let mut example = crate::types::example::generate_example_rust_from_schema(
            type_space,
            &t.rendered()?,
            &schema.expand(&type_space.spec)?,
            false,
        )?;

        if !parameter_data.required
            && !example
                .rendered()?
                .starts_with("crate::types::phone_number::PhoneNumber")
        {
            example = quote!(Some(#example));
        } else if t.is_string()? {
            // Fix the parameter to be a &str, if it is a String.
            example = example.strip_to_string()?;
        }

        new_params.insert(name, example);
    }

    // Add attachments if we have a multipart request.
    if let Some(request_body) = get_request_body(&mut type_space.clone(), name, method, op)? {
        if request_body.media_type.as_str() == "multipart/form-data" {
            new_params.insert(
                "attachments".to_string(),
                quote! {
                    vec![crate::types::multipart::Attachment {
                        name: "thing".to_string(),
                        filepath: Some("myfile.json".into()),
                        content_type: Some("application/json".to_string()),
                        data: std::fs::read("myfile.json").unwrap(),
                    }]
                },
            );
        }
    }

    Ok(new_params)
}

/// Return the path params for the operation.
fn get_path_params_schema(
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<
    BTreeMap<
        String,
        (
            openapiv3::ReferenceOr<openapiv3::Schema>,
            openapiv3::ParameterData,
        ),
    >,
> {
    // Let's get the path parameters.
    let mut path_params: BTreeMap<
        String,
        (
            openapiv3::ReferenceOr<openapiv3::Schema>,
            openapiv3::ParameterData,
        ),
    > = Default::default();

    let mut parameters = op.parameters.clone();
    let mut global_params = global_params.to_vec();
    // Add in our global_params.
    parameters.append(&mut global_params);

    // Let's get the arguments for the function.
    for parameter in &parameters {
        // Get the parameter.
        let parameter = parameter.expand(spec)?;

        // Get the data for the parameter.
        // We only care about path parameters, currently.
        if let openapiv3::Parameter::Path {
            parameter_data,
            style: _,
        } = parameter
        {
            // Get the schema for the parameter.
            let schema = parameter_data.format.schema()?;

            // Add path parameter to our list.
            path_params.insert(parameter_data.name.to_string(), (schema, parameter_data));
        }
    }

    Ok(path_params)
}

/// Return the path params for the operation.
fn get_path_params(
    type_space: &mut crate::types::TypeSpace,
    op: &openapiv3::Operation,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<BTreeMap<String, proc_macro2::TokenStream>> {
    let params = get_path_params_schema(op, &type_space.spec, global_params)?;

    let mut path_params: BTreeMap<String, proc_macro2::TokenStream> = Default::default();

    for (name, (schema, parameter_data)) in params {
        // Get the type for the parameter.
        let mut t = match schema {
            openapiv3::ReferenceOr::Reference { .. } => crate::types::get_type_name_from_reference(
                &schema.reference()?,
                &type_space.spec,
                false,
            )?,
            openapiv3::ReferenceOr::Item(ref s) => {
                let mut t_name =
                    crate::types::get_type_name_for_schema(&name, s, &type_space.spec, false)?;
                // Check if we should render the schema.
                if schema.should_render()? {
                    // Check if we already have a type with this name.
                    if let Some(rendered) = type_space.types.get(&t_name.rendered()?) {
                        if rendered != s {
                            // Update the name of the type.
                            t_name = crate::types::get_type_name_for_schema(
                                &format!("{} {}", op.get_fn_name()?, name),
                                s,
                                &type_space.spec,
                                false,
                            )?;
                        }
                    }

                    type_space.render_schema(&t_name.rendered()?, s)?;
                }

                t_name
            }
        };

        // Make it an option if it's optional.
        if !parameter_data.required && !t.is_option()? {
            t = quote!(Option<#t>);
        }

        // Add path parameter to our list.
        path_params.insert(name, t.get_parameter_value()?);
    }

    Ok(path_params)
}

/// Return the query params for the operation.
fn get_query_params_schema(
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<
    BTreeMap<
        String,
        (
            openapiv3::ReferenceOr<openapiv3::Schema>,
            openapiv3::ParameterData,
        ),
    >,
> {
    // Let's get the path parameters.
    let mut query_params: BTreeMap<
        String,
        (
            openapiv3::ReferenceOr<openapiv3::Schema>,
            openapiv3::ParameterData,
        ),
    > = Default::default();

    let mut parameters = op.parameters.clone();
    let mut global_params = global_params.to_vec();
    // Add in our global_params.
    parameters.append(&mut global_params);

    // Let's get the arguments for the function.
    for parameter in &parameters {
        // Get the parameter.
        let parameter = parameter.expand(spec)?;

        // Get the data for the parameter.
        // We only care about query parameters, currently.
        if let openapiv3::Parameter::Query {
            parameter_data,
            style: _,
            allow_reserved: _,
            allow_empty_value: _,
        } = parameter
        {
            // Get the schema for the parameter.
            let schema = parameter_data.format.schema()?;

            // Add query parameter to our list.
            query_params.insert(parameter_data.name.to_string(), (schema, parameter_data));
        }
    }

    Ok(query_params)
}

/// Return the query params for the operation.
fn get_query_params(
    type_space: &mut crate::types::TypeSpace,
    op: &openapiv3::Operation,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<BTreeMap<String, proc_macro2::TokenStream>> {
    let params = get_query_params_schema(op, &type_space.spec, global_params)?;

    let mut query_params: BTreeMap<String, proc_macro2::TokenStream> = Default::default();

    for (name, (schema, parameter_data)) in params {
        // Get the type for the parameter.
        let mut t = match schema {
            openapiv3::ReferenceOr::Reference { .. } => crate::types::get_type_name_from_reference(
                &schema.reference()?,
                &type_space.spec,
                false,
            )?,
            openapiv3::ReferenceOr::Item(ref s) => {
                let mut t_name =
                    crate::types::get_type_name_for_schema(&name, s, &type_space.spec, false)?;
                // Check if we should render the schema.
                if schema.should_render()? {
                    // Check if we already have a type with this name.
                    if let Some(rendered) = type_space.types.get(
                        &t_name
                            .strip_option()?
                            .strip_vec()?
                            .rendered()?
                            .replace("crate::types::", ""),
                    ) {
                        if rendered != s {
                            // Update the name of the type.
                            t_name = crate::types::get_type_name_for_schema(
                                &format!("{} {}", op.get_fn_name()?, name),
                                s,
                                &type_space.spec,
                                false,
                            )?;
                        }
                    }

                    type_space.render_schema(&t_name.rendered()?, s)?;
                }

                t_name
            }
        };

        // Make it an option if it's optional.
        if !parameter_data.required && !t.is_option()? {
            t = quote!(Option<#t>);
        }

        // Add query parameter to our list.
        query_params.insert(name, t.get_parameter_value()?);
    }

    Ok(query_params)
}

fn clean_url_from(path_params: &BTreeMap<String, TokenStream>) -> Result<TokenStream> {
    if path_params.is_empty() {
        return Ok(quote!());
    }
    let mut clean_string = quote!();
    for (name, t) in path_params {
        let url_string = format!("{{{}}}", name);
        let cleaned_name = crate::types::clean_property_name(name);
        let name_ident = format_ident!("{}", cleaned_name);

        clean_string = if t.is_string()? {
            quote! {
                #clean_string.replace(#url_string, #name_ident)
            }
        } else {
            quote! {
                #clean_string.replace(#url_string, &format!("{}", #name_ident))
            }
        };
    }
    Ok(clean_string)
}

fn gen_query_params_code(
    query_params: &BTreeMap<String, TokenStream>,
    paginated: bool,
) -> Result<TokenStream> {
    if query_params.is_empty() || paginated {
        return Ok(quote!());
    }

    let mut required_params = Vec::new();
    let mut optional_params = Vec::new();
    for (name, t) in query_params {
        let cleaned_name = crate::types::clean_property_name(name);
        let name_ident = format_ident!("{}", cleaned_name);

        let type_text = crate::types::get_text(t)?;

        if t.is_vec()? {
            required_params.push(quote! {
               (#name, itertools::join(#name_ident, ","))
            })
        } else if !t.is_option()? {
            if type_text == "String" {
                required_params.push(quote! {
                   (#name, #name_ident)
                })
            } else {
                required_params.push(quote! {
                   (#name, format!("{}", #name_ident))
                })
            }
        } else if type_text == "Option<String>" {
            optional_params.push(quote! {
                if let Some(p) = #name_ident {
                    query_params.push((#name, p));
                }
            })
        } else if type_text == "crate::types::phone_number::PhoneNumber" {
            optional_params.push(quote! {
                if let Some(p) = #name_ident.0 {
                    query_params.push((#name, format!("{p}")));
                }
            })
        } else if t.is_option_vec()? {
            optional_params.push(quote! {
                if let Some(p) = #name_ident {
                    query_params.push((#name, itertools::join(p, ",")));
                }
            })
        } else {
            optional_params.push(quote! {
                if let Some(p) = #name_ident {
                    query_params.push((#name, format!("{}", p)));
                }
            })
        }
    }

    let is_mut = if optional_params.is_empty() {
        quote!()
    } else {
        quote!(mut)
    };
    Ok(quote! {
        let #is_mut query_params = vec![ #(#required_params),* ];
        #(#optional_params)*
        req = req.query(&query_params);
    })
}

fn generate_auth_code(opts: &crate::Opts) -> Result<TokenStream> {
    let out = if opts.token_endpoint.is_some() {
        quote!(req = req.bearer_auth(&self.client.token.read().await.access_token);)
    } else if opts.basic_auth {
        quote!(req = req.basic_auth(&self.client.username, Some(&self.client.password));)
    } else {
        quote!(req = req.bearer_auth(&self.client.token);)
    };
    Ok(out)
}

/// Return the function body for the operation.
fn get_function_body(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
    paginated: bool,
    opts: &crate::Opts,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<proc_macro2::TokenStream> {
    let path = name.trim_start_matches('/');
    let method_ident = format_ident!("{}", method.to_string());

    // Let's get the path parameters.
    let path_params = get_path_params(type_space, op, global_params)?;
    let clean_url = clean_url_from(&path_params)?;

    // Let's get the query parameters.
    let query_params = get_query_params(type_space, op, global_params)?;
    let query_params_code = gen_query_params_code(&query_params, paginated)?;

    // Get if there is a request body.
    let request_body = if let Some(request_body) = get_request_body(type_space, name, method, op)? {
        match request_body.media_type.as_str() {
            "application/json" => {
                quote! {
                    // Add the json body.
                    req = req.json(body);
                }
            }
            "application/x-www-form-urlencoded" => {
                quote! {
                    // Add the form body.
                    req = req.form(body);
                }
            }
            "application/octet-stream" => {
                quote! {
                    // Add the raw body.
                    req = req.body(body.clone());
                }
            }
            "multipart/form-data" => {
                // The json part of multipart data is sent as a file.
                if !multipart_has_body(&request_body.type_name)? {
                    // We don't add the body to the form.
                    quote! {
                        use std::convert::TryInto;
                        // Create the multipart form.
                        let mut form = reqwest::multipart::Form::new();

                        // For each of the files add them to the form.
                        for attachment in attachments {
                            form = form.part(attachment.name.clone(), attachment.try_into()?);
                        }

                        // Add to the request.
                        req = req.multipart(form);
                    }
                } else {
                    // We have an actual type.
                    quote! {
                        use std::convert::TryInto;
                        // Create the multipart form.
                        let mut form = reqwest::multipart::Form::new();
                        // Add the body to the form.

                        let mut json_part = reqwest::multipart::Part::text(serde_json::to_string(&body)?);
                        json_part = json_part.file_name(format!("{}.json", "body"));
                        json_part = json_part.mime_str("application/json")?;
                        form = form.part("body", json_part);

                        // For each of the files add them to the form.
                        for attachment in attachments {
                            form = form.part(attachment.name.clone(), attachment.try_into()?);
                        }

                        // Add to the request.
                        req = req.multipart(form);
                    }
                }
            }
            _ => {
                if request_body.type_name.is_string()? {
                    quote! {
                        // Add the raw body.
                        req = req.body(body.clone());
                    }
                } else {
                    anyhow::bail!(
                        "unsupported media type for request body: {}",
                        request_body.media_type
                    );
                }
            }
        }
    } else {
        // Do nothing.
        quote!()
    };

    // TODO: we should add the headers.

    // Get the response if there is one.
    let response = if let Some(response) = get_response_type(type_space, name, method, op)? {
        match response.media_type.as_str() {
            "application/json" => {
                quote! {
                    // Get the text for the response.
                    let text = resp.text().await.unwrap_or_default();

                    // Parse the json response.
                    // Return a human error.
                    serde_json::from_str(&text).map_err(|err| crate::types::error::Error::from_serde_error(format_serde_error::SerdeError::new(text.to_string(), err), status))
                }
            }
            "application/vnd.github.v3.object" => {
                quote! {
                    // Get the text for the response.
                    let text = resp.text().await.unwrap_or_default();

                    // Parse the json response.
                    // Return a human error.
                    serde_json::from_str(&text).map_err(|err| crate::types::error::Error::from_serde_error(format_serde_error::SerdeError::new(text.to_string(), err), status))
                }
            }
            "application/scim+json" => {
                quote! {
                    // Get the text for the response.
                    let text = resp.text().await.unwrap_or_default();

                    // Parse the json response.
                    // Return a human error.
                    serde_json::from_str(&text).map_err(|err| crate::types::error::Error::from_serde_error(format_serde_error::SerdeError::new(text.to_string(), err), status))
                }
            }
            _ => {
                if response.type_name.is_string()? {
                    quote! {
                        // Get the text for the response.
                        let text = resp.text().await?;

                        Ok(text)
                    }
                } else {
                    anyhow::bail!(
                        "unsupported media type for response: {}",
                        response.media_type
                    );
                }
            }
        }
    } else {
        // Do nothing.
        quote!(Ok(()))
    };

    let send_request = if paginated {
        quote!(
            // Build the request.
            let mut request = req.build()?;
            // Now we will modify the request to add the pagination.
            request = new_result.next_page(request)?;
            // Now we will execute the request.
            let resp = self.client.client.execute(request).await?;
        )
    } else {
        // Do nothing.
        quote!(
            // Send the request.
            let resp = req.send().await?;
        )
    };

    let auth_code = generate_auth_code(opts)?;

    Ok(quote! {
        let mut req = self.client.client.request(
            http::Method::#method_ident,
            format!("{}/{}", self.client.base_url, #path #clean_url),
        );

        // Add in our authentication.
        #auth_code

        #query_params_code

        #request_body

        #send_request

        // Get the response status.
        let status = resp.status();

        if status.is_success() {
            #response
        } else {
            // Try to decode the error.
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server{body:text.to_string(), status})
        }
    })
}

/// Check if a operation is paginated.
fn get_pagination_properties(
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<crate::types::PaginationProperties> {
    crate::types::PaginationProperties::from_operation(name, method, op, spec)
}

/// Add a function to our list of tagged functions.
fn add_fn_to_tag(
    tag_files: &mut BTreeMap<String, proc_macro2::TokenStream>,
    tag: &str,
    function: &proc_macro2::TokenStream,
) -> Result<()> {
    // Add our function to our existing tag file, or create a new one.
    if let std::collections::btree_map::Entry::Vacant(e) = tag_files.entry(tag.to_string()) {
        e.insert(function.clone());
    } else {
        let current = tag_files
            .get(tag)
            .ok_or_else(|| anyhow::anyhow!("failed to find tag file for tag `{}`", tag))?;
        tag_files.insert(
            tag.to_string(),
            quote! {
                #current

                #function
            },
        );
    }

    Ok(())
}

/// Generate example code for afunction.
fn generate_example_code_fn(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    tag: &str,
    op: &openapiv3::Operation,
    opts: &crate::Opts,
    global_params: &[openapiv3::ReferenceOr<openapiv3::Parameter>],
) -> Result<String> {
    // Get the docs.
    let docs = generate_docs(type_space, name, method, op, global_params)?;
    let docs = docs.replace('\n', "\n/// ");

    // Get the function name.
    let fn_name = op.get_fn_name()?;
    let fn_name_ident = format_ident!("{}", fn_name);
    let example_fn_name_ident = format_ident!("example_{}_{}", tag, fn_name);

    let tag_ident = format_ident!("{}", tag);

    let mut function_start = quote!();
    let mut print_result = quote!();
    if let Some(response) = get_response_type(type_space, name, method, op)? {
        let t = response.type_name;
        function_start = quote!(let result: #t = );
        print_result = quote!(println!("{:?}", result););
    }

    // Get the function args.
    let raw_args = get_example_args(name, method, type_space, op, global_params)?;
    let args = if raw_args.is_empty() {
        quote!()
    } else {
        let a = raw_args.values().map(|v| quote!(#v));
        quote!(#(#a),*,)
    };

    // Get the request body for the function if there is one.
    let request_body = if let Some(rb) = get_request_body_example(type_space, name, method, op)? {
        let t = rb.type_name;
        if is_multipart(type_space, name, method, op)? && !multipart_has_body(&t)? {
            // We don't have a request body, so we'll return nothing.
            quote!()
        } else {
            // We add the comma at the front, so it works.
            quote!(&#t)
        }
    } else {
        // We don't have a request body, so we'll return nothing.
        quote!()
    };

    let mut imports = quote!();
    if args.rendered()?.contains("::from_str(") || request_body.rendered()?.contains("::from_str(")
    {
        imports = quote!(
            use std::str::FromStr;
        );
    }

    let client_code: proc_macro2::TokenStream = generate_example_client_env(opts)
        .parse()
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let function = quote!(
        #imports

        async fn #example_fn_name_ident() -> anyhow::Result<()> {
            #client_code

            #function_start client.#tag_ident().#fn_name_ident(#args #request_body).await?;

            #print_result

            Ok(())
        }
    );

    // Let's check if this function can be paginated.
    let pagination_properties = get_pagination_properties(name, method, op, &type_space.spec)?;
    if pagination_properties.can_paginate() {
        // We need to generate the stream function as well.
        let stream_fn_name_ident = format_ident!("{}_stream", fn_name);
        let example_stream_fn_name_ident = format_ident!("example_{}_{}_stream", tag, fn_name);

        // We want all the args except for the page_token.
        let page_param_str = pagination_properties.page_param_str()?;
        let mut min_args = if raw_args.is_empty() {
            quote!()
        } else {
            let mut a = Vec::new();
            for (k, v) in raw_args.iter() {
                // Skip the next page arg.
                if k != &page_param_str {
                    a.push(quote!(#v))
                }
            }
            quote!(#(#a),*)
        };

        // Make sure we don't just have a comma.
        if min_args.rendered()? == "," {
            min_args = quote!();
        }

        let stream_function = quote!(
            use futures_util::TryStreamExt;

            async fn #example_stream_fn_name_ident() -> anyhow::Result<()> {
                #client_code

                let mut #tag_ident =  client.#tag_ident();
                let mut stream = #tag_ident.#stream_fn_name_ident(#min_args #request_body);

                // Loop over the items in the stream.
                loop {
                    match stream.try_next().await {
                        Ok(Some(item)) => {
                            // We got a result.
                            println!("{:?}", item);
                        }
                        Ok(None) => {
                            break;
                        }
                        Err(err) => {
                            // Handle the error.
                            return Err(err.into());
                        },
                    }
                }

                Ok(())
            }
        );

        // Return the formatted example.
        Ok(format!(
            r#"/// {}
{}

/// - OR -

/// Get a stream of results.
///
/// This allows you to paginate through all the items.
{}"#,
            docs,
            fmt_external_example_code(&function, opts)?,
            fmt_external_example_code(&stream_function, opts)?
        ))
    } else {
        // Return the formatted example.
        Ok(format!(
            r#"/// {}
{}"#,
            docs,
            fmt_external_example_code(&function, opts)?
        ))
    }
}

/// Generate the example client code.
pub fn generate_example_client(opts: &crate::Opts) -> String {
    if opts.token_endpoint.is_none() {
        return format!(
            r#"// Authenticate via an API token.
let client = {}::Client::new("$TOKEN");

// - OR -

// Authenticate with your token and host parsed from the environment variables:
// `{}_API_TOKEN`.
{}"#,
            opts.name,
            crate::template::get_env_variable_prefix(&opts.name),
            generate_example_client_env(opts),
        );
    }

    format!(
        r#"// Authenticate.
let client = {}::Client::new(
     String::from("client-id"),
     String::from("client-secret"),
     String::from("redirect-uri"),
     String::from("token"),
     String::from("refresh-token"),
);

// - OR -

// Authenticate with your credentials parsed from the environment variables:
// - `{}_CLIENT_ID`
// - `{}_CLIENT_SECRET`
// - `{}_REDIRECT_URI`
{}"#,
        opts.name,
        crate::template::get_env_variable_prefix(&opts.name),
        generate_example_client_env(opts),
        generate_example_client_env(opts),
        generate_example_client_env(opts),
    )
}

/// Generate the env example client code.
fn generate_example_client_env(opts: &crate::Opts) -> String {
    if opts.token_endpoint.is_none() {
        return format!(
            r#"let client = {}::Client::new_from_env();"#,
            opts.code_package_name()
        );
    }

    format!(
        r#"let client = {}::Client::new_from_env(String::from("token"), String::from("refresh-token"));"#,
        opts.code_package_name()
    )
}

/// This is a helper function that formats and fixes code for external usage, not
/// usage inside the crate.
fn fmt_external_example_code(t: &proc_macro2::TokenStream, opts: &crate::Opts) -> Result<String> {
    let rendered = crate::types::get_text_fmt(t)?;
    Ok(rendered
        .replace(
            "crate::types::",
            &format!("{}::types::", opts.code_package_name()),
        )
        .replace(
            "crate :: types :: ",
            &format!("{}::types::", opts.code_package_name()),
        ))
}

fn multipart_has_body(request_body: &proc_macro2::TokenStream) -> Result<bool> {
    Ok(!request_body.rendered()?.starts_with("bytes::Bytes"))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::types::exts::OperationExt;

    #[test]
    fn test_fn_name() {
        // Test remove stutters.
        assert_eq!(
            openapiv3::Operation {
                operation_id: Some("getThings".to_string()),
                tags: vec!["things".to_string()],
                ..Default::default()
            }
            .get_fn_name()
            .unwrap(),
            "get"
        );

        assert_eq!(
            openapiv3::Operation {
                operation_id: Some("getThingsFromZoo".to_string()),
                tags: vec!["things".to_string()],
                ..Default::default()
            }
            .get_fn_name()
            .unwrap(),
            "get_from_zoo"
        );

        assert_eq!(
            openapiv3::Operation {
                operation_id: Some("ThingFromZoo".to_string()),
                tags: vec!["things".to_string()],
                ..Default::default()
            }
            .get_fn_name()
            .unwrap(),
            "from_zoo"
        );

        assert_eq!(
            openapiv3::Operation {
                operation_id: Some("meta/info".to_string()),
                tags: vec!["things".to_string()],
                ..Default::default()
            }
            .get_fn_name()
            .unwrap(),
            "meta_info"
        );
    }
}
