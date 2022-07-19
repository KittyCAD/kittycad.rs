//! A library for converting OpenAPI schemas to Rust types.

pub mod base64;
pub mod error;
pub mod example;
pub mod exts;
pub mod paginate;
pub mod phone_number;
pub mod random;

use std::{collections::BTreeMap, str::FromStr};

use anyhow::Result;
use indexmap::map::IndexMap;
use numeral::Cardinal;

use crate::types::exts::{
    ParameterExt, ParameterSchemaOrContentExt, ReferenceOrExt, StatusCodeExt, TokenStreamExt,
};

/// Generate Rust types from an OpenAPI v3 spec.
pub fn generate_types(spec: &openapiv3::OpenAPI) -> Result<String> {
    // Include the base64 data type for byte data.
    let base64_mod = get_base64_mod()?;

    // Include the paginate data type for pagination.
    let paginate_mod = get_paginate_mod()?;

    // Include the phone number data type for phone numbers.
    let phone_number_mod = get_phone_number_mod()?;

    // Include the error data type for phone numbers.
    let error_mod = get_error_mod()?;

    // Let's start with the components if there are any.
    let mut rendered = quote!(
        //! This module contains the generated types for the library.

        use tabled::Tabled;

        #base64_mod

        #paginate_mod

        #phone_number_mod

        #error_mod
    );

    if let Some(components) = &spec.components {
        // Parse the schemas.
        for (name, schema) in &components.schemas {
            // Let's get the schema from the reference.
            let schema = schema.get_schema_from_reference(spec, true)?;
            // Let's handle all the kinds of schemas.
            let t = render_schema(name, &schema, spec)?;
            // Add it to our rendered types.
            rendered = quote! {
                #rendered

                #t
            };
        }
        // Parse the parameters.
        for (name, parameter) in &components.parameters {
            let schema = (&parameter.expand(spec)?).data()?.format.schema()?;
            // Let's get the schema from the reference.
            let schema = schema.get_schema_from_reference(spec, true)?;
            // Let's handle all the kinds of schemas.
            let t = render_schema(name, &schema, spec)?;
            // Add it to our rendered types.
            rendered = quote! {
                #rendered

                #t
            };
        }

        // Parse the responses.
        for (name, response) in &components.responses {
            render_response(name, &response.expand(spec)?, spec)?;
        }

        // Parse the request bodies.
        for (name, request_body) in &components.request_bodies {
            render_request_body(name, &request_body.expand(spec)?, spec)?;
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
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(a)) => {
            // We don't render arrays, since it is a combination of another type.
            // Let's ensure the items are a reference, otherwise we should render it.
            if let Some(openapiv3::ReferenceOr::Item(s)) = &a.items {
                // We need to render the item.
                return render_schema(name, s, spec);
            }

            Ok(quote!())
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean { .. }) => {
            // We don't render booleans yet, since it is a primitive type.
            Ok(quote!())
        }
        openapiv3::SchemaKind::OneOf { one_of } => {
            render_one_of(name, one_of, &schema.schema_data, spec)
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            render_all_of(name, all_of, &schema.schema_data, spec)
        }
        openapiv3::SchemaKind::AnyOf { any_of } => {
            render_any_of(name, any_of, &schema.schema_data, spec)
        }
        openapiv3::SchemaKind::Not { not } => {
            anyhow::bail!("XXX not not supported yet: {} => {:?}", name, not);
        }
        openapiv3::SchemaKind::Any(any) => render_any(name, any, &schema.schema_data, spec),
    }
}

/// Return the type name for a schema.
pub fn get_type_name_for_schema(
    name: &str,
    schema: &openapiv3::Schema,
    spec: &openapiv3::OpenAPI,
    in_crate: bool,
) -> Result<proc_macro2::TokenStream> {
    let t = match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
            get_type_name_for_string(name, s, &schema.schema_data, in_crate)?
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(n)) => get_type_name_for_number(n)?,
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(i)) => get_type_name_for_integer(i)?,
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) => {
            get_type_name_for_object(name, o, &schema.schema_data, spec, in_crate)?
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(a)) => {
            get_type_name_for_array(name, a, spec, in_crate)?
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean { .. }) => quote!(bool),
        openapiv3::SchemaKind::OneOf { one_of } => {
            if one_of.len() != 1 {
                if name.is_empty() {
                    anyhow::bail!(
                        "XXX one of with more than one value not supported yet when name is empty"
                    );
                } else {
                    let ident = format_ident!("{}", proper_name(name));
                    let t = if in_crate {
                        quote!(#ident)
                    } else {
                        quote!(crate::types::#ident)
                    };
                    return Ok(t);
                }
            }

            let internal_schema = &one_of[0];
            match internal_schema {
                openapiv3::ReferenceOr::Reference { .. } => {
                    get_type_name_from_reference(&internal_schema.reference()?, spec, in_crate)?
                }
                openapiv3::ReferenceOr::Item(s) => {
                    get_type_name_for_schema(name, s, spec, in_crate)?
                }
            }
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            get_type_name_for_all_of(name, all_of, &schema.schema_data, spec, in_crate)?
        }
        openapiv3::SchemaKind::AnyOf { any_of: _ } => get_type_name_for_object(
            name,
            &openapiv3::ObjectType::default(),
            &schema.schema_data,
            spec,
            in_crate,
        )?,
        openapiv3::SchemaKind::Not { not: _ } => {
            anyhow::bail!("XXX not not supported yet");
        }
        openapiv3::SchemaKind::Any(_any) => quote!(serde_json::Value),
    };

    if schema.schema_data.nullable && !t.is_option()? {
        Ok(quote!(Option<#t>))
    } else {
        Ok(t)
    }
}

/// Render a string type.
fn render_string_type(
    name: &str,
    s: &openapiv3::StringType,
    data: &openapiv3::SchemaData,
) -> Result<proc_macro2::TokenStream> {
    if !s.enumeration.is_empty() {
        return render_enum(name, s, data);
    }

    if let Some(ref max_length) = s.max_length {
        log::warn!(
            "XXX max_length not supported here yet: {} => {:?}",
            name,
            max_length
        );
    }

    if let Some(ref min_length) = s.min_length {
        log::warn!(
            "XXX min_length not supported here yet: {} => {:?}",
            name,
            min_length
        );
    }

    // We don't render primitives yet.
    Ok(quote!())
}

/// Get the type name for a string type.
fn get_type_name_for_string(
    name: &str,
    s: &openapiv3::StringType,
    data: &openapiv3::SchemaData,
    in_crate: bool,
) -> Result<proc_macro2::TokenStream> {
    if !s.enumeration.is_empty() {
        // We have an enum type.
        // Get the name for the enum.
        let ident = get_type_name(name, data)?;
        let t = if in_crate {
            quote!(#ident)
        } else {
            quote!(crate::types::#ident)
        };

        return Ok(t);
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
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Byte) => {
            // Use our custom base64 data type.
            if in_crate {
                quote!(base64::Base64Data)
            } else {
                quote!(crate::types::base64::Base64Data)
            }
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
            "ipv6" => quote!(std::net::Ipv6Addr),
            "ip" => quote!(std::net::Ipv4Addr),
            "uri" => quote!(url::Url),
            "uri-template" => quote!(String),
            "url" => quote!(url::Url),
            "email" => quote!(String),
            "phone" => {
                if in_crate {
                    quote!(phone_number::PhoneNumber)
                } else {
                    quote!(crate::types::phone_number::PhoneNumber)
                }
            }
            "uuid" => quote!(uuid::Uuid),
            "hostname" => quote!(String),
            "time" => quote!(chrono::NaiveTime),
            "date" => quote!(chrono::NaiveDate),
            "date-time" => quote!(chrono::DateTime<chrono::Utc>),
            "partial-date-time" => quote!(chrono::NaiveDateTime),
            f => {
                anyhow::bail!("XXX unknown string format {}", f)
            }
        },
    };

    Ok(t)
}

/// Get the type name for a reference.
pub fn get_type_name_from_reference(
    name: &str,
    spec: &openapiv3::OpenAPI,
    in_crate: bool,
) -> Result<proc_macro2::TokenStream> {
    // Get the spec for the reference.
    let schema = if let Some(components) = &spec.components {
        components
            .schemas
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("reference {} not found in components", name))?
            .item()?
    } else {
        anyhow::bail!("no components in spec, cannot get reference");
    };

    get_type_name_for_schema(name, schema, spec, in_crate)
}

/// Get the type name for a number type.
fn get_type_name_for_number(n: &openapiv3::NumberType) -> Result<proc_macro2::TokenStream> {
    let t = match &n.format {
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Float) => {
            quote!(f64)
        }
        openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Double) => {
            quote!(f64)
        }
        openapiv3::VariantOrUnknownOrEmpty::Empty => quote!(f64),
        openapiv3::VariantOrUnknownOrEmpty::Unknown(f) => {
            let width = match f.as_str() {
                "f32" => 32,
                "f64" => 64,
                "money-usd" => 64,
                /* int32 and int64 are build it and parse as the integer type */
                f => anyhow::bail!("unknown number format {}", f),
            };

            match width {
                32 => quote!(f32),
                64 => quote!(f64),
                _ => anyhow::bail!("unknown number width {}", width),
            }
        }
    };

    Ok(t)
}

/// Get the type name for an integer type.
fn get_type_name_for_integer(i: &openapiv3::IntegerType) -> Result<proc_macro2::TokenStream> {
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
                "duration" => {
                    uint = false;
                    width = 64;
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

/// Get the type name for an object type.
fn get_type_name_for_object(
    name: &str,
    o: &openapiv3::ObjectType,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
    in_crate: bool,
) -> Result<proc_macro2::TokenStream> {
    // If the object has no properties, but has additional_properties, just use that
    // for the type.
    if o.properties.is_empty() {
        if let Some(additional_properties) = &o.additional_properties {
            match additional_properties {
                openapiv3::AdditionalProperties::Any(_any) => {
                    // The GitHub API has additional properties that are not actually
                    // properties, but are instead literally empty.
                    // This shows up as `any == true || any == false` in the spec.
                    // We should just ignore these.
                }
                openapiv3::AdditionalProperties::Schema(schema) => {
                    let t = if let Ok(reference) = schema.reference() {
                        let ident = format_ident!("{}", proper_name(&reference));
                        if in_crate {
                            quote!(#ident)
                        } else {
                            quote!(crate::types::#ident)
                        }
                    } else {
                        get_type_name_for_schema(name, schema.item()?, spec, in_crate)?
                    };

                    // The additional properties is a HashMap of the key to the value.
                    // Where the key is a string.
                    return Ok(quote!(std::collections::HashMap<String, #t>));
                }
            }
        }
    }

    if o == &openapiv3::ObjectType::default()
        && name.is_empty()
        && data == &openapiv3::SchemaData::default()
    {
        anyhow::bail!("object `{}` has no properties: {:?} => {:?}", name, o, data);
    }

    // We have an object type.
    // Get the name for the object.
    let ident = get_type_name(name, data)?;
    let t = if in_crate {
        quote!(#ident)
    } else {
        quote!(crate::types::#ident)
    };
    Ok(t)
}

/// Get the type name for an all of type.
fn get_type_name_for_all_of(
    name: &str,
    all_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
    in_crate: bool,
) -> Result<proc_macro2::TokenStream> {
    if all_ofs.len() == 1 {
        let internal_schema = &all_ofs[0];
        return match internal_schema {
            openapiv3::ReferenceOr::Reference { .. } => {
                get_type_name_from_reference(&internal_schema.reference()?, spec, in_crate)
            }
            openapiv3::ReferenceOr::Item(s) => get_type_name_for_schema(name, s, spec, in_crate),
        };
    }

    // This became its own object.
    let ident = get_type_name(name, data)?;
    let t = if in_crate {
        quote!(#ident)
    } else {
        quote!(crate::types::#ident)
    };

    Ok(t)
}

/// Get the type name for an array type.
fn get_type_name_for_array(
    name: &str,
    a: &openapiv3::ArrayType,
    spec: &openapiv3::OpenAPI,
    in_crate: bool,
) -> Result<proc_macro2::TokenStream> {
    // Make sure we have a reference for our type.
    let t = if let Some(ref s) = a.items {
        if let Ok(r) = s.reference() {
            crate::types::get_type_name_from_reference(&r, spec, in_crate)?
        } else {
            // We have an item.
            let item = s.item()?;
            // Get the type name for the item.
            get_type_name_for_schema(name, item, spec, in_crate)?
        }
    } else {
        anyhow::bail!(
            "no items in array, cannot get type name: {} => {:?}",
            name,
            a
        );
    };

    Ok(quote!(Vec<#t>))
}

/// All of validates the value against all the subschemas.
fn render_all_of(
    name: &str,
    all_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    // If it's an all of with length 1, just use the type name.
    if all_ofs.len() == 1 {
        let first = all_ofs[0].item()?;
        // Return the all_of type.
        return render_schema(name, first, spec);
    }

    // The all of needs to be an object with all the values.
    // We want to iterate over each of the subschemas and combine all of the types.
    // We assume all of the subschemas are objects.
    let mut properties: IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>> =
        IndexMap::new();
    let mut required: Vec<String> = Vec::new();
    for all_of in all_ofs {
        // Get the schema for this all of.
        let schema = all_of.get_schema_from_reference(spec, true)?;

        // Ensure the type is an object.
        if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
            for (k, v) in o.properties.iter() {
                properties.insert(k.clone(), v.clone());
            }
            required.extend(o.required.iter().cloned());
        } else {
            // We got something that is not an object.
            // Therefore we need to render this as a one of instead.
            // Since it includes primitive types, we need to render this as a one of.
            return render_one_of(name, all_ofs, data, spec);
        }
    }

    // Let's render the object.
    render_object(
        name,
        &openapiv3::ObjectType {
            properties,
            required,
            ..Default::default()
        },
        data,
        spec,
    )
}

/// Any of validates the value against any (one or more) of the subschemas.
fn render_any_of(
    name: &str,
    any_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    // If it's an any of with length 1, just use the type name.
    if any_ofs.len() == 1 {
        let first = any_ofs[0].item()?;
        // Return the any_of type.
        return render_schema(name, first, spec);
    }

    // The any of needs to be an object with optional values since it can be any (one or more) of multiple types.
    // We want to iterate over each of the subschemas and combine all of the types.
    // We assume all of the subschemas are objects.
    let mut properties: IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>> =
        IndexMap::new();
    for any_of in any_ofs {
        // Get the schema for this any of.
        let schema = any_of.get_schema_from_reference(spec, true)?;

        // Ensure the type is an object.
        if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
            for (k, v) in o.properties.iter() {
                properties.insert(k.clone(), v.clone());
            }
        } else {
            // We got something that is not an object.
            // Therefore we need to render this as a one of instead.
            // Since it includes primitive types, we need to render this as a one of.
            return render_one_of(name, any_ofs, data, spec);
        }
    }

    // Let's render the object.
    render_object(
        name,
        &openapiv3::ObjectType {
            properties,
            ..Default::default()
        },
        data,
        spec,
    )
}

/// Render the full type for a one of.
fn render_one_of(
    name: &str,
    one_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    let description = if let Some(d) = &data.description {
        quote!(#[doc = #d])
    } else {
        quote!()
    };

    // Get the proper name version of the type.
    let one_of_name = get_type_name(name, data)?;

    // Check if this this a one_of with a single item.
    if one_ofs.len() == 1 {
        let first = one_ofs[0].item()?;
        // Return the one_of type.
        return render_schema(name, first, spec);
    }

    let tag_result = get_one_of_tag(one_ofs, spec)?;

    let mut serde_options = Vec::new();
    // Add our tag if we have one.
    if let Some(tag) = &tag_result.tag {
        serde_options.push(quote!(tag = #tag))
    }
    if let Some(content) = &tag_result.content {
        serde_options.push(quote!(content = #content))
    }

    let serde_options = if serde_options.is_empty() {
        quote!()
    } else {
        quote!(#[serde(#(#serde_options),*)] )
    };

    let (_, values) = get_one_of_values(name, one_ofs, spec, &tag_result)?;

    let rendered = quote! {
        #description
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, schemars::JsonSchema, tabled::Tabled)]
        #serde_options
        pub enum #one_of_name {
            #values
        }
    };

    Ok(rendered)
}

// Render the internal enum type for an object.
fn render_enum_object_internal(
    name: &str,
    o: &openapiv3::ObjectType,
    spec: &openapiv3::OpenAPI,
    ignore_key: &str,
) -> Result<proc_macro2::TokenStream> {
    let struct_name = format_ident!("{}", proper_name(name));
    let mut values = quote!();
    for (k, v) in &o.properties {
        if k == ignore_key {
            continue;
        }
        // Get the type name for the schema.
        let mut type_name = if let openapiv3::ReferenceOr::Item(i) = v {
            get_type_name_for_schema(k, &i, spec, true)?
        } else {
            get_type_name_from_reference(&v.reference()?, spec, true)?
        };

        // Check if this type is required.
        if !o.required.contains(k) && !type_name.is_option()? {
            // Make the type optional.
            type_name = quote!(Option<#type_name>);
        }
        let prop_ident = format_ident!("{}", k);

        let prop_value = quote!(
             #prop_ident: #type_name,
        );

        values = quote!(
            #values

            #prop_value
        );
    }

    let rendered = quote! {
        #struct_name {
            #values
        }
    };

    Ok(rendered)
}

/// A holder for our tag and content for enums.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TagContent {
    tag: Option<String>,
    content: Option<String>,
}

fn get_one_of_tag(
    one_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
    spec: &openapiv3::OpenAPI,
) -> Result<TagContent> {
    let mut result: TagContent = Default::default();

    for one_of in one_ofs {
        // Get the schema for this OneOf.
        let schema = one_of.get_schema_from_reference(spec, true)?;
        // Determine if we can do anything fancy with the resulting enum and flatten it.
        if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) = schema.schema_kind {
            // If the object contains a property that is an enum of 1, then that is the tag.
            for (k, v) in &o.properties {
                // Get the schema for the property.
                let inner_schema = if let openapiv3::ReferenceOr::Item(i) = v {
                    let s = &**i;
                    s.clone()
                } else {
                    v.get_schema_from_reference(spec, true)?
                };

                if let openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) =
                    inner_schema.schema_kind
                {
                    if s.enumeration.len() == 1 {
                        result.tag = Some(k.to_string());
                    }
                }
            }
        }
    }

    if let Some(tag) = &result.tag {
        // Check if we also have content.
        // This would be true if the objects only have 2 properties, one of which is the tag and the other is the content.
        for one_of in one_ofs {
            // Get the schema for this OneOf.
            let schema = one_of.get_schema_from_reference(spec, true)?;
            // Determine if we can do anything fancy with the resulting enum and flatten it.
            if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) = schema.schema_kind {
                if o.properties.len() == 2 {
                    for (k, _) in &o.properties {
                        if tag != k {
                            // This is the content.
                            result.content = Some(k.to_string());
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(result)
}

fn get_one_of_values(
    name: &str,
    one_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
    spec: &openapiv3::OpenAPI,
    tag_result: &TagContent,
) -> Result<(
    BTreeMap<String, openapiv3::ReferenceOr<openapiv3::Schema>>,
    proc_macro2::TokenStream,
)> {
    let mut values: BTreeMap<String, openapiv3::ReferenceOr<openapiv3::Schema>> =
        Default::default();
    let mut rendered_value = quote!();

    // If we have a tag and/or content this is pretty simple.
    if let Some(tag) = &tag_result.tag {
        for one_of in one_ofs {
            // Get the schema for this OneOf.
            let schema = one_of.get_schema_from_reference(spec, true)?;
            if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
                // Get the value of this tag.
                let tag_schema = match o.properties.get(tag) {
                    Some(v) => v,
                    None => {
                        anyhow::bail!(
                            "no property `{}` in object, even through we thought we had a tag",
                            tag
                        );
                    }
                };

                // Get the single value from the enum.
                let inner_schema = if let openapiv3::ReferenceOr::Item(i) = tag_schema {
                    let s = &**i;
                    s.clone()
                } else {
                    tag_schema.get_schema_from_reference(spec, true)?
                };

                let tag_name = if let openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) =
                    inner_schema.schema_kind
                {
                    if s.enumeration.len() == 1 {
                        s.enumeration[0]
                            .as_ref()
                            .map(|s| s.to_string())
                            .unwrap_or_default()
                    } else {
                        anyhow::bail!("enumeration for tag `{}` is not a single value", tag);
                    }
                } else {
                    anyhow::bail!("enumeration for tag `{}` is not a string", tag);
                };
                let p = proper_name(&tag_name);
                let n = format_ident!("{}", p);

                if let Some(content) = &tag_result.content {
                    // Get the value of the content.
                    let content_schema = match o.properties.get(content) {
                        Some(v) => v,
                        None => {
                            anyhow::bail!(
                            "no property `{}` in object, even through we thought we had content",
                            tag
                        );
                        }
                    };

                    // Get the single value from the enum.
                    let content_name = if let openapiv3::ReferenceOr::Item(i) = content_schema {
                        let s = &**i;
                        get_type_name_for_schema(name, s, spec, true)?
                    } else {
                        get_type_name_from_reference(&content_schema.reference()?, spec, true)?
                    };

                    // Get the type name for this value.
                    values.insert(p.to_string(), one_of.clone());

                    if p != tag_name {
                        // Rename serde to the correct tag name.
                        rendered_value = quote!(
                            #rendered_value

                            #[serde(rename = #tag_name)]
                            #n(#content_name),
                        );
                    } else {
                        rendered_value = quote!(
                            #rendered_value

                            #n(#content_name),
                        );
                    }
                } else {
                    // Render this object.
                    let content_name = render_enum_object_internal(&tag_name, &o, spec, tag)?;
                    // Get the type name for this value.
                    values.insert(p.to_string(), one_of.clone());

                    if p != tag_name {
                        // Rename serde to the correct tag name.
                        rendered_value = quote!(
                            #rendered_value

                            #[serde(rename = #tag_name)]
                            #content_name,
                        );
                    } else {
                        rendered_value = quote!(
                            #rendered_value

                            #content_name,
                        );
                    }
                }
            }
        }

        // We can return early here, we handled the tagged types.
        return Ok((values, rendered_value));
    }

    // Handle the untagged types.

    for one_of in one_ofs {
        // If we have a tag use the value of that property for the enum.
        let o_type = if let openapiv3::ReferenceOr::Reference { .. } = one_of {
            // If the one of is a reference just use the reference.
            let reference = proper_name(&one_of.reference()?);
            let reference_name = format_ident!("{}", reference);

            values.insert(reference.to_string(), one_of.clone());
            quote!(
                #reference_name(#reference_name),
            )
        } else {
            // We don't have a reference, we have an item.
            // We need to expand the item.
            let rendered_type = get_type_name_for_schema(name, &one_of.expand(spec)?, spec, true)?;

            let n = if let Some(title) = &one_of.expand(spec)?.schema_data.title {
                let p = proper_name(&title);
                p.parse().map_err(|e| anyhow::anyhow!("{}", e))?
            } else {
                rendered_type.clone()
            };
            values.insert(n.rendered()?, one_of.clone());

            quote!(
                #n(#rendered_type),
            )
        };

        rendered_value = quote!(
            #rendered_value

            #o_type
        );
    }

    Ok((values, rendered_value))
}

/// Render the full type for any.
fn render_any(
    name: &str,
    any: &openapiv3::AnySchema,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    // The GitHub API is sometimes missing `type: object`.
    // See: https://github.com/github/rest-api-description/issues/1354
    // If we have properties, we can assume this is an object.
    if !any.properties.is_empty() {
        return render_object(
            name,
            &openapiv3::ObjectType {
                properties: any.properties.clone(),
                required: any.required.clone(),
                additional_properties: any.additional_properties.clone(),
                min_properties: any.min_properties,
                max_properties: any.max_properties,
            },
            data,
            spec,
        );
    }

    anyhow::bail!("could not parse any: {} => {:?}", name, any);
}

/// Render the full type for an object.
fn render_object(
    name: &str,
    o: &openapiv3::ObjectType,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    if let Some(min_properties) = o.min_properties {
        log::warn!(
            "min properties not supported for objects: {} => {:?}",
            name,
            min_properties
        );
    }

    if let Some(max_properties) = o.max_properties {
        log::warn!(
            "max properties not supported for objects: {} => {:?}",
            name,
            max_properties
        );
    }

    let description = if let Some(d) = &data.description {
        quote!(#[doc = #d])
    } else {
        quote!()
    };

    // Get the proper name version of the name of the object.
    let struct_name = get_type_name(name, data)?;

    // If the object has no properties, but has additional_properties, just use that
    // for the type.
    if o.properties.is_empty() {
        if let Some(additional_properties) = &o.additional_properties {
            match additional_properties {
                openapiv3::AdditionalProperties::Any(_any) => {
                    // The GitHub API has additional properties that are not actually
                    // properties, but are instead literally empty.
                    // This shows up as `any == true || any == false` in the spec.
                    // We should just ignore these.
                }
                openapiv3::AdditionalProperties::Schema(schema) => {
                    let rendered = render_schema(name, schema.item()?, spec)?;
                    return Ok(rendered);
                }
            }
        }
    }

    let mut values = quote!();
    for (k, v) in &o.properties {
        let prop = clean_property_name(k);

        // Get the schema for the property.
        let inner_schema = if let openapiv3::ReferenceOr::Item(i) = v {
            let s = &**i;
            s.clone()
        } else {
            v.get_schema_from_reference(spec, true)?
        };

        let prop_desc = if let Some(d) = &inner_schema.schema_data.description {
            quote!(#[doc = #d])
        } else {
            quote!()
        };

        // Get the type name for the schema.
        let mut type_name = if let openapiv3::ReferenceOr::Item(i) = v {
            get_type_name_for_schema(&prop, &i, spec, true)?
        } else {
            get_type_name_from_reference(&v.reference()?, spec, true)?
        };

        // Check if this type is required.
        if !o.required.contains(k) && !type_name.is_option()? {
            // Make the type optional.
            type_name = quote!(Option<#type_name>);
        }
        let prop_ident = format_ident!("{}", prop);

        let prop_value = quote!(
            pub #prop_ident: #type_name,
        );

        let mut serde_props = Vec::<proc_macro2::TokenStream>::new();

        if &prop != k {
            serde_props.push(quote!(
                rename = #k
            ));
        }

        if type_name.is_option()? {
            serde_props.push(quote!(default));
            serde_props.push(quote!(skip_serializing_if = "Option::is_none"));
        }

        let serde_full = if serde_props.is_empty() {
            quote!()
        } else {
            quote!(#[serde(#(#serde_props),*)])
        };

        values = quote!(
            #values

            #prop_desc
            #serde_full
            #prop_value
        );
    }

    // Implement pagination for this type if we should.
    let mut pagination = quote!();
    let pagination_properties = PaginationProperties::from_object(o, spec)?;
    if pagination_properties.can_paginate() {
        let page_item = pagination_properties.item_type(true)?;
        let item_ident = pagination_properties.item_ident()?;
        let next_page_str = pagination_properties.next_page_str()?;
        let next_page_ident = format_ident!("{}", next_page_str);

        pagination = quote!(
            impl crate::types::paginate::Pagination for #struct_name {
                type Item = #page_item;

                fn has_more_pages(&self) -> bool {
                    self.next_page.is_some()
                }

                fn next_page(&self, req: reqwest::Request) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
                    let mut req = req.try_clone().ok_or_else(|| crate::types::error::Error::InvalidRequest(format!("failed to clone request: {:?}", req)))?;
                    req.url_mut().query_pairs_mut()
                        .append_pair(#next_page_str, self.#next_page_ident.as_deref().unwrap_or(""));

                    Ok(req)
                }

                fn items(&self) -> Vec<Self::Item> {
                    self.#item_ident.clone()
                }
            }
        );
    }

    let length: proc_macro2::TokenStream = o
        .properties
        .len()
        .to_string()
        .parse()
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    // Let's implement the tabled trait for the object.
    let mut headers = Vec::new();
    let mut fields = Vec::new();
    for (k, v) in &o.properties {
        let prop = clean_property_name(k);
        let prop_ident = format_ident!("{}", prop);
        headers.push(quote!(#prop.to_string()));

        // Get the schema for the property.
        let inner_schema = if let openapiv3::ReferenceOr::Item(i) = v {
            let s = &**i;
            s.clone()
        } else {
            v.get_schema_from_reference(spec, true)?
        };

        // Get the type name for the schema.
        let type_name = get_type_name_for_schema(&prop, &inner_schema, spec, true)?;
        // Check if this type is required.
        if o.required.contains(k) && type_name.is_string()? {
            fields.push(quote!(
                self.#prop_ident.clone()
            ));
        } else if !o.required.contains(k) && type_name.rendered()? != "phone_number::PhoneNumber" {
            fields.push(quote!(
                if let Some(#prop_ident) = &self.#prop_ident {
                    format!("{:?}", #prop_ident)
                } else {
                    String::new()
                }
            ));
        } else if type_name.rendered()? == "PhoneNumber" {
            fields.push(quote!(
                self.#prop_ident.to_string()
            ));
        } else {
            fields.push(quote!(format!("{:?}", self.#prop_ident)));
        }
    }

    let tabled = quote! {
        impl tabled::Tabled for #struct_name {
            const LENGTH: usize = #length;

            fn fields(&self) -> Vec<String> {
                vec![
                    #(#fields),*
                ]
            }
            fn headers() -> Vec<String> {
                vec![
                    #(#headers),*
                ]
            }
        }
    };

    let rendered = quote! {
        #description
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, schemars::JsonSchema)]
        pub struct #struct_name {
            #values
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?)
            }
        }

        #pagination

        #tabled
    };

    Ok(rendered)
}

/// Render the full type for a response.
fn render_response(
    name: &str,
    response: &openapiv3::Response,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    let mut responses = quote!();

    for (content_name, content) in &response.content {
        if let Some(openapiv3::ReferenceOr::Item(i)) = &content.schema {
            // If the schema is a reference we don't care, since we would have already rendered
            // that reference.
            let rendered = render_schema(&format!("{}_{}", name, content_name), i, spec)?;
            responses = quote!(
                #responses

                #rendered
            );
        }
    }

    Ok(responses)
}

/// Render the full type for a request body.
fn render_request_body(
    name: &str,
    request_body: &openapiv3::RequestBody,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    let mut request_bodies = quote!();

    for (content_name, content) in &request_body.content {
        if let Some(openapiv3::ReferenceOr::Item(i)) = &content.schema {
            // If the schema is a reference we don't care, since we would have already rendered
            // that reference.
            let rendered = render_schema(&format!("{}_{}", name, content_name), i, spec)?;
            request_bodies = quote!(
                #request_bodies

                #rendered
            );
        }
    }

    Ok(request_bodies)
}

/// Clean a property name for an object so we can use it in rust.
pub fn clean_property_name(s: &str) -> String {
    let mut prop = s.trim().to_string();

    // These must come first, otherwise when we go to snake_case it will drop the + and -.
    if prop == "+1" {
        // Account for any weird types.
        prop = "plus_one".to_string()
    } else if prop == "-1" {
        // Account for any weird types.
        prop = "minus_one".to_string()
    }

    prop = inflector::cases::snakecase::to_snake_case(&prop);

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
        || prop == "async"
    {
        prop = format!("{}_", prop);
    } else if prop == "$ref" || prop == "$type" {
        // Account for any weird types.
        prop = format!("{}_", prop.replace('$', ""));
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
        anyhow::bail!("Cannot render empty string enumeration: {}", name);
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
            // GitHub will sometimes put in a null value.
            // But it's fine because they also mark it as null.
            // Just in case tho let's ensure it's marked as nullable.
            if !data.nullable {
                anyhow::bail!("enum `{}` is not nullable, but it has a null value", name);
            }

            // We can continue early.
            continue;
        }

        let e = e.as_ref().unwrap().to_string();

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
            impl std::default::Default for #enum_name {
                fn default() -> Self {
                    #default
                }
            }
        )
    } else if s.enumeration.len() == 1 {
        let default = s.enumeration[0].as_ref().unwrap().to_string();
        let default = format_ident!("{}", proper_name(&default));
        quote!(
            impl std::default::Default for #enum_name {
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
pub fn proper_name(s: &str) -> String {
    if s.is_empty() {
        return "Empty".to_string();
    }

    // Check if s is a number like 1 or 2, etc.
    // If it is a number we want to convert it to a string as follows:
    // 1 => One
    // 2 => Two
    // 100 => OneHundred
    // 2FaDisabled => TwoFaDisabled
    // etc.
    let s = if let Ok(num) = s.parse::<i32>() {
        num.cardinal()
    } else {
        s.to_string()
    };

    // Check if just the first character is a number.
    // Get the first character of the string.
    let first_char = s.chars().next().unwrap();
    let s = if let Ok(num) = first_char.to_string().parse::<i32>() {
        if s.len() == 1 {
            num.cardinal()
        } else if !s.chars().nth(1).unwrap().is_numeric() {
            // Make sure the second character is not a number.
            // If it is, we want to add an underscore to the front of the string.
            s.replace(first_char, &num.cardinal())
        } else {
            s.to_string()
        }
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

/// Get the type name as a string.
pub fn get_text(output: &proc_macro2::TokenStream) -> Result<String> {
    let content = output.to_string();

    Ok(clean_text(&content).replace(' ', ""))
}

/// Format a TokenStream as a string and run `rustfmt` on the result.
pub fn get_text_fmt(output: &proc_macro2::TokenStream) -> Result<String> {
    // Format the file with rustfmt.
    let content = rustfmt_wrapper::rustfmt(output).unwrap();

    Ok(clean_text(&content))
}

fn get_base64_mod() -> Result<proc_macro2::TokenStream> {
    let file = include_str!("base64.rs");
    let stream = proc_macro2::TokenStream::from_str(file).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(quote!(
        pub mod base64 {
            #stream
        }
    ))
}

fn get_paginate_mod() -> Result<proc_macro2::TokenStream> {
    let file = include_str!("paginate.rs");
    let stream = proc_macro2::TokenStream::from_str(file).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(quote!(
        pub mod paginate {
            #stream
        }
    ))
}

fn get_phone_number_mod() -> Result<proc_macro2::TokenStream> {
    let file = include_str!("phone_number.rs");
    let stream = proc_macro2::TokenStream::from_str(file).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(quote!(
        pub mod phone_number {
            #stream
        }
    ))
}

fn get_error_mod() -> Result<proc_macro2::TokenStream> {
    let file = include_str!("error.rs");
    let stream = proc_macro2::TokenStream::from_str(file).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(quote!(
        pub mod error {
            #stream
        }
    ))
}

/// Information about an operation and the attributes that make allow for pagination.
#[derive(Debug, Clone, Default)]
pub struct PaginationProperties {
    /// The property for the next page.
    pub next_page: Option<(String, proc_macro2::TokenStream)>,
    /// The parameter we send back to the server to get the next page.
    pub page_param: Option<(String, proc_macro2::TokenStream)>,
    /// The property for the items.
    pub items: Option<(String, proc_macro2::TokenStream)>,
    /// The path of the operation.
    pub path: Option<String>,
    /// The method of the operation.
    pub method: Option<http::Method>,
}

impl PaginationProperties {
    /// Get the pagination properties for an object.
    pub fn from_object(o: &openapiv3::ObjectType, spec: &openapiv3::OpenAPI) -> Result<Self> {
        let mut properties = PaginationProperties::default();

        for (k, v) in &o.properties {
            let prop = crate::types::clean_property_name(k);
            // Get the schema for the property.
            let inner_schema = if let openapiv3::ReferenceOr::Item(i) = v {
                let s = &**i;
                s.clone()
            } else {
                v.get_schema_from_reference(spec, true)?
            };

            // Get the type name for the schema.
            let mut type_name =
                crate::types::get_type_name_for_schema(&prop, &inner_schema, spec, true)?;

            let type_name_str = crate::types::get_text(&type_name)?;
            // Check if this type is required.
            if !o.required.contains(k) && !type_name.is_option()? {
                // Make the type optional.
                type_name = quote!(Option<#type_name>);
            }

            if is_pagination_property_next_page(&prop) {
                properties.next_page = Some((prop, type_name));
            } else if is_pagination_property_items(&prop, &type_name)? {
                let ident = format_ident!(
                    "{}",
                    type_name_str
                        .trim_start_matches("Vec<")
                        .trim_end_matches('>')
                );
                properties.items = Some((prop, quote!(#ident)));
            }
        }

        Ok(properties)
    }

    /// Get the pagination properties for an operation.
    pub fn from_operation(
        name: &str,
        method: &http::Method,
        op: &openapiv3::Operation,
        spec: &openapiv3::OpenAPI,
    ) -> Result<Self> {
        // If the method is not a get, we can return early.
        if method != http::Method::GET {
            return Ok(PaginationProperties::default());
        }

        // Get the return type for the operation.
        let mut schema = None;
        for (status_code, response) in &op.responses.responses {
            // We only care if the response is a success since this is for the function
            // to return upon success.
            if status_code.is_success() {
                // Then let's get the type for the response.
                let response = response.expand(spec)?;

                // Iterate over all the media types and return the first response.
                for (_name, content) in &response.content {
                    if let Some(s) = &content.schema {
                        schema = Some(s.get_schema_from_reference(spec, true)?);
                        break;
                    }
                }
            }
        }

        // Iterate over the parameters and get the page param.
        let mut page_param = None;
        for param in &op.parameters {
            // Get the parameter.
            let param = param.expand(spec)?;
            if let openapiv3::Parameter::Query {
                parameter_data,
                style: _,
                allow_reserved: _,
                allow_empty_value: _,
            } = param
            {
                // Get the schema for the parameter.
                let s = parameter_data.format.schema()?;

                // Get the type for the parameter.
                let mut t = match s {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        crate::types::get_type_name_from_reference(&s.reference()?, spec, true)?
                    }
                    openapiv3::ReferenceOr::Item(s) => crate::types::get_type_name_for_schema(
                        &parameter_data.name,
                        &s,
                        spec,
                        true,
                    )?,
                };

                // Make it an option if it's optional.
                if !parameter_data.required && !t.is_option()? {
                    t = quote!(Option<#t>);
                }

                if is_pagination_property_param_page(&parameter_data.name) {
                    page_param = Some((parameter_data.name.to_string(), t.clone()));
                }
            }
        }

        let schema = if let Some(schema) = schema {
            schema
        } else {
            // We don't have a response, so we can't get the pagination properties.
            return Ok(PaginationProperties::default());
        };

        let mut properties =
            if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
                // Get the pagination properties for the object.
                PaginationProperties::from_object(o, spec)?
            } else {
                // We don't have an object, so we can't get the pagination properties.
                return Ok(PaginationProperties::default());
            };

        properties.path = Some(name.to_string());
        properties.method = Some(method.clone());
        properties.page_param = page_param;

        Ok(properties)
    }

    /// Return is we can paginate this object.
    pub fn can_paginate(&self) -> bool {
        self.next_page.is_some() && self.items.is_some()
    }

    /// Get the item type for this object.
    pub fn item_type(&self, in_crate: bool) -> Result<proc_macro2::TokenStream> {
        if let Some((_k, v)) = &self.items {
            if in_crate {
                return Ok(v.clone());
            } else {
                return Ok(quote!(crate::types::#v));
            }
        }

        anyhow::bail!("No item type found")
    }

    /// Get the item ident for this object.
    pub fn item_ident(&self) -> Result<proc_macro2::Ident> {
        if let Some((k, _v)) = &self.items {
            return Ok(format_ident!("{}", k));
        }

        anyhow::bail!("No item type found")
    }

    /// Get the item type for this object.
    pub fn next_page_str(&self) -> Result<String> {
        if let Some((k, _v)) = &self.next_page {
            return Ok(k.to_string());
        }

        anyhow::bail!("No next page property found")
    }

    /// Get the item type for this object.
    pub fn page_param_str(&self) -> Result<String> {
        if let Some((k, _v)) = &self.page_param {
            return Ok(k.to_string());
        }

        anyhow::bail!("No next page property found")
    }
}

fn is_pagination_property_next_page(s: &str) -> bool {
    ["next_page", "next"].contains(&s)
}

fn is_pagination_property_param_page(s: &str) -> bool {
    ["page_token", "page"].contains(&s)
}

fn is_pagination_property_items(s: &str, t: &proc_macro2::TokenStream) -> Result<bool> {
    Ok(["items", "data"].contains(&s) && get_text(t)?.starts_with("Vec<"))
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_generate_kittycad_types() {
        let result = super::generate_types(
            &crate::load_json_spec(include_str!("../../../spec.json")).unwrap(),
        )
        .unwrap();
        expectorate::assert_contents("tests/types/kittycad.rs.gen", &result);
    }

    #[test]
    fn test_generate_github_types() {
        let result = super::generate_types(
            &crate::load_json_spec(include_str!("../../tests/api.github.com.json")).unwrap(),
        )
        .unwrap();
        expectorate::assert_contents("tests/types/github.rs.gen", &result);
    }

    #[test]
    fn test_generate_oxide_types() {
        let result = super::generate_types(
            &crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
        )
        .unwrap();
        expectorate::assert_contents("tests/types/oxide.rs.gen", &result);
    }

    #[test]
    fn test_proper_name_number() {
        assert_eq!(super::proper_name("1"), "One");
        assert_eq!(super::proper_name("2"), "Two");
        assert_eq!(super::proper_name("100"), "OneHundred");
        assert_eq!(super::proper_name("2FaDisabled"), "TwoFaDisabled");
    }

    #[test]
    fn test_proper_name_kebab() {
        assert_eq!(super::proper_name("kebab-case"), "KebabCase");
        assert_eq!(
            super::proper_name("webhook-config-insecure-ssl"),
            "WebhookConfigInsecureSsl"
        );
    }

    #[test]
    fn test_clean_property_name() {
        assert_eq!(super::clean_property_name("+1"), "plus_one");
        assert_eq!(super::clean_property_name("-1"), "minus_one");
    }

    #[test]
    fn test_schema_parsing_with_refs() {
        let schema = r##"{
        "description": "A route defines a rule that governs where traffic should be sent based on its destination.",
        "type": "object",
        "properties": {
          "description": {
            "description": "human-readable free-form text about a resource",
            "type": "string"
          },
          "destination": {
            "$ref": "#/components/schemas/RouteDestination"
          },
          "id": {
            "description": "unique, immutable, system-controlled identifier for each resource",
            "type": "string",
            "format": "uuid"
          },
          "kind": {
            "description": "Describes the kind of router. Set at creation. `read-only`",
            "allOf": [
              {
                "$ref": "#/components/schemas/RouterRouteKind"
              }
            ]
          },
          "name": {
            "description": "unique, mutable, user-controlled identifier for each resource",
            "allOf": [
              {
                "$ref": "#/components/schemas/Name"
              }
            ]
          },
          "target": {
            "$ref": "#/components/schemas/RouteTarget"
          },
          "time_created": {
            "description": "timestamp when this resource was created",
            "type": "string",
            "format": "date-time"
          },
          "time_modified": {
            "description": "timestamp when this resource was last modified",
            "type": "string",
            "format": "date-time"
          },
          "vpc_router_id": {
            "description": "The VPC Router to which the route belongs.",
            "type": "string",
            "format": "uuid"
          }
        },
        "required": [
          "description",
          "destination",
          "id",
          "kind",
          "name",
          "target",
          "time_created",
          "time_modified",
          "vpc_router_id"
        ]
      }"##;

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let result = super::render_schema(
            "RouterRoute",
            &schema,
            &crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
        )
        .unwrap();

        expectorate::assert_contents(
            "tests/types/oxide.router-route.rs.gen",
            &super::get_text_fmt(&result).unwrap(),
        );
    }

    #[test]
    fn test_schema_parsing_one_of_with_titles() {
        let schema = r##"{
        "oneOf": [
          {
            "title": "v4",
            "allOf": [
              {
                "$ref": "#/components/schemas/Ipv4Net"
              }
            ]
          },
          {
            "title": "v6",
            "allOf": [
              {
                "$ref": "#/components/schemas/Ipv6Net"
              }
            ]
          }
        ]
      }"##;

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let result = super::render_schema(
            "IpNet",
            &schema,
            &crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
        )
        .unwrap();

        expectorate::assert_contents(
            "tests/types/oxide.ip-net.rs.gen",
            &super::get_text_fmt(&result).unwrap(),
        );
    }

    #[test]
    fn test_schema_parsing_one_of_with_tag_content() {
        let schema = r##"{
        "description": "A `VpcFirewallRuleTarget` is used to specify the set of [`Instance`]s to which a firewall rule applies.",
        "oneOf": [
          {
            "description": "The rule applies to all instances in the VPC",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "enum": [
                  "vpc"
                ]
              },
              "value": {
                "$ref": "#/components/schemas/Name"
              }
            },
            "required": [
              "type",
              "value"
            ]
          },
          {
            "description": "The rule applies to all instances in the VPC Subnet",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "enum": [
                  "subnet"
                ]
              },
              "value": {
                "$ref": "#/components/schemas/Name"
              }
            },
            "required": [
              "type",
              "value"
            ]
          },
          {
            "description": "The rule applies to this specific instance",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "enum": [
                  "instance"
                ]
              },
              "value": {
                "$ref": "#/components/schemas/Name"
              }
            },
            "required": [
              "type",
              "value"
            ]
          },
          {
            "description": "The rule applies to a specific IP address",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "enum": [
                  "ip"
                ]
              },
              "value": {
                "type": "string",
                "format": "ip"
              }
            },
            "required": [
              "type",
              "value"
            ]
          },
          {
            "description": "The rule applies to a specific IP subnet",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "enum": [
                  "ip_net"
                ]
              },
              "value": {
                "$ref": "#/components/schemas/IpNet"
              }
            },
            "required": [
              "type",
              "value"
            ]
          }
        ]
      }"##;

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let result = super::render_schema(
            "VpcFirewallRuleTarget",
            &schema,
            &crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
        )
        .unwrap();

        expectorate::assert_contents(
            "tests/types/oxide.vpc-filewall-rule-target.rs.gen",
            &super::get_text_fmt(&result).unwrap(),
        );
    }

    #[test]
    fn test_schema_parsing_one_of_with_tag_no_content() {
        let schema = r##"{
        "description": "The output from the async API call.",
        "oneOf": [
          {
            "description": "A file conversion.",
            "properties": {
              "completed_at": {
                "description": "The time and date the file conversion was completed.",
                "format": "date-time",
                "nullable": true,
                "title": "DateTime",
                "type": "string"
              },
              "created_at": {
                "description": "The time and date the file conversion was created.",
                "format": "date-time",
                "title": "DateTime",
                "type": "string"
              },
              "error": {
                "description": "The error the function returned, if any.",
                "nullable": true,
                "type": "string"
              },
              "id": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/Uuid"
                  }
                ],
                "description": "The unique identifier of the file conversion.\n\nThis is the same as the API call ID."
              },
              "output": {
                "description": "The converted file, if completed, base64 encoded.",
                "format": "byte",
                "nullable": true,
                "title": "String",
                "type": "string"
              },
              "output_format": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/FileOutputFormat"
                  }
                ],
                "description": "The output format of the file conversion."
              },
              "src_format": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/FileSourceFormat"
                  }
                ],
                "description": "The source format of the file conversion."
              },
              "started_at": {
                "description": "The time and date the file conversion was started.",
                "format": "date-time",
                "nullable": true,
                "title": "DateTime",
                "type": "string"
              },
              "status": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/ApiCallStatus"
                  }
                ],
                "description": "The status of the file conversion."
              },
              "type": {
                "enum": [
                  "FileConversion"
                ],
                "type": "string"
              },
              "updated_at": {
                "description": "The time and date the file conversion was last updated.",
                "format": "date-time",
                "title": "DateTime",
                "type": "string"
              },
              "user_id": {
                "description": "The user ID of the user who created the file conversion.",
                "type": "string"
              }
            },
            "required": [
              "created_at",
              "id",
              "output_format",
              "src_format",
              "status",
              "type",
              "updated_at"
            ],
            "type": "object"
          },
          {
            "description": "A file mass.",
            "properties": {
              "completed_at": {
                "description": "The time and date the mass was completed.",
                "format": "date-time",
                "nullable": true,
                "title": "DateTime",
                "type": "string"
              },
              "created_at": {
                "description": "The time and date the mass was created.",
                "format": "date-time",
                "title": "DateTime",
                "type": "string"
              },
              "error": {
                "description": "The error the function returned, if any.",
                "nullable": true,
                "type": "string"
              },
              "id": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/Uuid"
                  }
                ],
                "description": "The unique identifier of the mass request.\n\nThis is the same as the API call ID."
              },
              "mass": {
                "description": "The resulting mass.",
                "format": "double",
                "nullable": true,
                "type": "number"
              },
              "material_density": {
                "default": 0.0,
                "description": "The material density as denoted by the user.",
                "format": "float",
                "type": "number"
              },
              "src_format": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/FileSourceFormat"
                  }
                ],
                "description": "The source format of the file."
              },
              "started_at": {
                "description": "The time and date the mass was started.",
                "format": "date-time",
                "nullable": true,
                "title": "DateTime",
                "type": "string"
              },
              "status": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/ApiCallStatus"
                  }
                ],
                "description": "The status of the mass."
              },
              "type": {
                "enum": [
                  "FileMass"
                ],
                "type": "string"
              },
              "updated_at": {
                "description": "The time and date the mass was last updated.",
                "format": "date-time",
                "title": "DateTime",
                "type": "string"
              },
              "user_id": {
                "description": "The user ID of the user who created the mass.",
                "type": "string"
              }
            },
            "required": [
              "created_at",
              "id",
              "src_format",
              "status",
              "type",
              "updated_at"
            ],
            "type": "object"
          },
          {
            "description": "A file volume.",
            "properties": {
              "completed_at": {
                "description": "The time and date the volume was completed.",
                "format": "date-time",
                "nullable": true,
                "title": "DateTime",
                "type": "string"
              },
              "created_at": {
                "description": "The time and date the volume was created.",
                "format": "date-time",
                "title": "DateTime",
                "type": "string"
              },
              "error": {
                "description": "The error the function returned, if any.",
                "nullable": true,
                "type": "string"
              },
              "id": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/Uuid"
                  }
                ],
                "description": "The unique identifier of the volume request.\n\nThis is the same as the API call ID."
              },
              "src_format": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/FileSourceFormat"
                  }
                ],
                "description": "The source format of the file."
              },
              "started_at": {
                "description": "The time and date the volume was started.",
                "format": "date-time",
                "nullable": true,
                "title": "DateTime",
                "type": "string"
              },
              "status": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/ApiCallStatus"
                  }
                ],
                "description": "The status of the volume."
              },
              "type": {
                "enum": [
                  "FileVolume"
                ],
                "type": "string"
              },
              "updated_at": {
                "description": "The time and date the volume was last updated.",
                "format": "date-time",
                "title": "DateTime",
                "type": "string"
              },
              "user_id": {
                "description": "The user ID of the user who created the volume.",
                "type": "string"
              },
              "volume": {
                "description": "The resulting volume.",
                "format": "double",
                "nullable": true,
                "type": "number"
              }
            },
            "required": [
              "created_at",
              "id",
              "src_format",
              "status",
              "type",
              "updated_at"
            ],
            "type": "object"
          },
          {
            "description": "A file density.",
            "properties": {
              "completed_at": {
                "description": "The time and date the density was completed.",
                "format": "date-time",
                "nullable": true,
                "title": "DateTime",
                "type": "string"
              },
              "created_at": {
                "description": "The time and date the density was created.",
                "format": "date-time",
                "title": "DateTime",
                "type": "string"
              },
              "density": {
                "description": "The resulting density.",
                "format": "double",
                "nullable": true,
                "type": "number"
              },
              "error": {
                "description": "The error the function returned, if any.",
                "nullable": true,
                "type": "string"
              },
              "id": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/Uuid"
                  }
                ],
                "description": "The unique identifier of the density request.\n\nThis is the same as the API call ID."
              },
              "material_mass": {
                "default": 0.0,
                "description": "The material mass as denoted by the user.",
                "format": "float",
                "type": "number"
              },
              "src_format": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/FileSourceFormat"
                  }
                ],
                "description": "The source format of the file."
              },
              "started_at": {
                "description": "The time and date the density was started.",
                "format": "date-time",
                "nullable": true,
                "title": "DateTime",
                "type": "string"
              },
              "status": {
                "allOf": [
                  {
                    "$ref": "#/components/schemas/ApiCallStatus"
                  }
                ],
                "description": "The status of the density."
              },
              "type": {
                "enum": [
                  "FileDensity"
                ],
                "type": "string"
              },
              "updated_at": {
                "description": "The time and date the density was last updated.",
                "format": "date-time",
                "title": "DateTime",
                "type": "string"
              },
              "user_id": {
                "description": "The user ID of the user who created the density.",
                "type": "string"
              }
            },
            "required": [
              "created_at",
              "id",
              "src_format",
              "status",
              "type",
              "updated_at"
            ],
            "type": "object"
          }
        ]
        }"##;

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let result = super::render_schema(
            "AsyncApiCallOutput",
            &schema,
            &crate::load_json_spec(include_str!("../../../spec.json")).unwrap(),
        )
        .unwrap();

        expectorate::assert_contents(
            "tests/types/kittycad.async-api-call-output.rs.gen",
            &super::get_text_fmt(&result).unwrap(),
        );
    }
}
