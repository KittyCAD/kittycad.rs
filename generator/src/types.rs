use std::{cmp::Ordering, collections::BTreeMap};

use anyhow::{bail, Result};
use inflector::cases::snakecase::to_snake_case;

use crate::{render_param, struct_name, TypeDetails, TypeId, TypeSpace};

/*
 * Declare named types we know about:
 */
pub fn generate_types(api: &openapiv3::OpenAPI, ts: &mut TypeSpace) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    // Make sure we don't generate duplicate types.
    let mut seen: BTreeMap<String, bool> = BTreeMap::new();

    a("//! The data types sent to and returned from the API client.");
    a("    use parse_display::{Display, FromStr};");
    a("    use schemars::JsonSchema;");
    a("    use serde::{Serialize, Deserialize};");
    a("    use std::fmt;");
    a("    use tabled::Tabled;");
    a("");

    for te in ts.clone().id_to_entry.values() {
        if let Some(sn) = te.name.as_deref() {
            let mut sn = struct_name(sn);

            if seen.contains_key(sn.as_str()) {
                continue;
            }

            seen.insert(sn.clone(), true);

            if sn == "MetadataType" {
                sn = "Metadata".to_string();
            }

            if sn == "Error" {
                a(crate::types_templates::ERROR);
                sn = "ErrorResponse".to_string();
            }

            match &te.details {
                TypeDetails::Enum(vals, schema_data) => {
                    let mut desc = "".to_string();
                    if let Some(d) = &schema_data.description {
                        desc = d.to_string();
                    }
                    let p = render_param(
                        sn.as_str(),
                        vals,
                        false,
                        &desc,
                        schema_data.default.as_ref(),
                        true,
                    );
                    a(&p);
                }
                TypeDetails::Placeholder(..) => {}
                TypeDetails::OneOf(omap, _) => a(&do_one_of_type(ts, omap, sn)),
                TypeDetails::AnyOf(omap, _) => a(&do_all_of_type(ts, omap, sn)),
                TypeDetails::AllOf(omap, _) => a(&do_all_of_type(ts, omap, sn)),
                TypeDetails::Object(omap, schema_data) => {
                    let mut omap = omap.clone();
                    /*
                     * TODO: This breaks things so ignore for now.
                     * Eventually this should work, we should ignore empty structs.
                    if omap.is_empty() {
                        // Continue early.
                        // We don't care about empty structs.
                        continue;
                    }*/

                    let desc = if let Some(description) = &schema_data.description {
                        format!("/// {}", description.replace('\n', "\n/// "))
                    } else {
                        "".to_string()
                    };

                    if !desc.is_empty() {
                        a(&desc);
                    }

                    // TODO: just make everything a default,
                    // this is gated by the oneof types cooperating.
                    a("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema,");
                    a("Default,");
                    if sn != "PaymentMethod"
                        && sn != "Metadata"
                        && sn != "EngineMetadata"
                        && sn != "JetstreamStats"
                        && sn != "Jetstream"
                        && sn != "Invoice"
                        && sn != "BillingInfo"
                        && sn != "Customer"
                        && sn != "Connection"
                        && sn != "CardDetails"
                        && sn != "AsyncApiCall"
                    {
                        a("Tabled,");
                    }
                    a(r#")]"#);

                    a(&format!("pub struct {} {{", sn));

                    // If possible we want the order to be id, name, description,
                    // then everything else.
                    // Let's shoot for that.
                    let try_first = vec!["id", "name", "description"];
                    for f in try_first.iter() {
                        if let Some(tid) = omap.get(&f.to_string()) {
                            a(&render_property(ts, tid, f, &desc, &sn)?);
                            omap.remove(&f.to_string());
                        }
                    }

                    for (name, tid) in omap.iter() {
                        a(&render_property(ts, tid, name, &desc, &sn)?);
                    }
                    a("}");
                    a("");
                }
                TypeDetails::Basic(..) => {}
                TypeDetails::Unknown => {}
                TypeDetails::NamedType(..) => {}
                TypeDetails::ComponentSchema(tid, _schema_data) => {
                    a(&format!(
                        "pub type {} = {};",
                        sn,
                        ts.render_type(tid, true)?
                    ));
                }
                TypeDetails::Array(..) => {}
                TypeDetails::Optional(..) => {}
            }
        }
    }

    // Iterate over anything we missed.
    if let Some(components) = &api.components {
        for (_i, (sn, s)) in components.schemas.iter().enumerate() {
            if sn == "Ipv6Net" || sn == "Ipv4Net" {
                continue;
            }

            let id = ts.select(Some(sn.as_str()), s, "")?;

            let rendered = ts.render_type(&id, true)?;

            let et = ts.id_to_entry.get(&id).unwrap();
            let mut desc = "".to_string();
            if let TypeDetails::Basic(_, schema_data) = &et.details {
                desc = if let Some(description) = &schema_data.description {
                    format!("/// {}", description.replace('\n', "\n/// "))
                } else {
                    "".to_string()
                };
            }

            if rendered == "String" || rendered.starts_with('u') || rendered.starts_with('i') {
                if !desc.is_empty() {
                    a(&desc);
                }

                a(&format!("pub type {} = {};", sn, rendered));
            }
        }
    }

    Ok(out.to_string())
}

fn render_property(
    ts: &mut TypeSpace,
    tid: &TypeId,
    name: &str,
    desc: &str,
    sn: &str,
) -> Result<String> {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    if let Ok(mut rt) = ts.render_type(tid, true) {
        let mut prop = name.trim().to_string();
        if prop == "next" {
            rt = "String".to_string();
        }
        if prop == "ref"
            || prop == "type"
            || prop == "self"
            || prop == "box"
            || prop == "match"
            || prop == "foo"
            || prop == "enum"
            || prop == "const"
            || prop == "use"
        {
            prop = format!("{}_", name);
        } else if name == "$ref" {
            prop = format!("{}_", name.replace('$', ""));
        } else if name == "$type" {
            prop = format!("{}__", name.replace('$', ""));
        } else if name == "+1" {
            prop = "plus_one".to_string()
        } else if name == "-1" {
            prop = "minus_one".to_string()
        } else if name.starts_with('@') {
            prop = name.trim_start_matches('@').to_string();
        } else if name.starts_with('_') {
            prop = name.trim_start_matches('_').to_string();
        }

        // Try to render the docs.
        let p = ts.render_docs(tid);
        if !p.is_empty() && p != desc {
            a("/**");
            a(&p);
            a("*/");
        }

        let te = ts.id_to_entry.get(tid).unwrap();

        // Render the serde string.
        if rt == "String"
            || rt.starts_with("Vec<")
            || rt.starts_with("Option<")
            || rt.starts_with("BTreeMap<")
        {
            a(r#"#[serde(default,"#);
            if rt == "String" {
                a(r#"skip_serializing_if = "String::is_empty",
                                        deserialize_with = "crate::utils::deserialize_null_string::deserialize","#);
            } else if rt.starts_with("Vec<") {
                a(r#"skip_serializing_if = "Vec::is_empty",
                                      deserialize_with = "crate::utils::deserialize_null_vector::deserialize","#);
            } else if rt.starts_with("std::collections::BTreeMap<") {
                a(r#"skip_serializing_if = "std::collections::BTreeMap::is_empty","#);
            } else if rt.starts_with("Option<url::Url") {
                a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::deserialize_empty_url::deserialize","#);
            } else if rt.starts_with("Option<chrono::NaiveDate") {
                a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_format::deserialize","#);
            } else if rt.starts_with("crate::utils::DisplayOptionDateTime") {
                a(r#"skip_serializing_if = "Option::is_none",
                                      deserialize_with = "crate::utils::date_time_format::deserialize","#);
            } else if rt.starts_with("Option<") {
                a(r#"skip_serializing_if = "Option::is_none","#);
            }
        } else if rt == "bool" {
            if sn.ends_with("Request") {
                // We have a request, we want to make sure our bools are
                // options so we don't have to always provide them.
                a(r#"#[serde(default, skip_serializing_if = "Option::is_none","#);
                rt = "Option<bool>".to_string();
            } else {
                a(r#"#[serde(default,
                                    deserialize_with = "crate::utils::deserialize_null_boolean::deserialize","#);
            }
        } else if rt == "i32" {
            a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i32",
                                    deserialize_with = "crate::utils::deserialize_null_i32::deserialize","#);
        } else if rt == "i64" {
            a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_i64",
                                    deserialize_with = "crate::utils::deserialize_null_i64::deserialize","#);
        } else if rt == "f32" {
            a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f32",
                                    deserialize_with = "crate::utils::deserialize_null_f32::deserialize","#);
        } else if rt == "f64" {
            a(r#"#[serde(default,
                                    skip_serializing_if = "crate::utils::zero_f64",
                                    deserialize_with = "crate::utils::deserialize_null_f64::deserialize","#);
        } else if rt == "u32" || rt == "u64" {
            a(r#"#[serde(default,"#);
        } else if let TypeDetails::Enum(_, sd) = &te.details {
            // We for sure have a default for every single enum, even
            // if the default is a noop.
            a(r#"#[serde(default,"#);
            // Figure out if its a no op and skip serializing if it is.
            if sd.default.is_none() {
                a(&format!(r#"skip_serializing_if = "{}::is_noop","#, rt));
            }
        } else {
            a(r#"#[serde("#);
        }

        if !prop.ends_with('_') {
            prop = to_snake_case(&prop);
        }

        // DO this again.
        // I know this is shit sue me, but sometimes we change the prop
        // so much it becomes one of these, ie. in the case of shipbob.
        if prop == "ref"
            || prop == "type"
            || prop == "self"
            || prop == "box"
            || prop == "match"
            || prop == "foo"
            || prop == "enum"
            || prop == "const"
            || prop == "use"
        {
            prop = format!("{}_", prop);
        }

        if prop == "ipv_4_block" {
            prop = "ipv4_block".to_string();
        } else if prop == "ipv_6_block" {
            prop = "ipv6_block".to_string();
        } else if prop == "ipv_6_prefix" {
            prop = "ipv6_prefix".to_string();
        } else if prop == "ipv_4_prefix" {
            prop = "ipv4_prefix".to_string();
        }

        // Close the serde string.
        if *name != prop {
            a(&format!(r#"rename = "{}")]"#, name));
        } else if rt == "Page" && prop == "page" || rt.ends_with("Page") {
            a(r#"default)]"#);
        } else {
            a(r#")]"#);
        }

        // Hide things from the table that don't implement display.
        if rt.starts_with("Vec<") && rt != "Vec<InvoiceLineItem>" {
            a(r#"#[tabled(skip)]"#);
        }

        if prop == "type" {
            println!("{} {}", sn, prop);
        }

        a(&format!("pub {}: {},", prop, rt));
    } else {
        bail!("rendering type {} {:?} failed", name, tid);
    }

    Ok(out.to_string())
}

fn do_one_of_type(
    ts: &mut TypeSpace,
    one_of: &[openapiv3::ReferenceOr<openapiv3::Schema>],
    sn: String,
) -> String {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    let mut tag = "";
    let mut content = "";
    let mut omap: Vec<crate::TypeId> = Default::default();
    for one in one_of {
        let itid = ts.select(Some(&sn), one, "").unwrap();
        omap.push(itid);
    }

    omap.sort_unstable();
    omap.dedup();

    let mut is_enum = false;

    for itid in omap.iter() {
        // Determine if we can do anything fancy with the resulting enum and flatten it.
        let et = ts.id_to_entry.get(itid).unwrap();

        if let TypeDetails::Object(o, _) = &et.details {
            // Iterate over the properties of the object and try to find a tag.
            for (name, prop) in o.iter() {
                let pet = ts.id_to_entry.get(prop).unwrap();
                // Check if we have an enum of one.
                if let TypeDetails::Enum(e, _) = &pet.details {
                    is_enum = true;
                    if e.len() == 1 {
                        // We have an enum of one so we can use that as the tag.
                        tag = name;
                        continue;
                    }
                } else if o.len() == 2 {
                    content = name;
                }
            }
        }
    }

    a("#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]");
    if !tag.is_empty() {
        a("#[serde(rename_all = \"snake_case\")]");
        a(&format!("#[serde(tag = \"{}\"", tag));
        if !content.is_empty() {
            a(&format!(", content = \"{}\"", content));
        }
        a(")]");
    }
    a(&format!("pub enum {} {{", sn));

    let mut types_strings: BTreeMap<String, (String, String)> = Default::default();

    let mut prop_types: Vec<(String, String)> = Default::default();

    for tid in omap.iter() {
        let et = ts.id_to_entry.get(tid).unwrap();
        if let TypeDetails::Object(o, _) = &et.details {
            let mut name = String::new();
            for (key, prop) in o.iter() {
                let pet = ts.id_to_entry.get(prop).unwrap();
                // Check if we have an enum of one.
                if let TypeDetails::Enum(e, _) = &pet.details {
                    if e.len() == 1 {
                        let prop = struct_name(&e[0]);
                        let mut sep = "";
                        // We have an enum of one so we can use that as the tag.
                        if o.len() == 1 {
                            a(&format!("{},", prop));
                        } else if o.len() == 2 {
                            a(&format!("{}(", prop));
                            sep = "(..)";
                        } else if o.len() > 2 {
                            a(&format!("{} {{", prop));
                            sep = "{..}";
                        }
                        types_strings.insert(prop.clone(), (e[0].to_string(), sep.to_string()));
                        name = prop.clone();
                        break;
                    }
                } else if o.len() == 1 {
                    a(&format!("{}(", struct_name(key)));
                    types_strings.insert(key.clone(), (key.to_string(), "(..)".to_string()));
                    name = key.clone();
                }
            }
            for (n, prop) in o.iter() {
                let pet = ts.id_to_entry.get(prop).unwrap();
                // Check if we have an enum of one.
                if let TypeDetails::Enum(e, _) = &pet.details {
                    if e.len() == 1 {
                        continue;
                    }
                }

                if o.len() <= 2 {
                    let t = ts.render_type(prop, true).unwrap();
                    prop_types.push((name.to_string(), t.to_string()));
                    a(&format!("{},", t));
                } else {
                    let t = ts.render_type(prop, true).unwrap();
                    prop_types.push((name.to_string(), t.to_string()));
                    a(&format!("{}: {},", n, t));
                }
            }

            match o.len().cmp(&2) {
                Ordering::Less => {
                    if !is_enum {
                        a("),");
                    }
                }
                Ordering::Equal => {
                    a("),");
                }
                Ordering::Greater => {
                    a("},");
                }
            }
        }
    }

    a("}");
    a("");

    if !tag.is_empty() && !content.is_empty() {
        // Handle display and to_string differently.
        // Now we need to implement display for the enum.
        a(&format!("impl fmt::Display for {} {{", sn));
        a("fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {");
        a("   let j = serde_json::json!(self);");
        a(&format!(
            "   let mut tag: String = \
             serde_json::from_value(j[\"{}\"].clone()).unwrap_or_default();",
            tag
        ));
        if sn == "DiskSource" {
            a(&format!(
                "   let mut content: String = \
             match serde_json::from_value(j[\"{}\"].clone()) {{ \
             Ok(v) => v, \
                Err(_) => {{ \
                let int : i64 = serde_json::from_value(j[\"{}\"].clone()).unwrap_or_default(); \
                format!(\"{{}}\", int) \
                }} \
                }};",
                content, content
            ));
        } else {
            a(&format!(
                "   let mut content: String = \
             serde_json::from_value(j[\"{}\"].clone()).unwrap_or_default();",
                content
            ));
        }
        a(" if content.is_empty() {");
        a(&format!(
            "let map: std::collections::HashMap<String, String> = \
             serde_json::from_value(j[\"{}\"].clone()).unwrap_or_default();",
            content
        ));
        a("if let Some((_, v)) = map.iter().next() { content = v.to_string(); }");
        a("}");
        a("if tag == \"internet_gateway\" { tag = \"inetgw\".to_string(); }");
        a("     write!(f, \"{}={}\",tag, content)");
        a("}");
        a("}");
        a("");

        // Let's implement FromStr for clap so we can use enums there.
        a(&format!("impl std::str::FromStr for {} {{", sn));
        a("type Err = anyhow::Error;");
        a("fn from_str(s: &str) -> Result<Self, Self::Err> {");
        a("    let parts = s.split('=').collect::<Vec<&str>>();");
        a("    if parts.len() != 2 {");
        a(&format!(
            r#"        anyhow::bail!("invalid format for {}, got {{}}", s);"#,
            sn
        ));
        a("    }");
        a("    let tag = parts[0].to_string();");
        a("    let content = parts[1].to_string();");
        a("    let mut j = String::new();");
        for (name, p) in prop_types.iter() {
            let mut k = to_snake_case(name);
            if k == "internet_gateway" {
                k = "inetgw".to_string();
            }
            a(&format!("if tag == \"{}\" {{", k));
            a("j = format!(r#\"{{");
            a(&format!("\"{}\": \"{}\",", tag, to_snake_case(name)));
            if p == "String" || p.starts_with("Vec<") {
                a(&format!("\"{}\": \"{{}}\"", content));
                a("        }}\"#, content);");
            } else {
                a(&format!("\"{}\": {{}}", content));
                a(&format!(
                    "        }}}}\"#, serde_json::json!({}::from_str(&content).unwrap()));",
                    p
                ));
            }
            a("}");
        }
        a("    let result = serde_json::from_str(&j)?;");
        a("    Ok(result)");
        a("}");
        a("}");
    } else {
        // Now we need to implement display for the enum.
        a(&format!("impl fmt::Display for {} {{", sn));
        a("fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {");
        a("   write!(f, \"{}\", serde_json::json!(self))");
        a("}");
        a("}");
        a("");

        // Let's implement FromStr for clap so we can use enums there.
        a(&format!("impl std::str::FromStr for {} {{", sn));
        a("type Err = anyhow::Error;");
        a("fn from_str(s: &str) -> Result<Self, Self::Err> {");
        a("   Ok(serde_json::from_str(s)?)");
        a("}");
        a("}");
    }

    if !tag.is_empty() {
        let mut values: Vec<String> = Vec::new();
        // If we have a tag let's create the variant types.
        a(&format!("impl {} {{", sn));
        a("pub fn variants() -> Vec<String> {");
        a("    vec![");
        for (name, _) in types_strings.iter() {
            let mut k = to_snake_case(name);
            if k == "internet_gateway" {
                k = "inetgw".to_string();
            }
            a(&format!("        \"{}\".to_string(),", k));
            values.push(name.to_string());
        }
        a("    ]");
        a("}");
        a("}");

        // Now we want to render a new enum for this type.
        let render = render_param(
            &format!("{}Type", sn),
            &values,
            false,
            &format!("The types for {}.", sn),
            None,
            false,
        );

        a(&render);
    }

    out
}

fn do_all_of_type(ts: &mut TypeSpace, omap: &[crate::TypeId], sn: String) -> String {
    let mut out = String::new();

    let mut a = |s: &str| {
        out.push_str(s);
        out.push('\n');
    };

    // Get the description.
    let mut description =
        "All of the following types are flattened into one object:\n\n".to_string();

    for itid in omap {
        let rt = ts.render_type(itid, true).unwrap();
        description.push_str(&format!("- `{}`\n", rt));
    }
    description = format!("/// {}", description.replace('\n', "\n/// "));
    a(&description);

    a("#[derive(Serialize, Deserialize, Default, PartialEq, Debug, Clone, JsonSchema, Tabled)]");
    a(&format!("pub struct {} {{", sn));
    let mut name_map: BTreeMap<String, String> = Default::default();
    // Becasue we have so many defaults set on our serde types these enums
    // sometimes parse the wrong value. It's better to instead use the functions we
    // inject that force the value to a specific type.
    let mut fns: Vec<String> = Default::default();
    for tid in omap.iter() {
        let name = ts.render_type(tid, true).unwrap();

        let fn_name = if name.starts_with("Vec<") {
            format!(
                "{}Vector",
                name.trim_start_matches("Vec<")
                    .trim_end_matches('>')
                    .replace("serde_json::", "")
            )
        } else if name.starts_with("serde_json") {
            "Value".to_string()
        } else {
            struct_name(&name)
        };

        if !fns.contains(&fn_name) {
            // Try to render the docs.
            let p = ts.render_docs(tid);
            if !p.is_empty() && p != description {
                a("/**");
                a(&p);
                a("*/");
            }

            a("#[serde(flatten)]");
            a(&format!("pub {}: {},", to_snake_case(&fn_name), name));
            name_map.insert(fn_name.to_string(), name.to_string());
            fns.push(fn_name);
        }
    }
    a("}");
    a("");

    out
}
