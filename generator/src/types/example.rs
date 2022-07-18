//! Modules for generating example code.

use std::fmt::Write as _;

use anyhow::Result;
use chrono::{TimeZone, Timelike};
use rand::Rng;

use crate::types::{exts::ReferenceOrExt, random::Random};

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
                    // Return a random date.
                    let year = rng.gen_range(1970..2100);
                    let month = rng.gen_range(1..13);
                    let day = rng.gen_range(1..29);
                    let hour = rng.gen_range(0..24);
                    let minute = rng.gen_range(0..60);
                    let second = rng.gen_range(0..60);
                    let nanosecond = rng.gen_range(0..1_000_000_000);
                    serde_json::Value::String(
                        chrono::Utc
                            .ymd(year, month, day)
                            .and_hms(hour, minute, second)
                            .with_nanosecond(nanosecond)
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "invalid date: {}-{}-{} {}:{}:{}.{}",
                                    year,
                                    month,
                                    day,
                                    hour,
                                    minute,
                                    second,
                                    nanosecond
                                )
                            })?
                            .to_rfc3339(),
                    )
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => {
                    // Return a random `chrono::NaiveDate`.
                    let year = rng.gen_range(1970..2100);
                    let month = rng.gen_range(1..13);
                    let day = rng.gen_range(1..29);
                    serde_json::Value::String(
                        chrono::NaiveDate::from_ymd(year, month, day).to_string(),
                    )
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
                    // Generate some random bytes.
                    let mut bytes = vec![];
                    for _ in 0..rng.gen_range(8..16) {
                        bytes.push(rng.gen_range(0..256) as u8);
                    }
                    let data = crate::types::base64::Base64Data(bytes);
                    serde_json::Value::String(data.to_string())
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Binary) => {
                    // Generate some random bytes.
                    let bytes = vec![0; rng.gen_range(0..100)];
                    let data = crate::types::base64::Base64Data(bytes);
                    serde_json::Value::String(data.to_string())
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
                            uri.push_str(&format!("{}.", rng.gen_range(0..255)));
                        }
                        uri.pop();
                        serde_json::Value::String(uri)
                    }
                    "url" => serde_json::Value::String(url::Url::random()?.to_string()),
                    "email" => {
                        // Return a random email address.
                        let mut email = String::new();
                        for _ in 0..rng.gen_range(8..16) {
                            email.push_str(&format!("{}.", rng.gen_range(0..255)));
                        }
                        email.pop();
                        email.push_str("@");
                        for _ in 0..rng.gen_range(8..16) {
                            email.push_str(&format!("{}.", rng.gen_range(0..255)));
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
                    "time" => {
                        // Return a random time.
                        // This needs to be a chrono::NaiveTime.
                        let time = chrono::NaiveTime::from_hms_milli(
                            rng.gen_range(0..24),
                            rng.gen_range(0..60),
                            rng.gen_range(0..60),
                            rng.gen_range(0..1_000),
                        );
                        serde_json::Value::String(time.to_string())
                    }
                    "date-time" => {
                        // Return a random date-time.
                        // This needs to be a chrono::NaiveDateTime.
                        let date_time = chrono::Utc
                            .ymd(
                                rng.gen_range(1900..2100),
                                rng.gen_range(1..13),
                                rng.gen_range(1..32),
                            )
                            .and_hms_milli(
                                rng.gen_range(0..24),
                                rng.gen_range(0..60),
                                rng.gen_range(0..60),
                                rng.gen_range(0..1_000),
                            );
                        serde_json::Value::String(date_time.to_rfc3339())
                    }
                    "partial-date-time" => {
                        // This needs to be a chrono::NaiveDateTime.
                        let date_time = chrono::NaiveDateTime::from_timestamp(
                            rng.gen_range(0..1_000_000_000),
                            0,
                        );
                        serde_json::Value::String(date_time.to_string())
                    }
                    f => {
                        anyhow::bail!("XXX unknown string format {}", f)
                    }
                },
            }
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(n)) => match &n.format {
            openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Float) => {
                // Return a random float.
                let f = rng.gen_range(0.0..1234.0);
                serde_json::Value::Number(
                    serde_json::value::Number::from_f64(f)
                        .ok_or_else(|| anyhow::anyhow!("failed to convert {} to f64", f))?,
                )
            }
            openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Double) => {
                // Return a random double.
                let f = rng.gen_range(0.0..1.0);
                serde_json::Value::Number(
                    serde_json::value::Number::from_f64(f)
                        .ok_or_else(|| anyhow::anyhow!("failed to convert {} to f64", f))?,
                )
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
                    32 => {
                        // Generate a random 32-bit number.
                        let i = rng.gen_range(0.0..std::f32::MAX);
                        serde_json::Value::Number(
                            serde_json::value::Number::from_f64(i.into())
                                .ok_or_else(|| anyhow::anyhow!("failed to convert {} to f64", i))?,
                        )
                    }
                    64 => {
                        // Generate a random 64-bit number.
                        let i = rng.gen_range(0.0..std::f64::MAX);
                        serde_json::Value::Number(
                            serde_json::value::Number::from_f64(i)
                                .ok_or_else(|| anyhow::anyhow!("failed to convert {} to f64", i))?,
                        )
                    }
                    _ => anyhow::bail!("unknown number width {}", width),
                }
            }
        },
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(i)) => {
            match &i.format {
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int32) => {
                    // Return a random 32-bit number.
                    let i = rng.gen_range(0..std::i32::MAX);
                    serde_json::Value::Number(
                        serde_json::value::Number::from_f64(i as f64)
                            .ok_or_else(|| anyhow::anyhow!("failed to convert {} to f64", i))?,
                    )
                }
                openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) => {
                    // Return a random 64-bit number.
                    let i = rng.gen_range(0..std::i64::MAX);
                    serde_json::Value::Number(
                        serde_json::value::Number::from_f64(i as f64)
                            .ok_or_else(|| anyhow::anyhow!("failed to convert {} to f64", i))?,
                    )
                }
                openapiv3::VariantOrUnknownOrEmpty::Empty => {
                    // Return an empty number.
                    serde_json::Value::Number(
                        serde_json::value::Number::from_f64(0.0)
                            .ok_or_else(|| anyhow::anyhow!("failed to convert 0.0 to f64"))?,
                    )
                }
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
                            8 => {
                                // Generate a random 8-bit number.
                                let i = rng.gen_range(0..std::u8::MAX);
                                serde_json::Value::Number(
                                    serde_json::value::Number::from_f64(i as f64).ok_or_else(
                                        || anyhow::anyhow!("failed to convert {} to f64", i),
                                    )?,
                                )
                            }
                            16 => {
                                // Generate a random 16-bit number.
                                let i = rng.gen_range(0..std::u16::MAX);
                                serde_json::Value::Number(
                                    serde_json::value::Number::from_f64(i as f64).ok_or_else(
                                        || anyhow::anyhow!("failed to convert {} to f64", i),
                                    )?,
                                )
                            }
                            32 => {
                                // Generate a random 32-bit number.
                                let i = rng.gen_range(0..std::u32::MAX);
                                serde_json::Value::Number(
                                    serde_json::value::Number::from_f64(i as f64).ok_or_else(
                                        || anyhow::anyhow!("failed to convert {} to f64", i),
                                    )?,
                                )
                            }
                            64 => {
                                // Generate a random 64-bit number.
                                let i = rng.gen_range(0..std::u64::MAX);
                                serde_json::Value::Number(
                                    serde_json::value::Number::from_f64(i as f64).ok_or_else(
                                        || anyhow::anyhow!("failed to convert {} to f64", i),
                                    )?,
                                )
                            }
                            _ => anyhow::bail!("unknown uint width {}", width),
                        }
                    } else {
                        match width {
                            8 => {
                                // Generate a random 8-bit number.
                                let i = rng.gen_range(std::i8::MIN..std::i8::MAX);
                                serde_json::Value::Number(
                                    serde_json::value::Number::from_f64(i as f64).ok_or_else(
                                        || anyhow::anyhow!("failed to convert {} to f64", i),
                                    )?,
                                )
                            }
                            16 => {
                                // Generate a random 16-bit number.
                                let i = rng.gen_range(std::i16::MIN..std::i16::MAX);
                                serde_json::Value::Number(
                                    serde_json::value::Number::from_f64(i as f64).ok_or_else(
                                        || anyhow::anyhow!("failed to convert {} to f64", i),
                                    )?,
                                )
                            }
                            32 => {
                                // Generate a random 32-bit number.
                                let i = rng.gen_range(std::i32::MIN..std::i32::MAX);
                                serde_json::Value::Number(
                                    serde_json::value::Number::from_f64(i as f64).ok_or_else(
                                        || anyhow::anyhow!("failed to convert {} to f64", i),
                                    )?,
                                )
                            }
                            64 => {
                                // Generate a random 64-bit number.
                                let i = rng.gen_range(std::i64::MIN..std::i64::MAX);
                                serde_json::Value::Number(
                                    serde_json::value::Number::from_f64(i as f64).ok_or_else(
                                        || anyhow::anyhow!("failed to convert {} to f64", i),
                                    )?,
                                )
                            }
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
            serde_json::Value::Bool(rng.gen())
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
            let i = rng.gen();
            serde_json::Value::Bool(i)
        }
    })
}

#[cfg(test)]
mod test {

    use crate::types::exts::ReferenceOrExt;

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
}
