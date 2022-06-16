use std::collections::{BTreeMap, HashMap};

use anyhow::{bail, Result};
use inflector::cases::snakecase::to_snake_case;

use crate::{
    clean_fn_name, clean_name, get_parameter_data, oid_to_object_name, path_to_operation_id,
    struct_name, template::parse, ExtractJsonMediaType, ParameterDataExt, ReferenceOrExt, TypeId,
    TypeSpace,
};

/*
 * Generate a function for each Operation.
 */
pub fn generate_files(
    api: &openapiv3::OpenAPI,
    ts: &mut TypeSpace,
    parameters: &BTreeMap<String, &openapiv3::Parameter>,
) -> Result<(BTreeMap<String, String>, openapiv3::OpenAPI)> {
    let mut new_api = api.clone();

    let mut tag_files: BTreeMap<String, String> = Default::default();

    let mut fn_names: Vec<String> = Default::default();
    for (pn, path) in api.paths.iter() {
        let op = path.item().unwrap_or_else(|e| panic!("bad path: {}", e));

        let mut new_op = op.clone();

        let mut gen = |p: &str,
                       m: &str,
                       o: Option<&openapiv3::Operation>,
                       new_op: &mut openapiv3::PathItem|
         -> Result<()> {
            let o = if let Some(o) = o {
                o
            } else {
                return Ok(());
            };

            let op_id = if o.operation_id.is_none() {
                // Make the operation id, the function.
                path_to_operation_id(pn, m)
            } else {
                o.operation_id.as_ref().unwrap().to_string()
            };
            let od = to_snake_case(&op_id);

            // Make sure we have exactly 1 tag. This likely needs to change in the
            // future but for now it seems fairly consistent.
            let mut tags = o.tags.clone();
            if tags.is_empty() {
                // This "x-tags" bullshit is for Gusto.
                if let Some(x) = o.extensions.get("x-tags") {
                    let xtags: Vec<String> = serde_json::from_value(x.clone()).unwrap();
                    tags = xtags;
                }
            }
            if tags.is_empty() {
                // If we still have no tags...., parse it from
                // the path.
                let split = pn.trim_start_matches('/').split('/');
                let vec = split.collect::<Vec<&str>>();

                tags.push(vec.first().unwrap().to_string());
            }
            let tag = to_snake_case(&clean_name(tags.first().unwrap()));

            let oid = clean_fn_name(&od, &tag);

            let mut out = String::new();
            if let Some(o) = tag_files.get(&tag) {
                out = o.to_string();
            }

            let mut a = |s: &str| {
                out.push_str(s);
                out.push('\n');
            };

            let mut print_fn = |docs: &str,
                                bounds: &Vec<String>,
                                fn_params_str: &Vec<String>,
                                body_param: &Option<String>,
                                response_type: &str,
                                template: &str,
                                fn_inner: &str,
                                fn_name: &str| {
                // Print the function docs.
                a(docs);

                if bounds.is_empty() {
                    a(&format!("pub async fn {}(", fn_name,));
                } else {
                    a(&format!("pub async fn {}<{}>(", fn_name, bounds.join(", ")));
                }
                a("&self,");

                if !fn_params_str.is_empty() {
                    a(&fn_params_str.join(" "));
                }

                if let Some(bp) = &body_param {
                    a(&format!("body: {}", bp));
                }

                a(&format!(") -> Result<{}> {{", response_type));

                a(template);

                a(fn_inner);

                a("}");
                a("");
            };

            let docs = get_fn_docs(o, m, p, parameters, ts)?;

            let mut bounds: Vec<String> = Vec::new();

            let (body_param, body_func) = if let Some(b) = &o.request_body {
                if let Ok(b) = b.item() {
                    if b.is_binary()? {
                        bounds.push("B: Into<reqwest::Body>".to_string());
                        (Some("B".to_string()), Some("body".to_string()))
                    } else {
                        let (ct, mt) = b.content.first().unwrap();
                        if ct == "application/json"
                            || ct == "application/octet-stream"
                            || ct.contains("application/json")
                        {
                            if let Some(s) = &mt.schema {
                                let object_name = format!("{} request", oid_to_object_name(&od));
                                let id = ts.select(Some(&object_name), s, "")?;
                                let et = ts.id_to_entry.get(&id).unwrap();
                                if let crate::TypeDetails::Object(p, _) = &et.details {
                                    // We want to make sure we actally have properties
                                    // in our object.
                                    if p.is_empty() {
                                        (None, None)
                                    } else {
                                        let rt = ts.render_type(&id, false)?;
                                        if rt.starts_with("Vec") {
                                            (
                                                Some(format!(
                                                    "&[{}]",
                                                    rt.trim_start_matches("Vec<")
                                                        .trim_end_matches('>')
                                                )),
                                                Some("json".to_string()),
                                            )
                                        } else {
                                            (Some(format!("&{}", rt)), Some("json".to_string()))
                                        }
                                    }
                                } else {
                                    let rt = ts.render_type(&id, false)?;
                                    if rt.starts_with("Vec") {
                                        (
                                            Some(format!(
                                                "&[{}]",
                                                rt.trim_start_matches("Vec<").trim_end_matches('>')
                                            )),
                                            Some("json".to_string()),
                                        )
                                    } else {
                                        (Some(format!("&{}", rt)), Some("json".to_string()))
                                    }
                                }
                            } else {
                                (None, None)
                            }
                        } else if ct == "multipart/form-data" {
                            println!("got multipart/formdata for {}", oid);
                            // Skip it for now.
                            // TODO: fix this later.
                            (None, None)
                        } else if ct == "application/x-www-form-urlencoded" {
                            println!("got application/x-www-form-urlencoded for {}", oid);
                            // Skip it for now.
                            // TODO: fix this later.
                            (None, None)
                        } else if let Some(s) = &mt.schema {
                            let tid = ts.select(None, s, "")?;
                            let rt = ts.render_type(&tid, false)?;
                            bounds.push("T: Into<reqwest::Body>".to_string());
                            if rt == "String" {
                                (Some("T".to_string()), Some("body".to_string()))
                            } else {
                                (Some(rt), Some("body".to_string()))
                            }
                        } else {
                            (None, None)
                        }
                    }
                } else if let openapiv3::ReferenceOr::Reference { reference } = b {
                    // We must have had a reference.
                    let object_name = format!("{} request", oid_to_object_name(&od));
                    let id = ts.select_ref(Some(&clean_name(&object_name)), reference)?;
                    let rt = ts.render_type(&id, false)?;
                    (Some(format!("&{}", rt)), Some("json".to_string()))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            /*
             * Get the function parameters.
             */
            let (fn_params_str, query_params) = get_fn_params(ts, o, false)?;

            /*
             * Generate the URL for the request.
             */
            let tmp = parse(p)?;
            let template = tmp.compile(query_params);

            /*
             * Get the response type.
             */
            let (mut response_type, _tid, inner_response_type, pagination_property) =
                get_response_type(&od, ts, o)?;

            // We shouldn't ever have an optional response type, thats just annoying.
            if response_type.starts_with("Option<") {
                response_type = response_type
                    .trim_start_matches("Option<")
                    .trim_end_matches('>')
                    .to_string();
            }

            let fn_inner = get_fn_inner(
                &oid,
                m,
                &body_func,
                &response_type,
                &inner_response_type,
                &pagination_property,
                false,
            )?;

            // Get the function without the function inners.
            // This is specifically for Ramp.
            // We do this directly before we print the other function.
            let mut frt = response_type.to_string();
            if !inner_response_type.is_empty() {
                frt = inner_response_type.to_string();
            }

            let mut fn_name = oid
                .trim_start_matches(&tag)
                .trim_start_matches('_')
                .to_string();
            if !frt.starts_with("Vec<")
                && !frt.ends_with("Response")
                && !frt.ends_with("Summary")
                && http::Method::GET == m
                && !fn_name.ends_with("address")
                && !fn_name.ends_with("has")
                && !fn_name.ends_with("access")
            {
                // Make sure we don't add an s where we don't need one.
                // Don't make a function plural where it is not needed.
                fn_name = fn_name.trim_end_matches('s').to_string();
            } else if frt.starts_with("Vec<") && fn_name == "get" {
                fn_name = "get_page".to_string()
            }

            // Fix if we somehow created a function that is actually a keyword.
            if fn_name == "move" {
                fn_name = "mv".to_string();
            }

            // Do this right before printing. Check if we already have this function name.
            // This will ensure we don't have any duplicates.
            if fn_names.contains(&(fn_name.clone() + &tag)) {
                fn_name = format!("{}_{}", fn_name, tag);
            }
            fn_names.push(fn_name.clone() + &tag);

            // Print our standard function.
            print_fn(
                &docs,
                &bounds,
                &fn_params_str,
                &body_param,
                &frt,
                &template,
                &fn_inner,
                &fn_name,
            );

            // Add the docs to our spec.
            let mut new_operation = o.clone();

            let mut docs_params: Vec<String> = Vec::new();
            for param in fn_params_str {
                let split = param.split(':').collect::<Vec<&str>>();
                docs_params.push(split[0].to_string());
            }
            if body_param.is_some() {
                docs_params.push("body".to_string());
            }
            let mut example: HashMap<String, String> = HashMap::new();
            if frt == "()" {
                example.insert(
                    "example".to_string(),
                    format!(
                        "{}\nclient.{}().{}({}).await?;",
                        docs,
                        tag,
                        fn_name,
                        docs_params.join(", ")
                    ),
                );
            } else {
                example.insert(
                    "example".to_string(),
                    format!(
                        "{}\nlet {} = client.{}().{}({}).await?;",
                        docs,
                        to_snake_case(&frt).trim_start_matches("crate_types_"),
                        tag,
                        fn_name,
                        docs_params.join(", ")
                    ),
                );
            }
            example.insert(
                "libDocsLink".to_string(),
                format!(
                    "https://docs.rs/kittycad/latest/kittycad/{}/struct.{}.html#method.{}",
                    tag,
                    struct_name(&tag),
                    fn_name
                ),
            );

            // If we are returning a list of things and we have page, etc as
            // params, let's get all the pages.
            if frt.starts_with("Vec<") && http::Method::GET == m && !pagination_property.is_empty()
            {
                let docs = get_fn_docs_all(
                    o,
                    m,
                    p,
                    oid.trim_start_matches(&tag).trim_start_matches('_'),
                )?;

                let (fn_params_str, query_params) = get_fn_params(ts, o, true)?;

                let tmp = parse(p)?;
                let template = tmp.compile(query_params);

                let fn_inner = get_fn_inner(
                    &oid,
                    m,
                    &body_func,
                    &response_type,
                    &inner_response_type,
                    &pagination_property,
                    true,
                )?;

                let mut fn_name = oid
                    .replace("_get_", "_get_all_")
                    .replace("_list_", "_list_all_")
                    .trim_start_matches(&tag)
                    .trim_start_matches('_')
                    .to_string();

                if fn_name == "list" {
                    fn_name = "list_all".to_string();
                } else if fn_name == "get" {
                    fn_name = "get_all".to_string();
                } else if fn_name.starts_with("get_") && !fn_name.starts_with("get_all") {
                    fn_name = fn_name.replace("get_", "get_all_");
                } else if fn_name.starts_with("list_") && !fn_name.starts_with("list_all") {
                    fn_name = fn_name.replace("list_", "list_all_");
                } else if !fn_name.contains("get")
                    && !fn_name.contains("get_all")
                    && !fn_name.contains("list")
                    && !fn_name.contains("list_all")
                {
                    fn_name = format!("get_all_{}", fn_name);
                }

                // Do this right before printing. Check if we already have this function name.
                // This will ensure we don't have any duplicates.
                if fn_names.contains(&(fn_name.clone() + &tag)) {
                    fn_name = format!("{}_all", fn_name);
                }
                fn_names.push(fn_name.clone() + &tag);

                // Now let's print the new function.
                print_fn(
                    &docs,
                    &bounds,
                    &fn_params_str,
                    &body_param,
                    &frt,
                    &template,
                    &fn_inner,
                    &fn_name,
                );

                if let Some(index) = docs_params.iter().position(|x| *x == "page_token") {
                    docs_params.remove(index);
                }
                if let Some(index) = docs_params.iter().position(|x| *x == "limit") {
                    docs_params.remove(index);
                }

                example.insert(
                    "example".to_string(),
                    format!(
                        "{}\n\n// - OR -\n\n{}\nlet {} = client.{}().{}({}).await?;",
                        example.get("example").unwrap(),
                        docs,
                        to_snake_case(&frt).trim_start_matches("crate_types_"),
                        tag,
                        fn_name,
                        docs_params.join(", ")
                    ),
                );
            }

            new_operation
                .extensions
                .insert("x-rust".to_string(), serde_json::json!(example));
            match m {
                "GET" => {
                    new_op.get = Some(new_operation);
                }
                "POST" => {
                    new_op.post = Some(new_operation);
                }
                "PUT" => {
                    new_op.put = Some(new_operation);
                }
                "PATCH" => {
                    new_op.patch = Some(new_operation);
                }
                "DELETE" => {
                    new_op.delete = Some(new_operation);
                }
                _ => {}
            }
            new_api
                .paths
                .insert(pn.to_string(), openapiv3::ReferenceOr::Item(new_op.clone()));

            // Add this to our map of functions based on the tag name.
            tag_files.insert(tag, out.to_string());

            Ok(())
        };

        gen(pn.as_str(), "GET", op.get.as_ref(), &mut new_op)?;
        gen(pn.as_str(), "PUT", op.put.as_ref(), &mut new_op)?;
        gen(pn.as_str(), "POST", op.post.as_ref(), &mut new_op)?;
        gen(pn.as_str(), "DELETE", op.delete.as_ref(), &mut new_op)?;
        gen(pn.as_str(), "HEAD", op.head.as_ref(), &mut new_op)?;
        gen(pn.as_str(), "PATCH", op.patch.as_ref(), &mut new_op)?;
        gen(pn.as_str(), "TRACE", op.trace.as_ref(), &mut new_op)?;
    }

    Ok((tag_files, new_api))
}

fn get_response_type_from_object(
    od: &str,
    ts: &mut TypeSpace,
    s: Option<&openapiv3::ReferenceOr<openapiv3::Schema>>,
    r: Option<&openapiv3::ReferenceOr<openapiv3::Response>>,
) -> Result<(
    String,        // original response type
    crate::TypeId, // type id
    String,        // optional vec response type if this struct paginates
    String,        // optional name of vec response property if this struct paginates
)> {
    let object_name = format!("{} response", oid_to_object_name(od));
    let mut tid = TypeId(0);

    if let Some(s) = s {
        tid = ts.select(Some(&clean_name(&object_name)), s, "")?;

        if let openapiv3::ReferenceOr::Reference { reference } = s {
            tid = ts.select_ref(Some(&clean_name(&object_name)), reference)?;
        }
    }

    if let Some(openapiv3::ReferenceOr::Reference { reference }) = r {
        tid = ts.select_ref(Some(&clean_name(&object_name)), reference)?;
    }

    if tid == TypeId(0) {
        bail!("should have gotten type id for {}", od);
    }

    let og_rt = ts.render_type(&tid, false)?;
    let mut et = ts.id_to_entry.get(&tid).unwrap();

    if let crate::TypeDetails::NamedType(id, _) = &et.details {
        et = ts.id_to_entry.get(id).unwrap();
    }

    if let crate::TypeDetails::Object(p, _) = &et.details {
        // For KittyCAD, the pagination values are passed _in_ the resulting
        // struct, so we want to ignore them and just get the data.
        if let Some(pid) = p.get("next_page") {
            let rt = ts.render_type(pid, false)?;
            if rt == "String" {
                if let Some(did) = p.get("items") {
                    let rt = ts.render_type(did, false)?;
                    return Ok((og_rt, did.clone(), rt, "items".to_string()));
                } else {
                    for (n, id) in p {
                        // Now we must find the property with the vector for this struct.
                        let rt = ts.render_type(id, false)?;
                        if rt.starts_with("Vec<") {
                            return Ok((og_rt, id.clone(), rt, to_snake_case(n)));
                        }
                    }
                }
            }
        }
    }

    Ok((og_rt, tid, "".to_string(), "".to_string()))
}

fn get_response_type(
    od: &str,
    ts: &mut TypeSpace,
    o: &openapiv3::Operation,
) -> Result<(
    String,        // original response type
    crate::TypeId, // type id
    String,        // optional vec response type if this struct paginates
    String,        // optional name of vec response property if this struct paginates
)> {
    if o.responses.responses.is_empty() {
        // Return empty.
        return Ok((
            "()".to_string(),
            crate::TypeId(0),
            "".to_string(),
            "".to_string(),
        ));
    }

    // Get the first response.
    let first = o.responses.responses.first().unwrap();
    if let Ok(i) = first.1.item() {
        if i.content.is_empty() {
            // Return empty.
            return Ok((
                "()".to_string(),
                crate::TypeId(0),
                "".to_string(),
                "".to_string(),
            ));
        }

        // Get the json response, if it exists.
        if let Some(mt) = i.content.get("application/json") {
            if !mt.encoding.is_empty() {
                bail!("media type encoding not empty: {:#?}", mt);
            }

            if let Some(s) = &mt.schema {
                if let Ok(item) = s.item() {
                    // We have an item, we want to check
                    // if its an ANY kind and empty, then we can ignore it.
                    if let openapiv3::SchemaKind::Any(any) = &item.schema_kind {
                        if any.properties.is_empty() && any.format.is_none() && any.items.is_none()
                        {
                            // Return empty.
                            return Ok((
                                "()".to_string(),
                                crate::TypeId(0),
                                "".to_string(),
                                "".to_string(),
                            ));
                        }
                    }
                }

                // Get response type from object.
                return get_response_type_from_object(od, ts, Some(s), None);
            }
        }

        // Get the first response.
        let (ct, mt) = i.content.first().unwrap();
        if ct == "text/plain"
            || ct == "text/html"
            || ct == "application/octocat-stream"
            || ct == "*/*"
        {
            if let Some(s) = &mt.schema {
                let tid = ts.select(None, s, "")?;
                let rt = ts.render_type(&tid, false)?;
                return Ok((rt, tid, "".to_string(), "".to_string()));
            }
        } else if ct == "application/scim+json" {
            if !mt.encoding.is_empty() {
                bail!("media type encoding not empty: {:#?}", mt);
            }

            if let Some(s) = &mt.schema {
                let object_name = format!("{} response", oid_to_object_name(od));
                let tid = ts.select(Some(&clean_name(&object_name)), s, "")?;
                let rt = ts.render_type(&tid, false)?;
                return Ok((rt, tid, "".to_string(), "".to_string()));
            }
        }
    } else if let openapiv3::ReferenceOr::Reference { reference: _ } = first.1 {
        // Get response type from object.
        return get_response_type_from_object(od, ts, None, Some(first.1));
    }

    // Basically if we get here, likely its just an empty struct or something.
    // We never actually hit here before Zoom but then it was just an empty response.
    Ok((
        "()".to_string(),
        crate::TypeId(0),
        "".to_string(),
        "".to_string(),
    ))
}

fn sort_parameters(o: &openapiv3::Operation) -> Result<BTreeMap<String, openapiv3::Parameter>> {
    let mut parameters = BTreeMap::new();

    for param in o.parameters.iter() {
        let param = param.item()?;

        let parameter_data = match get_parameter_data(param) {
            Some(s) => s,
            None => return Ok(parameters),
        };

        parameters.insert(parameter_data.name.to_string(), param.clone());
    }

    let mut sorted_keys = parameters.keys().collect::<Vec<_>>();
    sorted_keys.sort();
    let mut sorted_parameters: BTreeMap<String, openapiv3::Parameter> = BTreeMap::new();
    for k in sorted_keys {
        sorted_parameters.insert(k.to_string(), parameters.get(k).unwrap().clone());
    }

    Ok(sorted_parameters)
}

#[allow(clippy::type_complexity)]
fn get_fn_params(
    ts: &mut TypeSpace,
    o: &openapiv3::Operation,
    all_pages: bool,
) -> Result<(Vec<String>, BTreeMap<String, (String, String)>)> {
    /*
     * Query parameters are sorted lexicographically to ensure a stable
     * order in the generated code.
     */
    let mut fn_params_str: Vec<String> = Default::default();
    let mut fn_params: Vec<String> = Default::default();
    let mut query_params: BTreeMap<String, (String, String)> = Default::default();
    let gp = sort_parameters(o)?;
    for (param_name, item) in gp.iter() {
        let parameter_data = get_parameter_data(item).unwrap();
        let nam = &to_snake_case(&parameter_data.name);

        if !fn_params.contains(nam) && !fn_params.contains(&format!("{}_", nam)) {
            let typ = parameter_data.render_type(param_name, ts)?;
            if nam == "ref"
                || nam == "type"
                || nam == "foo"
                || nam == "enum"
                || nam == "const"
                || nam == "use"
            {
                fn_params_str.push(format!("{}_: {},", nam, typ));
                fn_params.push(nam.to_string() + "_");
            } else if nam == "i_ds" {
                fn_params_str.push(format!("ids: {},", typ));
                fn_params.push("ids".to_string());
            } else if (!all_pages || !is_page_param(nam))
                && nam != "authorization"
                && !nam.starts_with("authorization_bearer")
            {
                if typ == "chrono::DateTime<chrono::Utc>" {
                    fn_params_str.push(format!("{}: Option<{}>,", nam, typ));
                    fn_params.push(nam.to_string());
                } else {
                    let p = format!("{}: {},", nam, typ);
                    if !fn_params.contains(nam) {
                        fn_params_str.push(p);
                        fn_params.push(nam.to_string());
                    }
                }
            }

            // Check if we have a query.
            // TODO: make this a bool ext.
            if let openapiv3::Parameter::Query {
                parameter_data: _,
                allow_reserved: _,
                style: openapiv3::QueryStyle::Form,
                // We can ignore the allow empty value, we support this by default and
                // aren't strict about not allowing empty values on other parameters
                // merely because specs cannot be trusted.
                allow_empty_value: _,
            } = item
            {
                if nam == "ref"
                    || nam == "type"
                    || nam == "foo"
                    || nam == "enum"
                    || nam == "const"
                    || nam == "use"
                {
                    query_params.insert(
                        format!("{}_", nam),
                        (typ.to_string(), parameter_data.name.to_string()),
                    );
                } else if nam == "i_ds" {
                    query_params.insert(
                        "ids".to_string(),
                        (typ.to_string(), parameter_data.name.to_string()),
                    );
                } else if (!all_pages || !is_page_param(nam))
                    && nam != "authorization"
                    && !nam.starts_with("authorization_bearer")
                {
                    if typ == "chrono::DateTime<chrono::Utc>" {
                        query_params.insert(
                            nam.to_string(),
                            (format!("Option<{}>", typ), parameter_data.name.to_string()),
                        );
                    } else {
                        query_params.insert(
                            nam.to_string(),
                            (typ.to_string(), parameter_data.name.to_string()),
                        );
                    }
                }
            }
        }
    }

    Ok((fn_params_str, query_params))
}

/*
 * Perform the function.
 */
// TODO: Fix this
#[allow(clippy::too_many_arguments)]
fn get_fn_inner(
    oid: &str,
    m: &str,
    body_func: &Option<String>,
    response_type: &str,
    inner_response_type: &str,
    pagination_property: &str,
    all_pages: bool,
) -> Result<String> {
    let body = if let Some(f) = &body_func {
        if f == "json" {
            "Some(reqwest::Body::from(serde_json::to_vec(body)?))"
        } else {
            "Some(body.into())"
        }
    } else {
        "None"
    };

    if all_pages && pagination_property.is_empty() {
        return Ok(format!("self.client.get_all_pages(&url, {}).await", body));
    } else if all_pages {
        // We will do a custom function here.
        let inner = format!(
            r#"let mut resp: {} = self.client.{}(&url, {}).await?;

            let mut {} = resp.{};
            let mut page = resp.next_page;

            // Paginate if we should.
            while !page.is_empty() {{
                if !url.contains('?') {{
                    resp = self.client.{}(&format!("{{}}?page={{}}", url, page), {}).await?;
                }} else {{
                    resp = self.client.{}(&format!("{{}}&page={{}}", url, page), {}).await?;
                }}


                {}.append(&mut resp.{});

                if !resp.next_page.is_empty() && resp.next_page != page {{
                    page = resp.next_page.to_string();
                }} else {{
                    page = "".to_string();
                }}
            }}

            // Return our response data.
            Ok({})"#,
            response_type,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            m.to_lowercase(),
            body,
            m.to_lowercase(),
            body,
            pagination_property,
            pagination_property,
            pagination_property,
        );

        return Ok(inner);
    } else if all_pages && !pagination_property.is_empty() {
        bail!(
            "must implement custom pagination function for {}",
            pagination_property
        );
    }

    if (m == http::Method::GET
        || m == http::Method::POST
        || m == http::Method::PATCH
        || m == http::Method::PUT
        || m == http::Method::DELETE)
        && oid != "apps_create_installation_access_token"
    {
        if inner_response_type.is_empty() {
            return Ok(format!(
                "self.client.{}(&url, {}).await",
                m.to_lowercase(),
                body
            ));
        }

        // Okay we have an inner response type, let's return that instead.
        return Ok(format!(
            r#"let resp: {} = self.client.{}(&url, {}).await?;

                // Return our response data.
                Ok(resp.{})"#,
            response_type,
            m.to_lowercase(),
            body,
            pagination_property
        ));
    }

    if oid != "apps_create_installation_access_token" {
        bail!("function {} should be authenticated", oid);
    }

    Ok(r#"self.client.post_media(
            &url,
            Some(reqwest::Body::from(serde_json::to_vec(body)?)),
            crate::utils::MediaType::Json,
            crate::auth::AuthenticationConstraint::JWT,
        ).await"#
        .to_string())
}

fn get_fn_docs(
    o: &openapiv3::Operation,
    m: &str,
    p: &str,
    parameters: &BTreeMap<String, &openapiv3::Parameter>,
    ts: &mut TypeSpace,
) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    a("/**");
    if let Some(summary) = &o.summary {
        a(&format!("* {}.", summary.trim_end_matches('.')));
        a("*");
    }
    a(&format!(
        "* This function performs a `{}` to the `{}` endpoint.",
        m, p
    ));
    if let Some(description) = &o.description {
        a("*");
        a(&format!("* {}", description.replace('\n', "\n* ")));
    }
    if let Some(external_docs) = &o.external_docs {
        a("*");
        a(&format!("* FROM: <{}>", external_docs.url));
    }
    if !o.parameters.is_empty() {
        a("*");
        a("* **Parameters:**");
        a("*");
    }
    // Iterate over the function parameters and add any data those had as well.
    for par in o.parameters.iter() {
        let mut param_name = "".to_string();
        let item = match par {
            openapiv3::ReferenceOr::Reference { reference } => {
                param_name = struct_name(&reference.replace("#/components/parameters/", ""));
                // Get the parameter from our BTreeMap.
                if let Some(param) = parameters.get(&param_name) {
                    param
                } else {
                    bail!("could not find parameter with reference: {}", reference);
                }
            }
            openapiv3::ReferenceOr::Item(item) => item,
        };

        let parameter_data = get_parameter_data(item).unwrap();

        let pid = ts.select_param(None, par)?;
        let mut docs = ts.render_docs(&pid);
        if let Some(d) = &parameter_data.description {
            if !d.is_empty() && d.len() > docs.len() {
                docs = format!(" -- {}.", d.trim_end_matches('.').replace('\n', "\n*   "));
            } else if !docs.is_empty() {
                docs = format!(
                    " -- {}.",
                    docs.trim_start_matches('*').trim_end_matches('.').trim()
                );
            }
        } else if !docs.is_empty() {
            docs = format!(
                " -- {}.",
                docs.trim_start_matches('*').trim_end_matches('.').trim()
            );
        }

        let nam = &to_snake_case(&clean_name(&parameter_data.name));
        let typ = parameter_data.render_type(&param_name, ts)?;

        if nam == "ref"
            || nam == "type"
            || nam == "foo"
            || nam == "enum"
            || nam == "const"
            || nam == "use"
        {
            a(&format!("* * `{}_: {}`{}", nam, typ, docs));
        } else {
            a(&format!("* * `{}: {}`{}", nam, typ, docs));
        }
    }
    a("*/");

    Ok(out.trim().to_string())
}

fn get_fn_docs_all(o: &openapiv3::Operation, m: &str, p: &str, fn_name: &str) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    a("/**");
    if let Some(summary) = &o.summary {
        a(&format!("* {}.", summary.trim_end_matches('.')));
        a("*");
    }
    a(&format!(
        "* This function performs a `{}` to the `{}` endpoint.",
        m, p
    ));
    a("*");
    a(&format!(
        "* As opposed to `{}`, this function returns all the pages of the request at once.",
        fn_name
    ));
    if let Some(description) = &o.description {
        a("*");
        a(&format!("* {}", description.replace('\n', "\n* ")));
    }
    if let Some(external_docs) = &o.external_docs {
        a("*");
        a(&format!("* FROM: <{}>", external_docs.url));
    }
    a("*/");

    Ok(out.trim().to_string())
}

fn is_page_param(s: &str) -> bool {
    s == "next_page" || s == "page_token" || s == "limit"
}
