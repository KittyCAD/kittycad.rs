use std::{collections::BTreeMap, fmt::Write as _};

use anyhow::Result;
use types::exts::{ParameterExt, ParameterSchemaOrContentExt, ReferenceOrExt};

/*
 * Generate a function for each Operation.
 */
pub fn generate_files(spec: &openapiv3::OpenAPI) -> Result<BTreeMap<String, String>> {
    let mut tag_files: BTreeMap<String, String> = Default::default();

    for (name, path) in spec.paths.iter() {
        let op = path.item()?;

        let mut gen = |name: &str, method: &str, op: Option<&openapiv3::Operation>| -> Result<()> {
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
            let docs = generate_docs(name, method, op)?;

            // Get the function name.
            let fn_name = get_fn_name(name, method, &tag, op)?;
            let fn_name_ident = format_ident!("{}", fn_name);

            // Get the response for the function.
            let response_type = if let Some(response) = get_response_type(op, spec)? {
                let t = response.type_name;
                quote!(#t)
            } else {
                // We don't have a response, so we'll return `()`.
                quote!(())
            };

            // Get the function args.
            let args = get_args(op, spec)?;
            // Make sure if we have args, we start with a comma.
            let args = if args.is_empty() {
                quote!()
            } else {
                quote!(,#(#args),*)
            };

            // Get the request body for the function if there is one.
            let request_body = if let Some(rb) = get_request_body(op, spec)? {
                let t = rb.type_name;
                // We add the comma at the front, so it works.
                quote!(, body: &#t)
            } else {
                // We don't have a request body, so we'll return nothing.
                quote!()
            };

            // Get the function body.
            let function_body = get_function_body(name, method, op, spec)?;

            let function = quote! {
                #[doc = #docs]
                pub async fn #fn_name_ident(&self #args #request_body) -> Result<#response_type> {
                    #function_body
                }
            };

            let mut fn_str = types::get_text_fmt(&function)?;

            // Add our function to our existing tag file, or create a new one.
            if let std::collections::btree_map::Entry::Vacant(e) = tag_files.entry(tag.to_string())
            {
                e.insert(fn_str);
            } else {
                // Add some new lines.
                fn_str = format!("\n\n{}", fn_str);

                tag_files.get_mut(&tag).unwrap().push_str(&fn_str);
            }

            Ok(())
        };

        gen(name.as_str(), "GET", op.get.as_ref())?;
        gen(name.as_str(), "PUT", op.put.as_ref())?;
        gen(name.as_str(), "POST", op.post.as_ref())?;
        gen(name.as_str(), "DELETE", op.delete.as_ref())?;
        gen(name.as_str(), "HEAD", op.head.as_ref())?;
        gen(name.as_str(), "PATCH", op.patch.as_ref())?;
        gen(name.as_str(), "TRACE", op.trace.as_ref())?;
    }

    Ok(tag_files)
}

/// Generate the docs for the given operation.
fn generate_docs(name: &str, method: &str, op: &openapiv3::Operation) -> Result<String> {
    let mut docs = if let Some(summary) = &op.summary {
        summary.to_string()
    } else {
        format!("Perform a `{}` request to `{}`.", method, name)
    };

    if let Some(description) = &op.description {
        docs.push_str("\n\n");
        docs.push_str(description);
    }

    // TODO: document the params.

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
fn get_fn_name(name: &str, method: &str, tag: &str, op: &openapiv3::Operation) -> Result<String> {
    let mut name = op
        .operation_id
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("operation `{}` `{}` has no operation_id", name, method))?
        .to_string();

    // Remove any stutters with the tag name.
    if name.starts_with(&format!("{}_", tag)) {
        name = name.trim_start_matches(&format!("{}_", tag)).to_string();
    }
    if name.ends_with(&format!("_{}", tag)) {
        name = name.trim_end_matches(&format!("_{}", tag)).to_string();
    }
    if name.contains(&format!("_{}_", tag)) {
        name = name.replace(&format!("_{}_", tag), "_");
    }

    Ok(name)
}

struct RequestOrResponse {
    media_type: String,
    type_name: proc_macro2::TokenStream,
}

/// Return the response type for the operation.
fn get_response_type(
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
            for (name, content) in &response.content {
                if let Some(s) = &content.schema {
                    let t = match s {
                        openapiv3::ReferenceOr::Reference { .. } => {
                            types::get_type_name_from_reference(&s.reference()?, spec, false)?
                        }
                        openapiv3::ReferenceOr::Item(s) => {
                            types::get_type_name_for_schema("", s, spec, false)?
                        }
                    };

                    // Return early since we found the type.
                    return Ok(Some(RequestOrResponse {
                        media_type: name.to_string(),
                        type_name: t,
                    }));
                }
            }
        }
    }

    // We couldn't find a type for the response.
    Ok(None)
}

pub trait StatusCodeExt {
    fn is_success(&self) -> bool;
}

impl StatusCodeExt for openapiv3::StatusCode {
    fn is_success(&self) -> bool {
        match self {
            openapiv3::StatusCode::Code(c) => (&200..&300).contains(&c),
            openapiv3::StatusCode::Range(r) => r.to_string().starts_with('2'),
        }
    }
}

/// Return the function arguments for the operation.
fn get_args(
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut args = vec![];

    // Let's get the arguments for the function.
    for parameter in &op.parameters {
        // Get the parameter.
        let parameter = parameter.expand(spec)?;

        // Get the data for the parameter.
        let data = (&parameter).data()?;

        let name = types::clean_property_name(&data.name);
        let name_ident = format_ident!("{}", name);
        // Get the schema for the parameter.
        let schema = data.format.schema()?;

        // Get the type for the parameter.
        let mut t = match schema {
            openapiv3::ReferenceOr::Reference { .. } => {
                types::get_type_name_from_reference(&schema.reference()?, spec, false)?
            }
            openapiv3::ReferenceOr::Item(s) => {
                types::get_type_name_for_schema("", &s, spec, false)?
            }
        };

        // Make it an option if it's optional.
        if !data.required && !types::get_text(&t)?.starts_with("Option<") {
            t = quote!(Option<#t>);
        }

        // Add the argument to the list.
        args.push(quote! {
            #name_ident: #t
        });
    }

    Ok(args)
}

/// Return the request body type for the operation.
fn get_request_body(
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<Option<RequestOrResponse>> {
    if let Some(request_body) = &op.request_body {
        // Then let's get the type for the response.
        let request_body = request_body.expand(spec)?;

        // Iterate over all the media types and return the first request.
        for (name, content) in &request_body.content {
            if let Some(s) = &content.schema {
                let t = match s {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        types::get_type_name_from_reference(&s.reference()?, spec, false)?
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        types::get_type_name_for_schema("", s, spec, false)?
                    }
                };

                // Return early since we found the type.
                // We start with a comma here so it's not weird.
                return Ok(Some(RequestOrResponse {
                    media_type: name.to_string(),
                    type_name: t,
                }));
            }
        }
    }

    // We don't have a request body.
    // So we return nothing.
    Ok(None)
}

/// Return the function body for the operation.
fn get_function_body(
    name: &str,
    method: &str,
    op: &openapiv3::Operation,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    let path = name.trim_start_matches('/');
    let method_ident = format_ident!("{}", method);

    // Get if there is a request body.
    let request_body = if let Some(request_body) = get_request_body(op, spec)? {
        match request_body.media_type.as_str() {
            "application/json" => {
                quote! {
                    // Add the json body.
                    rb = rb.json(body);
                }
            }
            "application/x-www-form-urlencoded" => {
                quote! {
                    // Add the form body.
                    rb = rb.form(body);
                }
            }
            "application/octet-stream" => {
                quote! {
                    // Add the raw body.
                    rb = rb.body(body.clone());
                }
            }
            _ => {
                anyhow::bail!(
                    "unsupported media type for request body: {}",
                    request_body.media_type
                );
            }
        }
    } else {
        // Do nothing.
        quote!()
    };

    Ok(quote! {
        let mut rb = self.client.client.request(
            http::Method::#method_ident,
            &format!("{}/{}", self.client.base_url, #path),
        );

        // Add in our authentication.
        rb = rb.bearer_auth(self.client.token);

        #request_body

        // Build the request.
        let req = rb.build()?;

        // Send the request.
        let resp = self.client.client.execute(req).await?;

        // Get the response.
        resp.json()?
    })
}
