// TODO: #![deny(missing_docs)]

pub mod exts;

#[macro_use]
extern crate quote;

use anyhow::Result;
use numeral::Cardinal;

use crate::exts::ReferenceOrExt;

/// Generate Rust types from an OpenAPI v3 spec.
pub fn generate_types(spec: &openapiv3::OpenAPI) -> Result<String> {
    // Let's start with the components if there are any.
    let mut rendered = quote!();

    if let Some(components) = &spec.components {
        // Parse the schemas.
        for (name, schema) in &components.schemas {
            // Let's get the schema from the reference.
            let schema = schema.get_schema_from_reference(spec, true)?;
            // Let's handle all the kinds of schemas.
            // TODO: handle the error
            if let Ok(t) = render_schema(name, &schema, spec) {
                // Add it to our rendered types.
                rendered = quote! {
                    #rendered

                    #t
                };
            }
            /*let t = render_schema(name, &schema, spec)?;
            // Add it to our rendered types.
            rendered = quote! {
                #rendered

                #t
            };*/
        }
    }

    get_text_fmt(&rendered)
}

/// Render a schema into a Rust type.
/// This generates the Rust type.
pub fn render_schema(
    name: &str,
    schema: &openapiv3::Schema,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
            render_string_type(name, s, &schema.schema_data)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(_n)) => {
            // We don't render numbers yet, since it is a primitive type.
            Ok(quote!())
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(_i)) => {
            // We don't render integers yet, since it is a primitive type.
            Ok(quote!())
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) => {
            render_object(name, o, &schema.schema_data, spec)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(_a)) => {
            println!("{} => Array", name);
            anyhow::bail!("XXX array not supported yet");
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean { .. }) => {
            // We don't render booleans yet, since it is a primitive type.
            Ok(quote!())
        }
        openapiv3::SchemaKind::OneOf { one_of: _ } => {
            println!("{} => OneOf", name);
            anyhow::bail!("XXX one of not supported yet");
        }
        openapiv3::SchemaKind::AllOf { all_of: _ } => {
            println!("{} => AllOf", name);
            anyhow::bail!("XXX all of not supported yet");
        }
        openapiv3::SchemaKind::AnyOf { any_of: _ } => {
            println!("{} => AnyOf", name);
            anyhow::bail!("XXX any of not supported yet");
        }
        openapiv3::SchemaKind::Not { not: _ } => {
            println!("{} => Not", name);
            anyhow::bail!("XXX not not supported yet");
        }
        openapiv3::SchemaKind::Any(_any) => {
            println!("{} => Any", name);
            anyhow::bail!("XXX any not supported yet");
        }
    }
}

/// Return the type name for a schema.
pub fn get_type_name_for_schema(
    name: &str,
    schema: &openapiv3::Schema,
) -> Result<proc_macro2::TokenStream> {
    match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
            get_type_name_for_string(name, s, &schema.schema_data)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(n)) => {
            get_type_name_for_number(name, n, &schema.schema_data)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(i)) => {
            get_type_name_for_integer(name, i, &schema.schema_data)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(_o)) => {
            // We have an object type.
            // Get the name for the object.
            let ident = get_type_name(name, &schema.schema_data)?;
            Ok(quote!(#ident))
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(a)) => {
            get_type_name_for_array(name, a, &schema.schema_data)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean { .. }) => Ok(quote!(bool)),
        openapiv3::SchemaKind::OneOf { one_of: _ } => {
            println!("{} => OneOf", name);
            anyhow::bail!("XXX one of not supported yet");
        }
        openapiv3::SchemaKind::AllOf { all_of: _ } => {
            println!("{} => AllOf", name);
            anyhow::bail!("XXX all of not supported yet");
        }
        openapiv3::SchemaKind::AnyOf { any_of: _ } => {
            println!("{} => AnyOf", name);
            anyhow::bail!("XXX any of not supported yet");
        }
        openapiv3::SchemaKind::Not { not: _ } => {
            println!("{} => Not", name);
            anyhow::bail!("XXX not not supported yet");
        }
        openapiv3::SchemaKind::Any(_any) => {
            println!("{} => Any", name);
            anyhow::bail!("XXX any not supported yet");
        }
    }
}

/// Render a string type.
fn render_string_type(
    name: &str,
    s: &openapiv3::StringType,
    data: &openapiv3::SchemaData,
) -> Result<proc_macro2::TokenStream> {
    println!("{} => String", name);
    println!("{} => {:?}", name, s);
    println!("{} => {:?}", name, data);

    if !s.enumeration.is_empty() {
        return render_enum(name, s, data);
    }

    if let Some(ref max_length) = s.max_length {
        anyhow::bail!("XXX max_length not supported here yet: {:?}", max_length);
    }

    if let Some(ref min_length) = s.min_length {
        anyhow::bail!("XXX min_length not supported here yet: {:?}", min_length);
    }

    // We don't render primitives yet.
    Ok(quote!())
}

/// Get the type name for a string type.
fn get_type_name_for_string(
    name: &str,
    s: &openapiv3::StringType,
    data: &openapiv3::SchemaData,
) -> Result<proc_macro2::TokenStream> {
    if !s.enumeration.is_empty() {
        // We have an enum type.
        // Get the name for the enum.
        let ident = get_type_name(name, data)?;
        return Ok(quote!(#ident));
    }

    let t = match &s.format {
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::DateTime) => {
            quote!(chrono::DateTime<chrono::Utc>)
        }
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => {
            quote!(chrono::NaiveDate)
        }
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Password) => {
            quote!(String)
        }
        // TODO: as per the spec this is base64 encoded chars.
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Byte) => {
            quote!(bytes::Bytes)
        }
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Binary) => {
            quote!(bytes::Bytes)
        }
        openapiv3::VariantOrUnknownOrEmpty::Empty => quote!(String),
        openapiv3::VariantOrUnknownOrEmpty::Unknown(f) => match f.as_str() {
            "float" => quote!(f64),
            "int64" => quote!(i64),
            "uint64" => quote!(u64),
            "ipv4" => quote!(std::net::Ipv4Addr),
            "ip" => quote!(std::net::Ipv4Addr),
            "uri" => quote!(url::Url),
            "uri-template" => quote!(String),
            "url" => quote!(url::Url),
            "email" => quote!(String),
            "phone" => quote!(String),
            "uuid" => quote!(uuid::Uuid),
            "hostname" => quote!(String),
            "time" => quote!(chrono::NaiveTime),
            "date-time" => quote!(chrono::DateTime<chrono::Utc>),
            "partial-date-time" => quote!(chrono::DateTime<chrono::Utc>),
            f => {
                anyhow::bail!("XXX unknown string format {}", f)
            }
        },
    };

    Ok(t)
}

/// Get the type name for a number type.
fn get_type_name_for_number(
    name: &str,
    n: &openapiv3::NumberType,
    data: &openapiv3::SchemaData,
) -> Result<proc_macro2::TokenStream> {
    println!("{} => Number", name);
    println!("{} => {:?}", name, n);
    println!("{} => {:?}", name, data);

    let t = match &n.format {
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Float) => {
            quote!(f64)
        }
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Double) => {
            quote!(f64)
        }
        openapiv3::VariantOrUnknownOrEmpty::Empty => quote!(f64),
        openapiv3::VariantOrUnknownOrEmpty::Unknown(f) => {
            anyhow::bail!("XXX unknown number format {}", f)
        }
    };

    Ok(t)
}

/// Get the type name for an integer type.
fn get_type_name_for_integer(
    name: &str,
    i: &openapiv3::IntegerType,
    data: &openapiv3::SchemaData,
) -> Result<proc_macro2::TokenStream> {
    println!("{} => Integer", name);
    println!("{} => {:?}", name, i);
    println!("{} => {:?}", name, data);

    let t = match &i.format {
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int32) => {
            quote!(i32)
        }
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) => {
            quote!(i64)
        }
        openapiv3::VariantOrUnknownOrEmpty::Empty => quote!(i64),
        openapiv3::VariantOrUnknownOrEmpty::Unknown(f) => {
            let uint;
            let width;
            match f.as_str() {
                "uint" | "uint32" => {
                    uint = true;
                    width = 32;
                }
                "uint8" => {
                    uint = true;
                    width = 8;
                }
                "uint16" => {
                    uint = true;
                    width = 16;
                }
                "uint64" => {
                    uint = true;
                    width = 64;
                }
                "int8" => {
                    uint = false;
                    width = 8;
                }
                "int16" => {
                    uint = false;
                    width = 16;
                }
                /* int32 and int64 are build it and parse as the integer type */
                f => anyhow::bail!("unknown integer format {}", f),
            }

            if uint {
                match width {
                    8 => quote!(u8),
                    16 => quote!(u16),
                    32 => quote!(u32),
                    64 => quote!(u64),
                    _ => anyhow::bail!("unknown uint width {}", width),
                }
            } else {
                match width {
                    8 => quote!(i8),
                    16 => quote!(i16),
                    32 => quote!(i32),
                    64 => quote!(i64),
                    _ => anyhow::bail!("unknown int width {}", width),
                }
            }
        }
    };

    Ok(t)
}

/// Get the type name for an array type.
fn get_type_name_for_array(
    name: &str,
    a: &openapiv3::ArrayType,
    data: &openapiv3::SchemaData,
) -> Result<proc_macro2::TokenStream> {
    println!("{} => Array", name);
    println!("{} => {:?}", name, a);
    println!("{} => {:?}", name, data);

    // Make sure we have a reference for our type.
    if let Some(ref s) = a.items {
        let reference = format_ident!("{}", s.reference()?);
        return Ok(quote!(#reference));
    }

    // This should never happen, but who knows.
    anyhow::bail!("no items in array, cannot get type name")
}

/// Render the full type for an object.
fn render_object(
    name: &str,
    o: &openapiv3::ObjectType,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    println!("{} => Object", name);
    println!("{} => {:?}", name, o);
    println!("{} => {:?}", name, data);

    let description = if let Some(d) = &data.description {
        quote!(#[doc = #d])
    } else {
        quote!()
    };

    // Get the proper name version of the name of the enum.
    let struct_name = get_type_name(name, data)?;

    let mut values = quote!();
    for (k, v) in &o.properties {
        let prop = clean_property_name(k);

        // Get the schema for the property.
        let schema = if let openapiv3::ReferenceOr::Item(i) = v {
            let s = &**i;
            s.clone()
        } else {
            v.get_schema_from_reference(spec, true)?
        };
        println!("OBJ {} => {:?}", prop, schema);

        let prop_desc = if let Some(d) = &schema.schema_data.description {
            quote!(#[doc = #d])
        } else {
            quote!()
        };

        // Get the type name for the schema.
        let mut type_name = get_type_name_for_schema(&prop, &schema)?;
        // Check if this type is required.
        if !o.required.contains(k) && get_text(&type_name)?.starts_with("Option<") {
            // Make the type optional.
            type_name = quote!(Option<#type_name>);
        }
        let prop_ident = format_ident!("{}", prop);

        let mut prop_value = quote!(
            #prop_ident: #type_name,
        );
        if &prop != k {
            prop_value = quote!(
                #[serde(rename = #k)]
                #prop_value
            );
        }

        values = quote!(
            #values

            #prop_desc
            #prop_value
        );
    }

    // TODO: defaults
    /*// If the data for the enum has a default value, implement default for the enum.
    let default = if let Some(default) = &data.default {
        let default = default.to_string();
        let default = format_ident!("{}", proper_name(&default));
        quote!(
            impl Default for #enum_name {
                fn default() -> Self {
                    #default
                }
            }
        )
    } else if s.enumeration.len() == 1 {
        let default = s.enumeration[0].as_ref().unwrap().to_string();
        let default = format_ident!("{}", proper_name(&default));
        quote!(
            impl Default for #enum_name {
                fn default() -> Self {
                    #enum_name::#default
                }
            }
        )
    } else {
        quote!()
    };*/

    let rendered = quote! {
        #description
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, Debug, Clone, schemars::JsonSchema, tabled::Tabled)]
        pub struct #struct_name {
            #values
        }
    };

    Ok(rendered)
}

/// Clean a property name for an object so we can use it in rust.
fn clean_property_name(s: &str) -> String {
    let mut prop = inflector::cases::snakecase::to_snake_case(s.trim());

    // Account for reserved keywords in rust.
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
    } else if prop == "$ref" || prop == "$type" {
        // Account for any weird types.
        prop = format!("{}_", prop.replace('$', ""));
    } else if prop == "+1" {
        // Account for any weird types.
        prop = "plus_one".to_string()
    } else if prop == "-1" {
        // Account for any weird types.
        prop = "minus_one".to_string()
    } else if prop.starts_with('@') {
        // Account for any weird types.
        prop = prop.trim_start_matches('@').to_string();
    } else if prop.starts_with('_') {
        // Account for any weird types.
        prop = prop.trim_start_matches('_').to_string();
    }

    prop
}

/// Render the full type for an enum.
fn render_enum(
    name: &str,
    s: &openapiv3::StringType,
    data: &openapiv3::SchemaData,
) -> Result<proc_macro2::TokenStream> {
    if s.enumeration.is_empty() {
        anyhow::bail!("Cannot render empty string enumeration");
    }

    let description = if let Some(d) = &data.description {
        quote!(#[doc = #d])
    } else {
        quote!()
    };

    // Get the proper name version of the name of the enum.
    let enum_name = get_type_name(name, data)?;

    let mut values = quote!();
    for e in &s.enumeration {
        if e.is_none() {
            // TODO: do something for empty(?)
            continue;
        }

        let e = e.as_ref().unwrap().to_string();

        if proper_name(&e).is_empty() || e.trim().is_empty() {
            // TODO: do something for empty(?)
            continue;
        }

        let e_name = format_ident!("{}", proper_name(&e));
        let mut e_value = quote!(
            #e_name,
        );
        if proper_name(&e) != e {
            e_value = quote!(
                #[serde(rename = #e)]
                #[display(#e)]
                #e_value
            );
        }

        values = quote!(
            #values

            #e_value
        );
    }

    // If the data for the enum has a default value, implement default for the enum.
    let default = if let Some(default) = &data.default {
        let default = default.to_string();
        let default = format_ident!("{}", proper_name(&default));
        quote!(
            impl Default for #enum_name {
                fn default() -> Self {
                    #default
                }
            }
        )
    } else if s.enumeration.len() == 1 {
        let default = s.enumeration[0].as_ref().unwrap().to_string();
        let default = format_ident!("{}", proper_name(&default));
        quote!(
            impl Default for #enum_name {
                fn default() -> Self {
                    #enum_name::#default
                }
            }
        )
    } else {
        quote!()
    };

    let rendered = quote! {
        #description
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, Debug, Clone, schemars::JsonSchema, tabled::Tabled, clap::ValueEnum, parse_display::FromStr, parse_display::Display)]
        pub enum #enum_name {
            #values
        }

        #default
    };

    Ok(rendered)
}

/// Return a proper rust name for a string.
/// For example, this gets used as the enum and struct name.
fn proper_name(s: &str) -> String {
    // Check if s is a number like 1 or 2, etc.
    // If it is a number we want to convert it to a string as follows:
    // 1 => One
    // 2 => Two
    // 100 => OneHundred
    // etc.
    let s = if let Ok(num) = s.parse::<i32>() {
        num.cardinal()
    } else {
        s.to_string()
    };

    inflector::cases::pascalcase::to_pascal_case(&s)
}

/// Return the name for a type based on a name if passed or the title of the schema data.
fn get_type_name(name: &str, data: &openapiv3::SchemaData) -> Result<proc_macro2::Ident> {
    let t = format_ident!(
        "{}",
        if !name.is_empty() {
            proper_name(name)
        } else if let Some(title) = &data.title {
            proper_name(title)
        } else {
            anyhow::bail!("Cannot get type name without name or title: {:?}", data);
        }
    );

    Ok(t)
}

fn clean_text(s: &str) -> String {
    // Add newlines after end-braces at <= two levels of indentation.
    if cfg!(not(windows)) {
        let regex = regex::Regex::new(r#"(})(\n\s{0,8}[^} ])"#).unwrap();
        regex.replace_all(s, "$1\n$2").to_string()
    } else {
        let regex = regex::Regex::new(r#"(})(\r\n\s{0,8}[^} ])"#).unwrap();
        regex.replace_all(s, "$1\r\n$2").to_string()
    }
}

fn get_text(output: &proc_macro2::TokenStream) -> Result<String> {
    let content = output.to_string();

    Ok(clean_text(&content).replace(' ', ""))
}

fn get_text_fmt(output: &proc_macro2::TokenStream) -> Result<String> {
    // Format the file with rustfmt.
    let content = rustfmt_wrapper::rustfmt(output).unwrap();

    Ok(clean_text(&content))
}

/// Parse an OpenAPI v3 spec JSON string as an OpenAPI struct.
pub fn load_spec(s: &str) -> Result<openapiv3::OpenAPI> {
    serde_json::from_str(s).map_err(|e| anyhow::anyhow!(e))
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_generate_kittycad_types() {
        let result =
            super::generate_types(&super::load_spec(include_str!("../../spec.json")).unwrap())
                .unwrap();
        expectorate::assert_contents("tests/kittycad.rs.gen", &result);
    }

    #[test]
    fn test_proper_name_number() {
        assert_eq!(super::proper_name("1"), "One");
        assert_eq!(super::proper_name("2"), "Two");
        assert_eq!(super::proper_name("100"), "OneHundred");
    }
}
