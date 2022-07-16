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
            println!("{} => {:?}", name, schema.schema_kind);
            // Let's handle all the kinds of schemas.
            match handle_schema(name, &schema) {
                Ok(t) => {
                    rendered = quote! {
                        #rendered

                        #t
                    };
                }
                Err(err) => {
                    // TODO: actually handle the error, but for now we are just testing.
                    println!("ERROR: {}", err);
                }
            }
        }
    }

    Ok(get_text_fmt(&rendered)?)
}

fn handle_schema(name: &str, schema: &openapiv3::Schema) -> Result<proc_macro2::TokenStream> {
    match &schema.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
            handle_string_type(name, s, &schema.schema_data)
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Number(_n)) => {
            println!("{} => Number", name);
            anyhow::bail!("XXX number not supported yet");
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Integer(_i)) => {
            println!("{} => Integer", name);
            anyhow::bail!("XXX integer not supported yet");
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(_o)) => {
            println!("{} => Object", name);
            anyhow::bail!("XXX object not supported yet");
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Array(_a)) => {
            println!("{} => Array", name);
            anyhow::bail!("XXX array not supported yet");
        }
        openapiv3::SchemaKind::Type(openapiv3::Type::Boolean { .. }) => {
            println!("{} => Boolean", name);
            anyhow::bail!("XXX boolean not supported yet");
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

fn handle_string_type(
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

    anyhow::bail!("XXX string not supported yet");

    Ok(match &s.format {
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
            f => {
                anyhow::bail!("XXX unknown string format {}", f)
            }
        },
    })
}

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

    // Get the struct name version of the name of the enum.
    let enum_name = format_ident!(
        "{}",
        if !name.is_empty() {
            struct_name(name)
        } else if let Some(title) = &data.title {
            struct_name(title)
        } else {
            anyhow::bail!(
                "Cannot render enum without name or title: {:?} {:?}",
                s,
                data
            );
        }
    );

    let mut values = quote!();
    for e in &s.enumeration {
        if e.is_none() {
            // TODO: do something for empty(?)
            continue;
        }

        let e = e.as_ref().unwrap().to_string();

        if struct_name(&e).is_empty() || e.trim().is_empty() {
            // TODO: do something for empty(?)
            continue;
        }

        let e_name = format_ident!("{}", struct_name(&e));
        let mut e_value = quote!(
            #e_name,
        );
        if struct_name(&e) != e {
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
        let default = format_ident!("{}", struct_name(&default));
        quote!(
            impl Default for #enum_name {
                fn default() -> Self {
                    #default
                }
            }
        )
    } else if s.enumeration.len() == 1 {
        let default = s.enumeration[0].as_ref().unwrap().to_string();
        let default = format_ident!("{}", struct_name(&default));
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

fn struct_name(s: &str) -> String {
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

pub fn get_text_fmt(output: &proc_macro2::TokenStream) -> Result<String> {
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
    fn test_struct_name_number() {
        assert_eq!(super::struct_name("1"), "One");
        assert_eq!(super::struct_name("2"), "Two");
        assert_eq!(super::struct_name("100"), "OneHundred");
    }
}
