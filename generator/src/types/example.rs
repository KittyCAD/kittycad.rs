//! Modules for generating example code.

use std::fmt::Write as _;

use anyhow::Result;
use rand::Rng;

use crate::types::{
    exts::{ReferenceOrExt, TokenStreamExt},
    random::Random,
};

/// Generates examples for our JSON schema types.
pub fn generate_example_json_from_schema(
    schema: &openapiv3::Schema,
    spec: &openapiv3::OpenAPI,
) -> Result<serde_json::Value> {
    let mut rng = rand::thread_rng();
    Ok(match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
            if !s.enumeration.is_empty() {
                // We have an enum type.
                // Return a random value from the enum.
                let index = rng.gen_range(0..s.enumeration.len());
                return Ok(serde_json::Value::String(
                    s.enumeration[index]
                        .as_ref()
                        .ok_or_else(|| {
                            anyhow::anyhow!("enum type has no value at index: {}", index)
                        })?
                        .to_string(),
                ));
            }

            if s.format.is_empty() {
                let min_length = s.min_length.unwrap_or(0);
                let max_length = s.max_length.unwrap_or(10);

                // Generate a random string.
                let mut s = String::new();
                for _ in 0..rng.gen_range(min_length..max_length) {
                    s.push(rng.gen_range(b'a'..b'z') as char);
                }
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
                    for _ in 0..rng.gen_range(8..16) {
                        password.push(rng.gen_range(b'a'..b'z') as char);
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
                    "uri" => serde_json::Value::String(url::Url::random()?.to_string()),
                    "uri-template" => {
                        // Return a random URI template.
                        let mut uri = String::new();
                        for _ in 0..rng.gen_range(8..16) {
                            write!(uri, "{}.", rng.gen_range(0..255))?;
                        }
                        uri.pop();
                        serde_json::Value::String(uri)
                    }
                    "url" => serde_json::Value::String(url::Url::random()?.to_string()),
                    "email" => {
                        // Return a random email address.
                        let mut email = String::new();
                        for _ in 0..rng.gen_range(8..16) {
                            write!(email, "{}.", rng.gen_range(0..255))?;
                        }
                        email.pop();
                        email.push('@');
                        for _ in 0..rng.gen_range(8..16) {
                            write!(email, "{}.", rng.gen_range(0..255))?;
                        }
                        email.pop();
                        serde_json::Value::String(email)
                    }
                    "phone" => serde_json::Value::String(
                        crate::types::phone_number::PhoneNumber::random()?.to_string(),
                    ),
                    "uuid" => serde_json::Value::String(uuid::Uuid::random()?.to_string()),
                    "hostname" => {
                        // Return a random hostname.
                        let mut hostname = String::new();
                        for _ in 0..rng.gen_range(8..16) {
                            write!(hostname, "{}.", rng.gen_range(0..255))?;
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
                let mut arr = Vec::new();
                for _ in 0..rng.gen_range(0..10) {
                    arr.push(generate_example_json_from_schema(&items, spec)?);
                }
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
            let mut results = Vec::new();
            for s in one_of {
                results.push(generate_example_json_from_schema(
                    &s.get_schema_from_reference(spec, true)?,
                    spec,
                )?);
            }
            let i = rng.gen_range(0..results.len());
            results[i].clone()
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            // Generate a random all of.
            let mut results = Vec::new();
            for s in all_of {
                results.push(generate_example_json_from_schema(
                    &s.get_schema_from_reference(spec, true)?,
                    spec,
                )?);
            }
            let i = rng.gen_range(0..results.len());
            results[i].clone()
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
    name: &str,
    schema: &openapiv3::Schema,
    spec: &openapiv3::OpenAPI,
) -> Result<proc_macro2::TokenStream> {
    Ok(match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
            let random_value = generate_example_json_from_schema(schema, spec)?.to_string();
            let random_value = random_value.trim_start_matches('"').trim_end_matches('"');

            if !s.enumeration.is_empty() {
                let name_ident = crate::types::get_type_name_for_schema(name, schema, spec, false)?;
                // Get a random item from the enum.
                let item_ident = format_ident!("{}", crate::types::proper_name(random_value));

                quote!(#name_ident::#item_ident)
            } else if s.format.is_empty() {
                quote!(#random_value.to_string())
            } else {
                match &s.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::DateTime) => {
                        quote!(
                            chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(#random_value)?
                        )
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => {
                        quote!(
                            chrono::NaiveDate::parse_from_str(#random_value, "%Y-%m-%d")?
                        )
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Password) => {
                        quote!(#random_value.to_string())
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Byte) => {
                        quote!(#random_value.to_string())
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Binary) => {
                        quote!(#random_value.to_string())
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Empty => quote!(""),
                    openapiv3::VariantOrUnknownOrEmpty::Unknown(f) => match f.as_str() {
                        "float" => quote!(#random_value.to_string()),
                        "int64" => quote!(#random_value.to_string()),
                        "uint64" => quote!(#random_value.to_string()),
                        "ipv4" => quote!(std::net::Ipv4Addr::from_str(#random_value)?),
                        "ipv6" => {
                            quote!(std::net::Ipv6Addr::from_str(#random_value)?)
                        }
                        "ip" => {
                            quote!(std::net::IpAddr::from_str(#random_value)?)
                        }
                        "uri" => quote!(url::Url::from_str(#random_value)?),
                        "uri-template" => {
                            quote!(#random_value)
                        }
                        "url" => quote!(url::Url::from_str(#random_value)?),
                        "email" => {
                            quote!(#random_value)
                        }
                        "phone" => quote!(
                            crate::types::phone_number::PhoneNumber::from_str(#random_value)?
                        ),
                        "uuid" => quote!(uuid::Uuid::from_str(#random_value)?),
                        "hostname" => {
                            quote!(#random_value)
                        }
                        "time" => {
                            quote!(chrono::NaiveTime::parse_from_str(#random_value, "%H:%M:%S")?)
                        }
                        "date" => {
                            quote!(chrono::NaiveDate::parse_from_str(#random_value, "%Y-%m-%d")?)
                        }
                        "date-time" => quote!(
                        chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(#random_value)?
                        ),
                        "partial-date-time" => {
                            quote!(chrono::NaiveDateTime::parse_from_str(#random_value, "%Y-%m-%d %H:%M:%S")?)
                        }
                        f => {
                            anyhow::bail!("XXX unknown string format {}", f)
                        }
                    },
                }
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(_)) => {
            let mut t = crate::types::get_type_name_for_schema(name, schema, spec, false)?;
            t = t.strip_option()?;
            quote!(3.14 as #t)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(_)) => {
            let mut t = crate::types::get_type_name_for_schema(name, schema, spec, false)?;
            t = t.strip_option()?;
            quote!(4 as #t)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) => {
            let object_name: proc_macro2::TokenStream =
                name.parse().map_err(|e| anyhow::anyhow!("{}", e))?;
            // Generate a random object.
            let mut args = Vec::new();
            for (k, v) in o.properties.iter() {
                let inner_name = match v {
                    openapiv3::ReferenceOr::Reference { .. } => {
                        crate::types::get_type_name_from_reference(&v.reference()?, spec, false)?
                    }
                    openapiv3::ReferenceOr::Item(s) => {
                        crate::types::get_type_name_for_schema("", s, spec, false)?
                    }
                }
                .strip_option()?
                .rendered()?;

                let inner_schema = v.get_schema_from_reference(spec, true)?;

                let example = generate_example_rust_from_schema(&inner_name, &inner_schema, spec)?;

                let k_ident = format_ident!("{}", crate::types::clean_property_name(k));

                // Check if this type is required.
                if !o.required.contains(k)
                    && inner_name != "crate::types::phone_number::PhoneNumber"
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
                let items = s.get_schema_from_reference(spec, true)?;
                let item_example = generate_example_rust_from_schema(name, &items, spec)?;
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
        openapiv3::SchemaKind::OneOf { one_of: _ } => {
            anyhow::bail!("XXX one of not supported yet");
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
        openapiv3::SchemaKind::Any(_any) => {
            anyhow::bail!("XXX any supported yet");
        }
    })
}

#[cfg(test)]
mod test {
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
        let result = super::generate_example_rust_from_schema("", &schema, &spec).unwrap();

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
        let result = super::generate_example_rust_from_schema("", &schema, &spec).unwrap();

        assert!(result.rendered().unwrap().ends_with("asi32"));
    }

    #[test]
    fn test_generate_example_rust_bool() {
        let spec: openapiv3::OpenAPI = Default::default();
        // Lets get a specific schema.
        let schema = openapiv3::Schema {
            schema_data: Default::default(),
            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Boolean {}),
        };
        let result = super::generate_example_rust_from_schema("", &schema, &spec).unwrap();

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
        let result = super::generate_example_rust_from_schema("", &schema, &spec).unwrap();

        // Make sure it's not a double quoted string.
        assert!(!result.rendered().unwrap().ends_with("\"\""));
    }
}
