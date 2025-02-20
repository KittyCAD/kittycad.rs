//! Modules for generating example code.

use std::fmt::Write as _;

use anyhow::Result;
use indexmap::map::IndexMap;
use openapiv3::Type;
use rand::{Rng, SeedableRng};

use crate::types::{
    exts::{ReferenceOrExt, SchemaRenderExt, TokenStreamExt},
    get_schema_from_any, is_default_property,
    random::Random,
};

/// Generates examples for our JSON schema types.
pub fn generate_example_json_from_schema(
    schema: &openapiv3::Schema,
    spec: &openapiv3::OpenAPI,
) -> Result<serde_json::Value> {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(23456);
    Ok(match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
            if !s.enumeration.is_empty() {
                // We have an enum type.
                // Return a random value from the enum.
                // Remove any null values from the array.
                let values = s.enumeration.to_vec();
                // Remove null values from the array.
                let values = values
                    .into_iter()
                    .filter(|v| v.is_some())
                    .collect::<Vec<_>>();
                let index = rng.random_range(0..values.len());
                return Ok(serde_json::Value::String(
                    values[index]
                        .as_ref()
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "enum type has no value at index `{}`: {:?}",
                                index,
                                values
                            )
                        })?
                        .to_string(),
                ));
            }

            if s.format.is_empty() {
                let min_length = s.min_length.unwrap_or(0);
                let max_length = s.max_length.unwrap_or(10);

                // Generate a random string.
                let s: String = (0..rng.random_range(min_length..max_length))
                    .map(|_| rng.random_range(b'a'..=b'z') as char)
                    .collect();
                return Ok(serde_json::Value::String(s));
            }

            match &s.format {
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::DateTime) => {
                    serde_json::Value::String(
                        chrono::DateTime::<chrono::Utc>::random()?.to_rfc3339(),
                    )
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => {
                    serde_json::Value::String(chrono::NaiveDate::random()?.to_string())
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Password) => {
                    // Return a random password.
                    let mut password = String::new();
                    for _ in 0..rng.random_range(8..16) {
                        password.push(rng.random_range(b'a'..=b'z') as char);
                    }
                    serde_json::Value::String(password)
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Byte) => {
                    serde_json::Value::String(
                        crate::types::base64::Base64Data::random()?.to_string(),
                    )
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Binary) => {
                    serde_json::Value::String(
                        crate::types::base64::Base64Data::random()?.to_string(),
                    )
                }
                openapiv3::VariantOrUnknownOrEmpty::Empty => {
                    // Return an empty string.
                    serde_json::Value::String(String::new())
                }
                openapiv3::VariantOrUnknownOrEmpty::Unknown(f) => match f.as_str() {
                    "float" => serde_json::Value::String(f64::random()?.to_string()),
                    "int64" => serde_json::Value::String(i64::random()?.to_string()),
                    "uint64" => serde_json::Value::String(u64::random()?.to_string()),
                    "ipv4" => serde_json::Value::String(std::net::Ipv4Addr::random()?.to_string()),
                    "ipv6" => serde_json::Value::String(std::net::Ipv6Addr::random()?.to_string()),
                    "ip" => serde_json::Value::String(std::net::IpAddr::random()?.to_string()),
                    "uri" => serde_json::Value::String("https://example.com".to_string()),
                    "uri-template" => {
                        // Return a random URI template.
                        let mut uri = String::new();
                        for _ in 0..rng.random_range(8..16) {
                            write!(uri, "{}.", rng.random_range(0..255))?;
                        }
                        uri.pop();
                        serde_json::Value::String(uri)
                    }
                    "url" => serde_json::Value::String("https://example.com".to_string()),
                    "email" => serde_json::Value::String("email@example.com".to_string()),
                    "phone" => serde_json::Value::String(
                        crate::types::phone_number::PhoneNumber::random()?.to_string(),
                    ),
                    "uuid" => serde_json::Value::String(uuid::Uuid::random()?.to_string()),
                    "hostname" => {
                        // Return a random hostname.
                        let mut hostname = String::new();
                        for _ in 0..rng.random_range(8..16) {
                            write!(hostname, "{}.", rng.random_range(0..255))?;
                        }
                        hostname.pop();
                        serde_json::Value::String(hostname)
                    }
                    "time" => serde_json::Value::String(chrono::NaiveTime::random()?.to_string()),
                    "date" => serde_json::Value::String(chrono::NaiveDate::random()?.to_string()),
                    "date-time" => serde_json::Value::String(
                        chrono::DateTime::<chrono::Utc>::random()?.to_rfc3339(),
                    ),
                    "partial-date-time" => {
                        serde_json::Value::String(chrono::NaiveDateTime::random()?.to_string())
                    }
                    "id" => serde_json::Value::String(uuid::Uuid::random()?.to_string()),
                    f => {
                        anyhow::bail!("XXX unknown string format {}", f)
                    }
                },
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(n)) => match &n.format {
            openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Float) => {
                serde_json::from_str(&f64::random()?.to_string())?
            }
            openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Double) => {
                serde_json::from_str(&f64::random()?.to_string())?
            }
            openapiv3::VariantOrUnknownOrEmpty::Empty => {
                // Return an empty number.
                serde_json::Value::Number(
                    serde_json::value::Number::from_f64(0.0)
                        .ok_or_else(|| anyhow::anyhow!("failed to convert 0.0 to f64"))?,
                )
            }
            openapiv3::VariantOrUnknownOrEmpty::Unknown(f) => {
                let width = match f.as_str() {
                    "f32" => 32,
                    "f64" => 64,
                    "money-usd" => 64,
                    /* int32 and int64 are build it and parse as the integer type */
                    f => anyhow::bail!("unknown number format {}", f),
                };

                match width {
                    32 => serde_json::from_str(&f32::random()?.to_string())?,
                    64 => serde_json::from_str(&f64::random()?.to_string())?,
                    _ => anyhow::bail!("unknown number width {}", width),
                }
            }
        },
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(i)) => {
            match &i.format {
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int32) => {
                    serde_json::from_str(&i32::random()?.to_string())?
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) => {
                    serde_json::from_str(&i64::random()?.to_string())?
                }
                openapiv3::VariantOrUnknownOrEmpty::Empty => serde_json::from_str("0")?,
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
                            8 => serde_json::from_str(&u8::random()?.to_string())?,
                            16 => serde_json::from_str(&u16::random()?.to_string())?,
                            32 => serde_json::from_str(&u32::random()?.to_string())?,
                            64 => serde_json::from_str(&u64::random()?.to_string())?,
                            _ => anyhow::bail!("unknown uint width {}", width),
                        }
                    } else {
                        match width {
                            8 => serde_json::from_str(&i8::random()?.to_string())?,
                            16 => serde_json::from_str(&i16::random()?.to_string())?,
                            32 => serde_json::from_str(&i32::random()?.to_string())?,
                            64 => serde_json::from_str(&i64::random()?.to_string())?,
                            _ => anyhow::bail!("unknown int width {}", width),
                        }
                    }
                }
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) => {
            // Generate a random object.
            let mut obj = serde_json::Map::new();
            for (k, v) in o.properties.iter() {
                let inner_schema = v.get_schema_from_reference(spec, true)?;
                obj.insert(
                    k.clone(),
                    generate_example_json_from_schema(&inner_schema, spec)?,
                );
            }
            serde_json::Value::Object(obj)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(a)) => {
            // Make sure we have a reference for our type.
            if let Some(ref s) = a.items {
                let items = s.get_schema_from_reference(spec, true)?;

                // Generate a random array.
                let arr: Vec<_> = (0..rng.random_range(0..10))
                    .map(|_| generate_example_json_from_schema(&items, spec))
                    .collect::<Result<_, _>>()?;
                serde_json::Value::Array(arr)
            } else {
                // We have no items.
                anyhow::bail!("no items in array, cannot get type name")
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean { .. }) => {
            serde_json::Value::Bool(bool::random()?)
        }
        openapiv3::SchemaKind::OneOf { one_of } => {
            // Generate a random one of.
            let results: Vec<_> = one_of
                .iter()
                .map(|s| {
                    generate_example_json_from_schema(
                        &s.get_schema_from_reference(spec, true)?,
                        spec,
                    )
                })
                .collect::<Result<_, _>>()?;
            let i = rng.random_range(0..results.len());
            results[i].clone()
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            let i = rng.random_range(0..all_of.len());
            generate_example_json_from_schema(
                &all_of[i].get_schema_from_reference(spec, true)?,
                spec,
            )?
        }
        openapiv3::SchemaKind::AnyOf { any_of: _ } => {
            anyhow::bail!("XXX any of not supported yet");
        }
        openapiv3::SchemaKind::Not { not: _ } => {
            anyhow::bail!("XXX not not supported yet");
        }
        openapiv3::SchemaKind::Any(_any) => {
            // Generate any random value.
            serde_json::Value::Bool(bool::random()?)
        }
    })
}

/// Generates example rust code for creating a specific type.
pub fn generate_example_rust_from_schema(
    type_space: &crate::types::TypeSpace,
    name: &str,
    schema: &openapiv3::Schema,
    in_crate: bool,
) -> Result<proc_macro2::TokenStream> {
    Ok(match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
            if !s.enumeration.is_empty() {
                let name_ident = crate::types::get_type_name_for_schema(
                    name,
                    schema,
                    &type_space.spec,
                    in_crate,
                )?
                .strip_option()?;
                // Get a random item from the enum.
                let random_value =
                    generate_example_json_from_schema(schema, &type_space.spec)?.to_string();
                let random_value = random_value.trim_start_matches('"').trim_end_matches('"');
                let item_ident: proc_macro2::TokenStream = crate::types::proper_name(random_value)
                    .parse()
                    .map_err(|err| anyhow::anyhow!("{}", err))?;

                quote!(#name_ident::#item_ident)
            } else if s.format.is_empty() {
                quote!("some-string".to_string())
            } else {
                match &s.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::DateTime) => {
                        quote!(chrono::Utc::now())
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => {
                        quote!(chrono::Utc::now().date_naive())
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Password) => {
                        quote!("some-password".to_string())
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Byte) => {
                        quote!(crate::types::base64::Base64Data(
                            "some-base64-encoded-string".as_bytes().to_vec()
                        ))
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Binary) => {
                        quote!(bytes::Bytes::from("some-string"))
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Empty => quote!(""),
                    openapiv3::VariantOrUnknownOrEmpty::Unknown(f) => match f.as_str() {
                        "float" => quote!("123.245"),
                        "int64" => quote!("123"),
                        "uint64" => quote!("123"),
                        "ipv4" => quote!(std::net::Ipv4Addr::from_str("203.0.113.1")?),
                        "ipv6" => {
                            quote!(std::net::Ipv6Addr::from_str("2001:db8:8:4::2")?)
                        }
                        "ip" => {
                            quote!(std::net::IpAddr::from_str("2001:db8:8:4::2")?)
                        }
                        "uri" => quote!("https://example.com/foo/bar".to_string()),
                        "uri-template" => {
                            quote!("http://example.com/{folder}/{file}.json".to_string())
                        }
                        "url" => quote!("https://example.com/foo/bar".to_string()),
                        "email" => {
                            quote!("email@example.com".to_string())
                        }
                        "phone" => quote!(crate::types::phone_number::PhoneNumber::from_str(
                            "+1555-555-5555"
                        )?),
                        "uuid" => quote!(uuid::Uuid::from_str(
                            "d9797f8d-9ad6-4e08-90d7-2ec17e13471c"
                        )?),
                        "hostname" => {
                            quote!("localhost")
                        }
                        "time" => {
                            quote!(chrono::Utc::now().time())
                        }
                        "date" => {
                            quote!(chrono::Utc::now().date_naive())
                        }
                        "date-time" => quote!(chrono::Utc::now()),
                        "partial-date-time" => {
                            quote!(chrono::Utc::now().naive_utc())
                        }
                        "id" => quote!("d9797f8d-9ad6-4e08-90d7-2ec17e13471c"),
                        f => {
                            anyhow::bail!("XXX unknown string format {}", f)
                        }
                    },
                }
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(_)) => {
            let mut t =
                crate::types::get_type_name_for_schema(name, schema, &type_space.spec, in_crate)?;
            t = t.strip_option()?;
            quote!(3.14 as #t)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(_)) => {
            let mut t =
                crate::types::get_type_name_for_schema(name, schema, &type_space.spec, in_crate)?;
            t = t.strip_option()?;
            quote!(4 as #t)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) => {
            let object_name =
                crate::types::get_type_name_for_schema(name, schema, &type_space.spec, in_crate)?
                    .strip_option()?;

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
                                generate_example_rust_from_schema(
                                    type_space,
                                    &reference,
                                    &schema.expand(&type_space.spec)?,
                                    in_crate,
                                )?
                            } else {
                                generate_example_rust_from_schema(
                                    type_space,
                                    name,
                                    &schema.expand(&type_space.spec)?,
                                    in_crate,
                                )?
                            };

                            // The additional properties is a HashMap of the key to the value.
                            // Where the key is a string.
                            return Ok(quote!(std::collections::HashMap::from([(
                                "some-key".to_string(),
                                #t
                            )])));
                        }
                    }
                }
            }

            // Generate a random object.
            let mut args = Vec::new();
            for (k, v) in o.properties.iter() {
                let inner_name = match v {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        crate::types::get_type_name_from_reference(
                            &v.reference()?,
                            &type_space.spec,
                            true,
                        )?
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        let mut item = s.clone();
                        let mut t_name = crate::types::get_type_name_for_schema(
                            k,
                            &item,
                            &type_space.spec,
                            true,
                        )?;
                        // Check if we should render the schema.
                        if v.should_render()? {
                            // Check if we already have a type with this name.
                            if let Some(rendered) = type_space
                                .types
                                .get(&t_name.strip_option()?.strip_vec()?.rendered()?)
                            {
                                // Since above we are stripping the Vec above, we should also
                                // strip the Vec when we compare the types.
                                if let openapiv3::SchemaKind::Type(openapiv3::Type::Array(
                                    inner_array,
                                )) = &item.schema_kind
                                {
                                    if let Some(openapiv3::ReferenceOr::Item(item_schema)) =
                                        &inner_array.items
                                    {
                                        item = item_schema.clone();
                                    }
                                }
                                if rendered.schema_kind != item.schema_kind
                                    || rendered.schema_data != item.schema_data
                                {
                                    // Update the name of the type.
                                    t_name = crate::types::get_type_name_for_schema(
                                        &format!("{} {}", name, k),
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

                let inner_name_rendered = inner_name.strip_option()?.rendered()?;

                let inner_schema = v.get_schema_from_reference(&type_space.spec, true)?;
                let type_name = crate::types::get_type_name_for_schema(
                    k,
                    &inner_schema,
                    &type_space.spec,
                    true,
                )?;

                let example = generate_example_rust_from_schema(
                    type_space,
                    &inner_name_rendered,
                    &inner_schema,
                    in_crate,
                )?;

                let k_ident = format_ident!("{}", crate::types::clean_property_name(k));

                // Check if this type is required.
                if (!o.required.contains(k) || inner_schema.schema_data.nullable)
                    && !is_default_property(&type_name, &inner_schema.schema_data)?
                    && !example
                        .rendered()?
                        .starts_with("crate::types::phone_number::PhoneNumber")
                    && !example.rendered()?.starts_with("phone_number::PhoneNumber")
                {
                    args.push(quote!(#k_ident: Some(#example)));
                } else {
                    args.push(quote!(#k_ident: #example));
                }
            }

            quote!(#object_name {
                #(#args),*
            })
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(a)) => {
            // Make sure we have a reference for our type.
            if let Some(ref s) = a.items {
                let items = s.get_schema_from_reference(&type_space.spec, true)?;
                let item_example = generate_example_rust_from_schema(
                    type_space,
                    name.trim_start_matches("Vec").trim_end_matches('>'),
                    &items,
                    in_crate,
                )?;
                quote!(vec![#item_example])
            } else {
                // We have no items.
                anyhow::bail!("no items in array, cannot get type name")
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean { .. }) => {
            let b = format_ident!("{}", bool::random()?);
            quote!(#b)
        }
        openapiv3::SchemaKind::OneOf { one_of } => {
            let mut is_enum_with_docs = false;
            let mut enum_schema = openapiv3::StringType {
                enumeration: Default::default(),
                ..Default::default()
            };
            for of in one_of {
                let schema = of.get_schema_from_reference(&type_space.spec, true)?;
                if let openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) = &schema.schema_kind
                {
                    if s.enumeration.len() == 1 {
                        // This is an enum with only one value.
                        // Add the description to our array of descriptions.
                        is_enum_with_docs = true;
                        // Add the value to our enum.
                        enum_schema.enumeration.push(s.enumeration[0].clone());
                    } else {
                        // This is not an object.
                        is_enum_with_docs = false;
                        break;
                    }
                } else {
                    // This is not an object.
                    is_enum_with_docs = false;
                    break;
                }
            }

            let mut is_one_of_nested_object = false;
            for of in one_of {
                let schema = of.get_schema_from_reference(&type_space.spec, true)?;
                if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind
                {
                    if o.properties.len() == 1 {
                        // Check if the property is a nested object.
                        for (_, property) in o.properties.iter() {
                            let property_schema =
                                property.get_schema_from_reference(&type_space.spec, true)?;
                            if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(_)) =
                                &property_schema.schema_kind
                            {
                                is_one_of_nested_object = true;
                            }
                        }
                    } else {
                        // This is not an object.
                        is_one_of_nested_object = false;
                        break;
                    }
                } else {
                    // This is not an object.
                    is_one_of_nested_object = false;
                    break;
                }
            }

            if is_one_of_nested_object {
                let name_ident = crate::types::get_type_name(name, &Default::default())?;

                // Get the render of the first object.
                let mut inner_object = quote!();
                if let Some(of) = one_of.iter().next() {
                    let schema = of.get_schema_from_reference(&type_space.spec, true)?;
                    if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) =
                        &schema.schema_kind
                    {
                        // Check if the property is a nested object.
                        for (property_name, property) in o.properties.iter() {
                            let property_schema =
                                property.get_schema_from_reference(&type_space.spec, true)?;
                            let inner_object_string = generate_example_rust_from_schema(
                                type_space,
                                property_name,
                                &property_schema,
                                in_crate,
                            )?
                            .to_string();
                            let rendered_inner = inner_object_string
                                .trim_start_matches("crate::types::")
                                .trim_start_matches("crate :: types ::");
                            inner_object = rendered_inner
                                .parse()
                                .map_err(|e| anyhow::anyhow!("{}", e))?;
                        }
                    }
                }

                if in_crate {
                    quote!(#name_ident::#inner_object)
                } else {
                    quote!(crate::types::#name_ident::#inner_object)
                }
            } else if is_enum_with_docs {
                let enum_schema = openapiv3::Schema {
                    schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(enum_schema)),
                    schema_data: Default::default(),
                };
                let name_ident = crate::types::get_type_name_for_schema(
                    name,
                    &enum_schema,
                    &type_space.spec,
                    in_crate,
                )?
                .strip_option()?;
                // Get a random item from the enum.
                let random_value =
                    generate_example_json_from_schema(&enum_schema, &type_space.spec)?.to_string();
                let random_value = random_value.trim_start_matches('"').trim_end_matches('"');
                let item_ident: proc_macro2::TokenStream = crate::types::proper_name(random_value)
                    .parse()
                    .map_err(|err| anyhow::anyhow!("{}", err))?;

                quote!(#name_ident::#item_ident)
            } else if one_of.len() == 1 {
                let one_of_item = &one_of[0];
                let one_of_item_schema =
                    one_of_item.get_schema_from_reference(&type_space.spec, true)?;
                generate_example_rust_from_schema(type_space, name, &one_of_item_schema, in_crate)?
            } else {
                let type_name = crate::types::get_type_name_for_schema(
                    name,
                    schema,
                    &type_space.spec,
                    in_crate,
                )?;

                let tag_result = crate::types::get_one_of_tag(one_of, &type_space.spec)?;
                let mut ts = type_space.clone();
                let (values, _) = ts.get_one_of_values(name, one_of, &tag_result, false)?;

                if let Some((mut k, v)) = values.into_iter().next() {
                    if let openapiv3::ReferenceOr::Item(i) = &v {
                        match &i.schema_kind {
                            openapiv3::SchemaKind::Type(Type::Object(o))
                                if o.properties.len() == 1 =>
                            {
                                // Enum variants should be named after their nested object.
                                // E.g. instead of ModelingCmd::ModelingCmd, it should be
                                // ModelingCmd::ModelingCmdCameraDragStart.
                                let variant = o.properties.first().unwrap().0;
                                if variant != "type" {
                                    k.push_str(variant);
                                }
                            }
                            _ => {}
                        }
                    };

                    let enum_name: proc_macro2::TokenStream =
                        k.parse().map_err(|e| anyhow::anyhow!("{}", e))?;

                    if let Some(content) = &tag_result.content {
                        // We want to get the content type from the one_of.
                        if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) =
                            &v.expand(&type_space.spec)?.schema_kind
                        {
                            if let Some(s) = o.properties.get(content) {
                                let example_schema =
                                    s.get_schema_from_reference(&type_space.spec, true)?;
                                let example = generate_example_rust_from_schema(
                                    type_space,
                                    name,
                                    &example_schema,
                                    in_crate,
                                )?;
                                quote!(#type_name::#enum_name(#example))
                            } else {
                                anyhow::bail!(
                                    "no content property `{}` found in one_of: {:?}",
                                    content,
                                    o
                                )
                            }
                        } else {
                            anyhow::bail!("one of item is not an object: {:?}", v)
                        }
                    } else if let Some(tag) = &tag_result.tag {
                        // We want to get the content type from the one_of.
                        let expanded = v.expand(&type_space.spec)?;
                        if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) =
                            &expanded.schema_kind
                        {
                            // Remove the tag from the object.
                            let mut properties = o.properties.clone();
                            properties.shift_remove(tag);
                            let o = openapiv3::ObjectType {
                                properties,
                                required: o.required.clone(),
                                additional_properties: o.additional_properties.clone(),
                                ..Default::default()
                            };

                            // Create our schema.
                            let schema = openapiv3::Schema {
                                schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Object(
                                    o,
                                )),
                                schema_data: expanded.schema_data.clone(),
                            };
                            let example = generate_example_rust_from_schema(
                                type_space,
                                &enum_name.rendered()?,
                                &schema,
                                in_crate,
                            )?
                            .to_string();

                            let rendered = example
                                .trim_start_matches("crate::types::")
                                .trim_start_matches("crate :: types ::");
                            let example: proc_macro2::TokenStream =
                                rendered.parse().map_err(|e| anyhow::anyhow!("{}", e))?;
                            quote!(#type_name::#example)
                        } else {
                            anyhow::bail!("one of item is not an object: {:?}", v)
                        }
                    } else {
                        let inner_name = match &v {
                            openapiv3::ReferenceOr::Reference { .. } => {
                                crate::types::get_type_name_from_reference(
                                    &v.reference()?,
                                    &type_space.spec,
                                    true,
                                )?
                            }
                            openapiv3::ReferenceOr::Item(s) => {
                                crate::types::get_type_name_for_schema(
                                    &k,
                                    s,
                                    &type_space.spec,
                                    true,
                                )?
                            }
                        };
                        let example = generate_example_rust_from_schema(
                            type_space,
                            &inner_name.rendered()?,
                            &v.expand(&type_space.spec)?,
                            in_crate,
                        )?;
                        quote!(#type_name::#enum_name(#example))
                    }
                } else {
                    anyhow::bail!("no one_of values found")
                }
            }
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            if all_of.len() == 1 {
                let all_of_item = &all_of[0];
                let all_of_item_schema =
                    all_of_item.get_schema_from_reference(&type_space.spec, true)?;
                generate_example_rust_from_schema(type_space, name, &all_of_item_schema, in_crate)?
            } else {
                let (properties, required) = match type_space.get_all_of_properties(name, all_of) {
                    Ok(p) => p,
                    Err(err) => {
                        if err.to_string().contains("not an object") {
                            // We got something that is not an object.
                            // Therefore we need to render this as a one of instead.
                            // Since it includes primitive types, we need to render this as a one of.
                            return generate_example_rust_from_schema(
                                type_space,
                                name,
                                &openapiv3::Schema {
                                    schema_data: schema.schema_data.clone(),
                                    schema_kind: openapiv3::SchemaKind::OneOf {
                                        one_of: all_of.clone(),
                                    },
                                },
                                in_crate,
                            );
                        }

                        return Err(err);
                    }
                };

                generate_example_rust_from_schema(
                    type_space,
                    name,
                    &openapiv3::Schema {
                        schema_data: schema.schema_data.clone(),
                        schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Object(
                            openapiv3::ObjectType {
                                properties,
                                required,
                                ..Default::default()
                            },
                        )),
                    },
                    in_crate,
                )?
            }
        }
        openapiv3::SchemaKind::AnyOf { any_of } => {
            if any_of.len() == 1 {
                let any_of_item = &any_of[0];
                let any_of_item_schema =
                    any_of_item.get_schema_from_reference(&type_space.spec, true)?;
                generate_example_rust_from_schema(type_space, name, &any_of_item_schema, in_crate)?
            } else {
                // The any of needs to be an object with optional values since it can be any (one or more) of multiple types.
                // We want to iterate over each of the subschemas and combine all of the types.
                // We assume all of the subschemas are objects.
                let mut properties: IndexMap<
                    String,
                    openapiv3::ReferenceOr<Box<openapiv3::Schema>>,
                > = IndexMap::new();
                for a in any_of {
                    // Get the schema for this any of.
                    let schema = a.get_schema_from_reference(&type_space.spec, true)?;

                    // Ensure the type is an object.
                    if let openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) =
                        &schema.schema_kind
                    {
                        for (k, v) in o.properties.iter() {
                            properties.insert(k.clone(), v.clone());
                        }
                    } else {
                        // We got something that is not an object.
                        // Therefore we need to render this as a one of instead.
                        // Since it includes primitive types, we need to render this as a one of.
                        return generate_example_rust_from_schema(
                            type_space,
                            name,
                            &openapiv3::Schema {
                                schema_data: schema.schema_data.clone(),
                                schema_kind: openapiv3::SchemaKind::OneOf {
                                    one_of: any_of.clone(),
                                },
                            },
                            in_crate,
                        );
                    }
                }

                generate_example_rust_from_schema(
                    type_space,
                    name,
                    &openapiv3::Schema {
                        schema_data: schema.schema_data.clone(),
                        schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Object(
                            openapiv3::ObjectType {
                                properties,
                                ..Default::default()
                            },
                        )),
                    },
                    in_crate,
                )?
            }
        }
        openapiv3::SchemaKind::Not { not: _ } => {
            anyhow::bail!("XXX not not supported yet");
        }
        openapiv3::SchemaKind::Any(any) => {
            if let Some(s) = get_schema_from_any(&schema.schema_data, any) {
                return generate_example_rust_from_schema(type_space, name, &s, in_crate);
            }

            quote!(serde_json::Value::String("some-string".to_string()))
        }
    })
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::types::exts::{ReferenceOrExt, TokenStreamExt};

    #[test]
    fn test_generate_example_file_conversion() {
        let spec = crate::load_json_spec(include_str!("../../../spec.json")).unwrap();
        // Lets get a specific schema.
        let schema = spec
            .components
            .as_ref()
            .unwrap()
            .schemas
            .get("FileConversion")
            .unwrap();
        let result =
            super::generate_example_json_from_schema(&schema.expand(&spec).unwrap(), &spec)
                .unwrap();

        let example_json = serde_json::to_string_pretty(&result).unwrap();

        // TODO: have a better way to test that this object can serialize and deserialize.
        assert!(example_json.contains(r#""completed_at": ""#));
    }

    #[test]
    fn test_generate_example_rust_number() {
        let spec: openapiv3::OpenAPI = Default::default();
        // Lets get a specific schema.
        let schema = openapiv3::Schema {
            schema_data: Default::default(),
            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Number(
                openapiv3::NumberType {
                    format: openapiv3::VariantOrUnknownOrEmpty::Item(
                        openapiv3::NumberFormat::Float,
                    ),
                    ..Default::default()
                },
            )),
        };
        let result = super::generate_example_rust_from_schema(
            &crate::types::TypeSpace {
                spec,
                rendered: Default::default(),
                types: Default::default(),
                opts: Default::default(),
            },
            "",
            &schema,
            false,
        )
        .unwrap();

        assert!(result.rendered().unwrap().ends_with("asf64"));
    }

    #[test]
    fn test_generate_example_rust_integer() {
        let spec: openapiv3::OpenAPI = Default::default();
        // Lets get a specific schema.
        let schema = openapiv3::Schema {
            schema_data: Default::default(),
            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Integer(
                openapiv3::IntegerType {
                    format: openapiv3::VariantOrUnknownOrEmpty::Item(
                        openapiv3::IntegerFormat::Int32,
                    ),
                    ..Default::default()
                },
            )),
        };
        let result = super::generate_example_rust_from_schema(
            &crate::types::TypeSpace {
                spec,
                rendered: Default::default(),
                types: Default::default(),
                opts: Default::default(),
            },
            "",
            &schema,
            false,
        )
        .unwrap();

        assert!(result.rendered().unwrap().ends_with("asi32"));
    }

    #[test]
    fn test_generate_example_rust_bool() {
        let spec: openapiv3::OpenAPI = Default::default();
        // Lets get a specific schema.
        let schema = openapiv3::Schema {
            schema_data: Default::default(),
            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Boolean(Default::default())),
        };
        let result = super::generate_example_rust_from_schema(
            &crate::types::TypeSpace {
                spec,
                rendered: Default::default(),
                types: Default::default(),
                opts: Default::default(),
            },
            "",
            &schema,
            false,
        )
        .unwrap();

        let rendered = result.rendered().unwrap();

        assert!(rendered == "true" || rendered == "false");
    }

    #[test]
    fn test_generate_example_rust_string() {
        let spec: openapiv3::OpenAPI = Default::default();
        // Lets get a specific schema.
        let schema = openapiv3::Schema {
            schema_data: Default::default(),
            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(
                openapiv3::StringType {
                    ..Default::default()
                },
            )),
        };
        let result = super::generate_example_rust_from_schema(
            &crate::types::TypeSpace {
                spec,
                rendered: Default::default(),
                types: Default::default(),
                opts: Default::default(),
            },
            "",
            &schema,
            false,
        )
        .unwrap();

        // Make sure it's not a double quoted string.
        assert!(!result.rendered().unwrap().ends_with("\"\""));
    }

    #[test]
    fn test_generate_example_rust_inline_enum_no_type_space() {
        let spec: openapiv3::OpenAPI = Default::default();
        // Lets get a specific schema.
        let schema = openapiv3::Schema {
            schema_data: Default::default(),
            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Object(
                openapiv3::ObjectType {
                    properties: indexmap::IndexMap::from([(
                        "thing".to_string(),
                        openapiv3::ReferenceOr::Item(Box::new(openapiv3::Schema {
                            schema_data: Default::default(),
                            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                openapiv3::StringType {
                                    enumeration: vec![Some("other".to_string())],
                                    ..Default::default()
                                },
                            )),
                        })),
                    )]),
                    ..Default::default()
                },
            )),
        };
        let result = super::generate_example_rust_from_schema(
            &crate::types::TypeSpace {
                spec,
                rendered: Default::default(),
                types: Default::default(),
                opts: Default::default(),
            },
            "MyType",
            &schema,
            false,
        )
        .unwrap();

        // Make sure it's not a double quoted string.
        assert_eq!(
            result.rendered().unwrap(),
            "crate::types::MyType{thing:Some(crate::types::Thing::Other)}"
        );
    }

    #[test]
    fn test_generate_example_rust_inline_enum_taken_type_space() {
        let spec: openapiv3::OpenAPI = Default::default();
        // Lets get a specific schema.
        let schema = openapiv3::Schema {
            schema_data: Default::default(),
            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Object(
                openapiv3::ObjectType {
                    properties: indexmap::IndexMap::from([(
                        "thing".to_string(),
                        openapiv3::ReferenceOr::Item(Box::new(openapiv3::Schema {
                            schema_data: Default::default(),
                            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                openapiv3::StringType {
                                    enumeration: vec![Some("other".to_string())],
                                    ..Default::default()
                                },
                            )),
                        })),
                    )]),
                    ..Default::default()
                },
            )),
        };
        let result = super::generate_example_rust_from_schema(
            &crate::types::TypeSpace {
                spec,
                rendered: Default::default(),
                types: indexmap::IndexMap::from([(
                    "Thing".to_string(),
                    openapiv3::Schema {
                        schema_data: Default::default(),
                        schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(
                            openapiv3::StringType {
                                enumeration: vec![Some("bleh".to_string())],
                                ..Default::default()
                            },
                        )),
                    },
                )]),
                opts: Default::default(),
            },
            "MyType",
            &schema,
            false,
        )
        .unwrap();

        // Make sure it's not a double quoted string.
        assert_eq!(
            result.rendered().unwrap(),
            "crate::types::MyType{thing:Some(crate::types::MyTypeThing::Other)}"
        );
    }
}
