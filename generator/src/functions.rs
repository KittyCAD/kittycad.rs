use std::collections::BTreeMap;

use anyhow::Result;
use types::exts::ReferenceOrExt;

/*
 * Generate a function for each Operation.
 */
pub fn generate_files(api: &openapiv3::OpenAPI) -> Result<BTreeMap<String, String>> {
    let mut tag_files: BTreeMap<String, String> = Default::default();

    for (name, path) in api.paths.iter() {
        let op = path.item()?;

        let mut gen = |name: &str, method: &str, op: Option<&openapiv3::Operation>| -> Result<()> {
            // Ensure we have an operation for this path and method, otherwise return early.
            let op = if let Some(op) = op {
                op
            } else {
                return Ok(());
            };

            let tag = crate::clean_tag_name(op.tags.first().ok_or(anyhow::anyhow!(
                "operation `{}` `{}` has no tags",
                name,
                method
            ))?);

            let docs = generate_docs(name, method, op)?;

            let fn_name = get_fn_name(name, method, &tag, op)?;
            let fn_name_ident = format_ident!("{}", fn_name);

            /* let function = quote! {
                #docs
                pub fn #name(#(#(#args),*),*) -> #return_type {
                    #body
                }
            };*/
            let function = quote! {
                #[doc = #docs]
                pub fn #fn_name_ident(&self) -> Result<()> {
                    Ok(())
                }
            };

            let mut fn_str = types::get_text_fmt(&function)?;

            // Add our function to our existing tag file, or create a new one.
            if tag_files.contains_key(&tag) {
                // Add some new lines.
                fn_str = format!("\n\n{}", fn_str);

                tag_files.get_mut(&tag).unwrap().push_str(&fn_str);
            } else {
                tag_files.insert(tag, fn_str);
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
            docs.push_str(&format!(
                "See <{}|{}> for more information.",
                external_docs.url, description
            ));
        } else {
            docs.push_str(&format!(
                "See <{}> for more information.",
                external_docs.url
            ));
        }
    }

    Ok(docs)
}

/// Return the function name for the operation.
fn get_fn_name(name: &str, method: &str, tag: &str, op: &openapiv3::Operation) -> Result<String> {
    let mut name = op
        .operation_id
        .as_ref()
        .ok_or(anyhow::anyhow!(
            "operation `{}` `{}` has no operation_id",
            name,
            method
        ))?
        .to_string();

    // Remove any stutters with the tag name.
    if name.starts_with(&format!("{}_", tag)) {
        name = name.trim_start_matches(&format!("{}_", tag)).to_string();
    }
    if name.ends_with(&format!("_{}", tag)) {
        name = name.trim_end_matches(&format!("_{}", tag)).to_string();
    }
    if name.contains(&format!("_{}_", tag)) {
        name = name.replace(&format!("_{}_", tag), "_").to_string();
    }

    Ok(name)
}
