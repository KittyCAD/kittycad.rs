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

            // Add this to our map of functions based on the tag name.
            // TODO: actually generate the function.
            tag_files.insert(tag, String::new());

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
    let mut docs = String::new();

    Ok(docs)
}
