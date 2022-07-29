//! Utilities for generating rust functions from an OpenAPI spec.

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Write as _,
};

use anyhow::Result;

use crate::types::exts::{
    ParameterSchemaOrContentExt, ReferenceOrExt, StatusCodeExt, TokenStreamExt,
};

/// Generate functions for each path operation.
pub fn generate_files(
    type_space: &mut crate::types::TypeSpace,
    spec: &openapiv3::OpenAPI,
    opts: &crate::Opts,
) -> Result<(
    BTreeMap<String, proc_macro2::TokenStream>,
    openapiv3::OpenAPI,
)> {
    let mut tag_files: BTreeMap<String, proc_macro2::TokenStream> = Default::default();

    // Make a spec we can modify for the docs.
    let mut new_spec = spec.clone();

    for (name, path) in spec.paths.iter() {
        let op = path.item()?;

        let mut new_path = op.clone();

        let mut gen = |name: &str,
                       method: &http::Method,
                       op: Option<&openapiv3::Operation>|
         -> Result<()> {
            // Ensure we have an operation for this path and method, otherwise return early.
            let op = if let Some(op) = op {
                op
            } else {
                return Ok(());
            };

            let tag =
                crate::clean_tag_name(op.tags.first().ok_or_else(|| {
                    anyhow::anyhow!("operation `{}` `{}` has no tags", name, method)
                })?);

            // Get the docs.
            let docs = generate_docs(name, method, op, spec)?;

            // Get the function name.
            let fn_name = get_fn_name(name, method, &tag, op)?;
            let fn_name_ident = format_ident!("{}", fn_name);

            // Get the response for the function.
            let response_type =
                if let Some(response) = get_response_type(type_space, name, method, op, spec)? {
                    let t = response.type_name;
                    quote!(#t)
                } else {
                    // We don't have a response, so we'll return `()`.
                    quote!(())
                };

            // Get the function args.
            let raw_args = get_args(op, spec)?;
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
            let request_body =
                if let Some(rb) = get_request_body(type_space, name, method, op, spec)? {
                    let t = rb.type_name;
                    // We add the comma at the front, so it works.
                    quote!(, body: &#t)
                } else {
                    // We don't have a request body, so we'll return nothing.
                    quote!()
                };

            // Get the function body.
            let function_body = get_function_body(type_space, name, method, op, spec, false)?;

            let example_code_fn =
                generate_example_code_fn(type_space, name, method, &tag, op, spec, opts)?;
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
            let mut new_operation = op.clone();
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
            let pagination_properties = get_pagination_properties(name, method, op, spec)?;
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
                    get_function_body(type_space, name, method, op, spec, true)?;

                let item_type = pagination_properties.item_type(false)?;

                let function = quote! {
                    #[doc = #docs]
                    #[tracing::instrument]
                    pub fn #stream_fn_name_ident<'a>(&'a self #min_args #request_body) -> impl futures::Stream<Item = Result<#item_type, crate::types::error::Error>> + Unpin + '_  {
                        use futures::{StreamExt, TryFutureExt, TryStreamExt};
                        use crate::types::paginate::Pagination;

                        // Get the result from our main function.
                        self.#fn_name_ident(#inner_args #body_arg)
                            .map_ok(move |result| {
                                let items = futures::stream::iter(result.items().into_iter().map(Ok));

                                // Get the next pages.
                                let next_pages = futures::stream::try_unfold(
                                    result,
                                    move |new_result| async move {
                                        if new_result.has_more_pages() {
                                            // Get the next page, we modify the request directly,
                                            // so that if we want to generate an API that uses
                                            // Link headers or any other weird shit it works.
                                            async {
                                                #paginated_function_body
                                            }.map_ok(|result: #response_type| {
                                                Some((futures::stream::iter(
                                                        result.items().into_iter().map(Ok),
                                                    ),
                                                    result,
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

            // Update our api spec with the new functions.
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

        gen(name.as_str(), &http::Method::GET, op.get.as_ref())?;
        gen(name.as_str(), &http::Method::PUT, op.put.as_ref())?;
        gen(name.as_str(), &http::Method::POST, op.post.as_ref())?;
        gen(name.as_str(), &http::Method::DELETE, op.delete.as_ref())?;
        gen(name.as_str(), &http::Method::HEAD, op.head.as_ref())?;
        gen(name.as_str(), &http::Method::PATCH, op.patch.as_ref())?;
        gen(name.as_str(), &http::Method::TRACE, op.trace.as_ref())?;
    }

    Ok((tag_files, new_spec))
}

/// Generate the docs for the given operation.
fn generate_docs(
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<String> {
    let mut docs = if let Some(summary) = &op.summary {
        summary.to_string()
    } else {
        format!("Perform a `{}` request to `{}`.", method, name)
    };

    if let Some(description) = &op.description {
        docs.push_str("\n\n");
        docs.push_str(description);
    }

    // Document the params.
    let mut params = get_path_params_schema(op, spec)?;
    params.append(&mut get_query_params_schema(op, spec)?);

    let params_types = get_args(op, spec)?;

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
            param_docs.push_str(": ");
            param_docs.push_str(description);
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

/// Return the function name for the operation.
fn get_fn_name(
    name: &str,
    method: &http::Method,
    tag: &str,
    op: &openapiv3::Operation,
) -> Result<String> {
    let mut name = op
        .operation_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("operation `{}` `{}` has no operation_id", name, method))?
        .to_string();

    // Convert to snake case.
    name = inflector::cases::snakecase::to_snake_case(&name);

    // Remove any stutters with the tag name.
    name = remove_stutters(&name, tag);
    // Remove any stutters with the singular tag name.
    name = remove_stutters(&name, &singular(tag));

    Ok(name)
}

/// Return the singular version of a string (if it plural).
fn singular(s: &str) -> String {
    if let Some(b) = s.strip_suffix('s') {
        return b.to_string();
    }

    s.to_string()
}

/// Remove any stutters with a string.
fn remove_stutters(whole: &str, s: &str) -> String {
    let mut whole = whole.to_string();
    if whole.starts_with(&format!("{}_", s)) {
        whole = whole.trim_start_matches(&format!("{}_", s)).to_string();
    }
    if whole.ends_with(&format!("_{}", s)) {
        whole = whole.trim_end_matches(&format!("_{}", s)).to_string();
    }
    if whole.contains(&format!("_{}_", s)) {
        whole = whole.replace(&format!("_{}_", s), "_");
    }

    whole
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
    spec: &openapiv3::OpenAPI,
) -> Result<Option<RequestOrResponse>> {
    for (status_code, response) in &op.responses.responses {
        // We only care if the response is a success since this is for the function
        // to return upon success.
        if status_code.is_success() {
            // Then let's get the type for the response.
            let response = response.expand(spec)?;

            // Iterate over all the media types and return the first response.
            for (media_type, content) in &response.content {
                if let Some(s) = &content.schema {
                    let t = match s {
                        openapiv3::ReferenceOr::Reference { .. } => {
                            crate::types::get_type_name_from_reference(
                                &s.reference()?,
                                spec,
                                false,
                            )?
                        }
                        openapiv3::ReferenceOr::Item(s) => {
                            let on_the_fly_type = crate::types::get_type_name_for_schema(
                                &generate_name_for_fn_schema(name, method, s, op, "Response"),
                                s,
                                spec,
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
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<BTreeMap<String, proc_macro2::TokenStream>> {
    let path_params = get_path_params(op, spec)?;
    let query_params = get_query_params(op, spec)?;

    Ok(path_params
        .into_iter()
        .chain(query_params.into_iter())
        .collect())
}

/// Return the request body type for the operation.
fn get_request_body(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<Option<RequestOrResponse>> {
    if let Some(request_body) = &op.request_body {
        // Then let's get the type for the response.
        let request_body = request_body.expand(spec)?;

        // Iterate over all the media types and return the first request.
        for (media_type, content) in &request_body.content {
            if let Some(s) = &content.schema {
                let t = match s {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        crate::types::get_type_name_from_reference(&s.reference()?, spec, false)?
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        let fly_request = crate::types::get_type_name_for_schema(
                            &generate_name_for_fn_schema(name, method, s, op, "Request Body"),
                            s,
                            spec,
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
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<Option<RequestOrResponse>> {
    if let Some(request_body) = &op.request_body {
        // Then let's get the type for the response.
        let request_body = request_body.expand(spec)?;

        // Iterate over all the media types and return the first request.
        for (media_type, content) in &request_body.content {
            if let Some(s) = &content.schema {
                let t = match s {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        let name = crate::types::get_type_name_from_reference(
                            &s.reference()?,
                            spec,
                            true,
                        )?;
                        crate::types::example::generate_example_rust_from_schema(
                            &name.rendered()?,
                            &s.expand(spec)?,
                            spec,
                        )?
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        crate::types::example::generate_example_rust_from_schema(
                            &generate_name_for_fn_schema(name, method, s, op, "Request Body"),
                            s,
                            spec,
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
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<BTreeMap<String, proc_macro2::TokenStream>> {
    let mut params = get_path_params_schema(op, spec)?;
    params.append(&mut get_query_params_schema(op, spec)?);

    let mut new_params: BTreeMap<String, proc_macro2::TokenStream> = Default::default();

    for (name, (schema, parameter_data)) in params {
        // Get the type for the parameter.
        let t = match &schema {
            openapiv3::ReferenceOr::Reference { .. } => {
                crate::types::get_type_name_from_reference(&schema.reference()?, spec, true)?
            }
            openapiv3::ReferenceOr::Item(s) => {
                crate::types::get_type_name_for_schema(&name, s, spec, true)?
            }
        };

        // Let's get the example rust code for the schema.
        let mut example = crate::types::example::generate_example_rust_from_schema(
            &t.rendered()?,
            &schema.expand(spec)?,
            spec,
        )?;

        if !parameter_data.required {
            example = quote!(Some(#example));
        } else if t.is_string()? {
            // Fix the parameter to be a &str, if it is a String.
            example = example.strip_to_string()?;
        }

        new_params.insert(name, example);
    }

    Ok(new_params)
}

/// Return the path params for the operation.
fn get_path_params_schema(
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
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

    // Let's get the arguments for the function.
    for parameter in &op.parameters {
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
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<BTreeMap<String, proc_macro2::TokenStream>> {
    let params = get_path_params_schema(op, spec)?;

    let mut path_params: BTreeMap<String, proc_macro2::TokenStream> = Default::default();

    for (name, (schema, parameter_data)) in params {
        // Get the type for the parameter.
        let mut t = match schema {
            openapiv3::ReferenceOr::Reference { .. } => {
                crate::types::get_type_name_from_reference(&schema.reference()?, spec, false)?
            }
            openapiv3::ReferenceOr::Item(s) => {
                crate::types::get_type_name_for_schema(&name, &s, spec, false)?
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
    // Let's get the arguments for the function.
    for parameter in &op.parameters {
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
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<BTreeMap<String, proc_macro2::TokenStream>> {
    let params = get_query_params_schema(op, spec)?;

    let mut query_params: BTreeMap<String, proc_macro2::TokenStream> = Default::default();

    for (name, (schema, parameter_data)) in params {
        // Get the type for the parameter.
        let mut t = match schema {
            openapiv3::ReferenceOr::Reference { .. } => {
                crate::types::get_type_name_from_reference(&schema.reference()?, spec, false)?
            }
            openapiv3::ReferenceOr::Item(s) => {
                crate::types::get_type_name_for_schema(&name, &s, spec, false)?
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

/// Return the function body for the operation.
fn get_function_body(
    type_space: &mut crate::types::TypeSpace,
    name: &str,
    method: &http::Method,
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
    paginated: bool,
) -> Result<proc_macro2::TokenStream> {
    let path = name.trim_start_matches('/');
    let method_ident = format_ident!("{}", method.to_string());

    // Let's get the path parameters.
    let path_params = get_path_params(op, spec)?;
    let clean_url = if !path_params.is_empty() {
        let mut clean_string = quote!();
        for (name, t) in &path_params {
            let url_string = format!("{{{}}}", name);
            let cleaned_name = crate::types::clean_property_name(name);
            let name_ident = format_ident!("{}", cleaned_name);

            clean_string = if t.is_string()? {
                quote! {
                    #clean_string.replace(#url_string, &#name_ident)
                }
            } else {
                quote! {
                    #clean_string.replace(#url_string, &format!("{}", #name_ident))
                }
            };
        }
        clean_string
    } else {
        quote!()
    };

    // Let's get the query parameters.
    let query_params = get_query_params(op, spec)?;
    let query_params_code = if !query_params.is_empty() && !paginated {
        let mut array = Vec::new();
        for (name, t) in &query_params {
            let cleaned_name = crate::types::clean_property_name(name);
            let name_ident = format_ident!("{}", cleaned_name);

            let type_text = crate::types::get_text(t)?;

            if !t.is_option()? {
                if type_text == "String" {
                    array.push(quote! {
                       query_params.push((#name, #name_ident));
                    })
                } else {
                    array.push(quote! {
                       query_params.push((#name, format!("{}", #name_ident)));
                    })
                }
            } else if type_text == "Option<String>" {
                array.push(quote! {
                    if let Some(p) = #name_ident {
                        query_params.push((#name, p));
                    }
                })
            } else {
                array.push(quote! {
                    if let Some(p) = #name_ident {
                        query_params.push((#name, format!("{}", p)));
                    }
                })
            }
        }

        quote! {
            let mut query_params = Vec::new();
            #(#array)*
            req = req.query(&query_params);
        }
    } else {
        quote!()
    };

    // Get if there is a request body.
    let request_body =
        if let Some(request_body) = get_request_body(type_space, name, method, op, spec)? {
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
    let response = if let Some(response) = get_response_type(type_space, name, method, op, spec)? {
        match response.media_type.as_str() {
            "application/json" => {
                quote! {
                    // Get the text for the response.
                    let text = resp.text().await.unwrap_or_default();

                    // Parse the json response.
                    // Return a human error.
                    serde_json::from_str(&text).map_err(|err| crate::types::error::Error::from_serde_error(format_serde_error::SerdeError::new(text.to_string(), err), status).into())
                }
            }
            "application/vnd.github.v3.object" => {
                quote! {
                    // Get the text for the response.
                    let text = resp.text().await.unwrap_or_default();

                    // Parse the json response.
                    // Return a human error.
                    serde_json::from_str(&text).map_err(|err| crate::types::error::Error::from_serde_error(format_serde_error::SerdeError::new(text.to_string(), err), status).into())
                }
            }
            "application/scim+json" => {
                quote! {
                    // Get the text for the response.
                    let text = resp.text().await.unwrap_or_default();

                    // Parse the json response.
                    // Return a human error.
                    serde_json::from_str(&text).map_err(|err| crate::types::error::Error::from_serde_error(format_serde_error::SerdeError::new(text.to_string(), err), status).into())
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

    Ok(quote! {
        let mut req = self.client.client.request(
            http::Method::#method_ident,
            &format!("{}/{}", self.client.base_url, #path#clean_url),
        );

        // Add in our authentication.
        req = req.bearer_auth(&self.client.token);

        #query_params_code

        #request_body

        #send_request

        // Get the response status.
        let status = resp.status();

        if status.is_success() {
            #response
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
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
    spec: &openapiv3::OpenAPI,
    opts: &crate::Opts,
) -> Result<String> {
    // Get the docs.
    let docs = generate_docs(name, method, op, spec)?;
    let docs = docs.replace('\n', "\n/// ");

    // Get the function name.
    let fn_name = get_fn_name(name, method, tag, op)?;
    let fn_name_ident = format_ident!("{}", fn_name);
    let example_fn_name_ident = format_ident!("example_{}_{}", tag, fn_name);

    let tag_ident = format_ident!("{}", tag);

    let mut function_start = quote!();
    let mut print_result = quote!();
    if let Some(response) = get_response_type(type_space, name, method, op, spec)? {
        let t = response.type_name;
        function_start = quote!(let result: #t = );
        print_result = quote!(println!("{:?}", result););
    }

    // Get the function args.
    let raw_args = get_example_args(op, spec)?;
    let args = if raw_args.is_empty() {
        quote!()
    } else {
        let a = raw_args.iter().map(|(_k, v)| quote!(#v));
        quote!(#(#a),*,)
    };

    // Get the request body for the function if there is one.
    let request_body = if let Some(rb) = get_request_body_example(name, method, op, spec)? {
        let t = rb.type_name;
        // We add the comma at the front, so it works.
        quote!(&#t)
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
    let pagination_properties = get_pagination_properties(name, method, op, spec)?;
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
    format!(
        r#"// Authenticate via an API token.
let client = {}::Client::new("$TOKEN");

// - OR -

// Authenticate with your token and host parsed from the environment variables:
// `{}`.
{}"#,
        opts.name,
        crate::template::get_token_env_variable(&opts.name),
        generate_example_client_env(opts),
    )
}

/// Generate the env example client code.
fn generate_example_client_env(opts: &crate::Opts) -> String {
    format!(r#"let client = {}::Client::new_from_env();"#, opts.name)
}

/// This is a helper function that formats and fixes code for external usage, not
/// usage inside the crate.
fn fmt_external_example_code(t: &proc_macro2::TokenStream, opts: &crate::Opts) -> Result<String> {
    let rendered = crate::types::get_text_fmt(t)?;
    Ok(rendered.replace("crate::types::", &format!("{}::types::", opts.name)))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_fn_name() {
        // Test remove stutters.
        assert_eq!(
            super::get_fn_name(
                "/foo/bar",
                &http::Method::GET,
                "things",
                &openapiv3::Operation {
                    operation_id: Some("getThings".to_string()),
                    ..Default::default()
                }
            )
            .unwrap(),
            "get"
        );

        assert_eq!(
            super::get_fn_name(
                "/foo/bar",
                &http::Method::GET,
                "things",
                &openapiv3::Operation {
                    operation_id: Some("getThingsFromZoo".to_string()),
                    ..Default::default()
                }
            )
            .unwrap(),
            "get_from_zoo"
        );

        assert_eq!(
            super::get_fn_name(
                "/foo/bar",
                &http::Method::GET,
                "things",
                &openapiv3::Operation {
                    operation_id: Some("ThingFromZoo".to_string()),
                    ..Default::default()
                }
            )
            .unwrap(),
            "from_zoo"
        );

        assert_eq!(
            super::get_fn_name(
                "/foo/bar",
                &http::Method::GET,
                "things",
                &openapiv3::Operation {
                    operation_id: Some("meta/info".to_string()),
                    ..Default::default()
                }
            )
            .unwrap(),
            "meta_info"
        );
    }
}
