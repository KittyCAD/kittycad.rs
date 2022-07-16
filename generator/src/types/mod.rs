//! A library for converting OpenAPI schemas to Rust types.

pub mod base64;
pub mod exts;
pub mod paginate;
pub mod phone_number;

use std::str::FromStr;

use anyhow::Result;
use numeral::Cardinal;

use crate::types::exts::{
    ParameterExt, ParameterSchemaOrContentExt, ReferenceOrExt, StatusCodeExt,
};

/// Generate Rust types from an OpenAPI v3 spec.
pub fn generate_types(spec: &openapiv3::OpenAPI) -> Result<String> {
    // Include the base64 data type for byte data.
    let base64_mod = get_base64_mod()?;

    // Include the paginate data type for pagination.
    let paginate_mod = get_paginate_mod()?;

    // Include the phone number data type for phone numbers.
    let phone_number_mod = get_phone_number_mod()?;

    // Let's start with the components if there are any.
    let mut rendered = quote!(
        //! This module contains the generated types for the library.

        use tabled::Tabled;

        #base64_mod

        #paginate_mod

        #phone_number_mod
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
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(_a)) => {
            anyhow::bail!("XXX array not supported yet");
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean { .. }) => {
            // We don't render booleans yet, since it is a primitive type.
            Ok(quote!())
        }
        openapiv3::SchemaKind::OneOf { one_of } => {
            render_one_of(name, one_of, &schema.schema_data, spec)
        }
        openapiv3::SchemaKind::AllOf { all_of: _ } => {
            anyhow::bail!("XXX all of not supported yet");
        }
        openapiv3::SchemaKind::AnyOf { any_of: _ } => {
            anyhow::bail!("XXX any of not supported yet");
        }
        openapiv3::SchemaKind::Not { not: _ } => {
            anyhow::bail!("XXX not not supported yet");
        }
        openapiv3::SchemaKind::Any(any) => {
            anyhow::bail!("XXX any not supported yet: {:?}", any);
        }
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
            if all_of.len() != 1 {
                anyhow::bail!("XXX all of with more than one value not supported yet");
            }

            let internal_schema = &all_of[0];
            match internal_schema {
                openapiv3::ReferenceOr::Reference { .. } => {
                    get_type_name_from_reference(&internal_schema.reference()?, spec, in_crate)?
                }
                openapiv3::ReferenceOr::Item(s) => {
                    get_type_name_for_schema(name, s, spec, in_crate)?
                }
            }
        }
        openapiv3::SchemaKind::AnyOf { any_of: _ } => {
            anyhow::bail!("XXX any of not supported yet");
        }
        openapiv3::SchemaKind::Not { not: _ } => {
            anyhow::bail!("XXX not not supported yet");
        }
        openapiv3::SchemaKind::Any(_any) => quote!(serde_json::Value),
    };

    if schema.schema_data.nullable {
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
            "date-time" => quote!(chrono::DateTime<chrono::Utc>),
            "partial-date-time" => quote!(chrono::DateTime<chrono::Utc>),
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
                openapiv3::AdditionalProperties::Any(any) => {
                    anyhow::bail!(
                        "additional_properties is not supported for any type: {:?}",
                        any
                    );
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
            let reference = format_ident!("{}", r);
            if in_crate {
                quote!(#reference)
            } else {
                quote!(crate::types::#reference)
            }
        } else {
            // We have an item.
            let item = s.item()?;
            // Get the type name for the item.
            get_type_name_for_schema(name, item, spec, in_crate)?
        }
    } else {
        // We have no items.
        anyhow::bail!("no items in array, cannot get type name")
    };

    Ok(quote!(Vec<#t>))
}

/// Render the full type for a one of.
fn render_one_of(
    name: &str,
    one_of: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
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

    // Any additional types we might need for rendering this type.
    let mut additional_types = quote!();

    let mut tag = "".to_string();
    // TODO: should we set the content?, like if its a object w only 2 properties, the one that is
    // not the tag should be the content.

    for o in one_of {
        // Get the schema for this OneOf.
        let schema = o.get_schema_from_reference(spec, true)?;
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
                        tag = k.to_string();
                    }
                }
            }
        }
    }

    let serde_options = if !tag.is_empty() {
        quote!(#[serde(tag = #tag)])
    } else {
        quote!()
    };

    let mut values = quote!();
    for o in one_of {
        // Get the schema for this OneOf.
        let schema = o.get_schema_from_reference(spec, true)?;

        // If we have a tag use the value of that property for the enum.
        let tag_name = if !tag.is_empty() {
            if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
                // Get the value of this tag.
                let v = match o.properties.get(&tag) {
                    Some(v) => v,
                    None => {
                        anyhow::bail!(
                            "no property `{}` in object, even through we thought we had a tag",
                            tag
                        );
                    }
                };

                // Get the single value from the enum.
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
                        s.enumeration[0]
                            .as_ref()
                            .map(|s| s.to_string())
                            .unwrap_or_default()
                    } else {
                        anyhow::bail!("enumeration for tag `{}` is not a single value", tag);
                    }
                } else {
                    anyhow::bail!("enumeration for tag `{}` is not a string", tag);
                }
            } else {
                anyhow::bail!("one of schema `{:?}` is not an object", schema);
            }
        } else {
            "".to_string()
        };

        let o_type = if let openapiv3::ReferenceOr::Reference { .. } = o {
            // If the one of is a reference just use the reference.
            let reference = o.reference()?;
            let reference_name = format_ident!("{}", proper_name(&reference));

            if !tag_name.is_empty() {
                let p = proper_name(&tag_name);
                let n = format_ident!("{}", proper_name(&tag_name));
                if p != tag_name {
                    // Rename serde to the correct tag name.
                    quote!(
                        #[serde(rename = #tag_name)]
                        #n(#reference_name),
                    )
                } else {
                    quote!(
                        #n(#reference_name),
                    )
                }
            } else {
                quote!(
                    #reference_name(#reference_name),
                )
            }
        } else {
            // We don't have a reference, we have an item.
            // We need to expand the item.
            let rendered_type = match &schema.schema_kind {
                openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) => {
                    if tag_name.is_empty() {
                        anyhow::bail!("no tag name for one of `{:?}`", schema);
                    }

                    // Check if we have a component schema already for this type.
                    if let Some(components) = &spec.components {
                        if !components.schemas.contains_key(&tag_name) {
                            // Ensure we have a type for this type.
                            let obj = render_object(&tag_name, o, &schema.schema_data, spec)?;
                            additional_types = quote!(
                                #additional_types

                                #obj
                            );
                        }
                        // TODO: ensure the types are equal with the exception of the tag.
                    }

                    // Return the type name.
                    let ident = format_ident!("{}", proper_name(&tag_name));
                    quote!(#ident)
                }
                _ => get_type_name_for_schema("", &schema, spec, true)?,
            };

            if !tag_name.is_empty() {
                let p = proper_name(&tag_name);
                let n = format_ident!("{}", proper_name(&tag_name));
                if p != tag_name {
                    // Rename serde to the correct tag name.
                    quote!(
                        #[serde(rename = #tag_name)]
                        #n(#rendered_type),
                    )
                } else {
                    quote!(
                        #n(#rendered_type),
                    )
                }
            } else {
                anyhow::bail!("no tag name for one of `{:?}`", schema);
            }
        };

        values = quote!(
            #values

            #o_type
        );
    }

    let rendered = quote! {
        #additional_types

        #description
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, schemars::JsonSchema, tabled::Tabled)]
        #serde_options
        pub enum #one_of_name {
            #values
        }
    };

    Ok(rendered)
}

/// Render the full type for an object.
fn render_object(
    name: &str,
    o: &openapiv3::ObjectType,
    data: &openapiv3::SchemaData,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    if let Some(min_properties) = o.min_properties {
        anyhow::bail!(
            "min properties not supported for objects: {:?}",
            min_properties
        );
    }

    if let Some(max_properties) = o.max_properties {
        anyhow::bail!(
            "max properties not supported for objects: {:?}",
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
                openapiv3::AdditionalProperties::Any(any) => {
                    anyhow::bail!(
                        "additional_properties is not supported for any type: {:?}",
                        any
                    );
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
        let mut type_name = get_type_name_for_schema(&prop, &inner_schema, spec, true)?;
        // Check if this type is required.
        if !o.required.contains(k) && !get_text(&type_name)?.starts_with("Option<") {
            // Make the type optional.
            type_name = quote!(Option<#type_name>);
        }
        let prop_ident = format_ident!("{}", prop);

        let prop_value = quote!(
            pub #prop_ident: #type_name,
        );

        let type_name_text = get_text(&type_name)?;

        let mut serde_props = Vec::<proc_macro2::TokenStream>::new();

        if &prop != k {
            serde_props.push(quote!(
                rename = #k
            ));
        }

        if type_name_text.starts_with("Option<") {
            serde_props.push(quote!(default));
            serde_props.push(quote!(skip_serializing_if = "Option::is_none"));
        }

        let mut tabled_props = quote!();
        if type_name_text.starts_with("Option<")
            || type_name_text.starts_with("Vec<")
            || type_name_text.starts_with("std::collections::HashMap<")
        {
            tabled_props = quote!(#[tabled(skip)]);
        }

        values = quote!(
            #values

            #prop_desc
            #[serde(#(#serde_props),*)]
            #tabled_props
            #prop_value
        );
    }

    // TODO: defaults

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

                fn has_more_pages(&self) -> anyhow::Result<bool> {
                    Ok(self.next_page.is_some())
                }

                fn next_page(&self, req: reqwest::Request) -> anyhow::Result<reqwest::Request> {
                    let mut req = req.try_clone().ok_or_else(|| anyhow::anyhow!("failed to clone request: {:?}", req))?;
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

    let rendered = quote! {
        #description
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, schemars::JsonSchema, tabled::Tabled)]
        pub struct #struct_name {
            #values
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?)
            }
        }

        #pagination
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
            anyhow::bail!("Cannot render None string enumeration");
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
pub fn proper_name(s: &str) -> String {
    if s.is_empty() {
        return "Empty".to_string();
    }

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

/// Information about an operation and the attributes that make allow for pagination.
#[derive(Debug, Clone, Default)]
pub struct PaginationProperties {
    /// The property for the next page.
    pub next_page: Option<(String, proc_macro2::TokenStream)>,
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
            if !o.required.contains(k) && !type_name_str.starts_with("Option<") {
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
}

fn is_pagination_property_next_page(s: &str) -> bool {
    ["next_page", "next"].contains(&s)
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
        expectorate::assert_contents("tests/kittycad.rs.gen", &result);
    }

    #[test]
    // TODO: also make these work but not a priority.
    #[ignore]
    fn test_generate_github_types() {
        let result = super::generate_types(
            &crate::load_json_spec(include_str!("../../tests/api.github.com.json")).unwrap(),
        )
        .unwrap();
        expectorate::assert_contents("tests/github.rs.gen", &result);
    }

    #[test]
    fn test_proper_name_number() {
        assert_eq!(super::proper_name("1"), "One");
        assert_eq!(super::proper_name("2"), "Two");
        assert_eq!(super::proper_name("100"), "OneHundred");
    }
}
