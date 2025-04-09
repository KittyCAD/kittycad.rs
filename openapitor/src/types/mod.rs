//! A library for converting OpenAPI schemas to Rust types.

pub mod base64;
pub mod error;
pub mod example;
pub mod exts;
pub mod multipart;
pub mod paginate;
pub mod phone_number;
pub mod random;

use std::{collections::BTreeMap, str::FromStr};

use anyhow::Result;
use indexmap::map::IndexMap;
use numeral::Cardinal;
use once_cell::sync::Lazy;
use openapiv3::{AnySchema, Schema, SchemaData, SchemaKind};
use regex::Regex;

use crate::types::exts::{
    ParameterExt, ParameterSchemaOrContentExt, ReferenceOrExt, SchemaRenderExt, StatusCodeExt,
    TokenStreamExt,
};

/// Regex to detect weird descriptions in OpenAPI specs.
/// Rustdoc interprets the indented lines as a code block, and tries to run them
/// as code. They aren't always code though.
static LEADING_SPACES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n +").unwrap());

/// Collapse leading spaces at the beginning of lines.
pub fn sanitize_indents(s: &str, name: String) -> std::borrow::Cow<'_, str> {
    let new_str = LEADING_SPACES.replace_all(s, "\n");
    if new_str.is_empty() {
        format!("{}.", name).into()
    } else {
        new_str
    }
}

/// Our collection of all our parsed types.
#[derive(Debug, Clone)]
pub struct TypeSpace {
    /// Our types.
    pub types: IndexMap<String, openapiv3::Schema>,
    /// Our spec.
    pub spec: openapiv3::OpenAPI,
    /// The rendered type space.
    pub rendered: proc_macro2::TokenStream,
    /// The options given to the generator
    pub opts: crate::Opts,
}

/// Generate Rust types from an OpenAPI v3 spec.
pub fn generate_types(spec: &openapiv3::OpenAPI, opts: crate::Opts) -> Result<TypeSpace> {
    // Include the base64 data type for byte data.
    let base64_mod = get_base64_mod()?;

    // Include the multipart type for multipart data.
    let multipart_mod = get_multipart_mod()?;

    // Include the paginate data type for pagination.
    let paginate_mod = get_paginate_mod()?;

    // Include the phone number data type for phone numbers.
    let phone_number_mod = get_phone_number_mod()?;

    // Include the error data type for phone numbers.
    let error_mod = get_error_mod()?;

    // Let's start with the components if there are any.

    // Create our new type space.
    let mut type_space = TypeSpace {
        types: IndexMap::new(),
        spec: spec.clone(),
        rendered: quote!(
            //! This module contains the generated types for the library.

            #[cfg(feature = "tabled")]
            use tabled::Tabled;

            #base64_mod

            #[cfg(feature = "requests")]
            #multipart_mod

            #[cfg(feature = "requests")]
            #paginate_mod

            #phone_number_mod

            #[cfg(feature = "requests")]
            #error_mod
        ),
        opts,
    };

    // The 'components' field of an OpenAPI object stores
    // definitions of various types/shapes of data.
    // We will generate a Rust type for these.
    // If there aren't any of them, then there's no work left to do!
    // So we can return early.
    let Some(components) = &spec.components else {
        return Ok(type_space);
    };

    // Parse the schemas.
    // Schemas are "definition of input and output data types",
    // i.e. the shape of requests to and responses from endpoints.
    //
    // Schemae are either defined as:
    //  - shared schemae that get used in many different places
    //  - a description of some parameter used in a request
    //    (e.g. a path/query/header parameter)
    //  - a description of a request body
    //  - a description of a response body
    //
    // We need to search each of these 4 places in the spec for schemae, and
    // generate Rust types for them.

    let mut schemas = Vec::new();
    // First, search for shared schemae that are reused across
    // parameters/bodies
    for (name, schema) in &components.schemas {
        // Let's get the schema from the reference.
        let schema = schema.get_schema_from_reference(spec, true)?;
        schemas.push((name.to_owned(), schema));
    }

    // Search the parameters for schemae
    for (name, parameter) in &components.parameters {
        let schema = (&parameter.expand(spec)?).data()?.format.schema()?;
        // Let's get the schema from the reference.
        let schema = schema.get_schema_from_reference(spec, true)?;
        schemas.push((name.to_owned(), schema));
    }

    // Search the responses for schemae
    for (name, response) in &components.responses {
        for (content_name, content) in response.expand(spec)?.content {
            if let Some(openapiv3::ReferenceOr::Item(i)) = content.schema {
                // If the schema is a reference we don't care, since we would have already rendered
                // that reference.
                schemas.push((format!("{}_{}", name, content_name), i));
            }
        }
    }

    // Search the requests for schemae
    for (name, request_body) in &components.request_bodies {
        for (content_name, content) in request_body.expand(spec)?.content {
            if let Some(openapiv3::ReferenceOr::Item(i)) = content.schema {
                // If the schema is a reference we don't care, since we would have already rendered
                // that reference.
                schemas.push((format!("{}_{}", name, content_name), i));
            }
        }
    }

    // Each schema becomes a Rust type
    for (name, schema) in schemas {
        type_space.render_schema(&name, &schema)?;
    }

    Ok(type_space)
}

impl TypeSpace {
    /// Pretty render the type space.
    pub fn render(&self) -> Result<String> {
        get_text_fmt(&self.rendered)
    }

    /// Add to our rendered types.
    pub fn add_to_rendered(
        &mut self,
        t: &proc_macro2::TokenStream,
        (name, s): (String, openapiv3::Schema),
    ) -> Result<()> {
        if let Some(item) = self.types.get(&name) {
            // We have a schema with the name already.
            // Let's check if it's the same.
            if &s != item {
                // Get the diff of the schemas.
                let new = serde_json::to_string(&s)?;
                let old = serde_json::to_string(&item)?;
                anyhow::bail!(
                    "Schema {} has changed.\n\nnew: {}\n\nold: {}",
                    name,
                    new,
                    old
                );
            }
        } else {
            // The item does not exist let's add it.
            self.types.insert(name, s);
            let r = &self.rendered;
            self.rendered = quote! {
                #r

                #t
            };
        }
        Ok(())
    }

    /// Render a schema into a Rust type.
    /// This generates the Rust type.
    pub fn render_schema(&mut self, name: &str, schema: &openapiv3::Schema) -> Result<()> {
        match &schema.schema_kind {
            // Don't render primitive types.
            SchemaKind::Type(openapiv3::Type::Number(_))
            | SchemaKind::Type(openapiv3::Type::Boolean { .. })
            | SchemaKind::Type(openapiv3::Type::Integer(_)) => Ok(()),

            // The remaining types are complex (non-primitive)
            // and do need to be rendered.
            SchemaKind::Type(openapiv3::Type::String(s)) => {
                self.render_string_type(name, s, &schema.schema_data)
            }
            SchemaKind::Type(openapiv3::Type::Object(o)) => {
                self.render_object(name, o, &schema.schema_data)
            }
            SchemaKind::Type(openapiv3::Type::Array(a)) => {
                // We don't render arrays, since it is a combination of another type.
                // Let's ensure the items are a reference, otherwise we should render it.
                if let Some(openapiv3::ReferenceOr::Item(s)) = &a.items {
                    // We need to render the item.
                    return self.render_schema(name, s);
                }

                Ok(())
            }
            SchemaKind::OneOf { one_of } => self.render_one_of(name, one_of, &schema.schema_data),
            SchemaKind::AllOf { all_of } => self.render_all_of(name, all_of, &schema.schema_data),
            SchemaKind::AnyOf { any_of } => self.render_any_of(name, any_of, &schema.schema_data),
            SchemaKind::Not { not } => {
                anyhow::bail!("XXX not not supported yet: {} => {:?}", name, not);
            }
            SchemaKind::Any(any) => self.render_any(name, any, &schema.schema_data),
        }
    }

    #[allow(clippy::type_complexity)]
    fn get_all_of_properties(
        &self,
        name: &str,
        all_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
    ) -> Result<(
        IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>>,
        Vec<String>,
    )> {
        let mut properties: IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>> =
            IndexMap::new();
        let mut required: Vec<String> = Vec::new();
        for all_of in all_ofs {
            // Get the schema for this all of.
            let schema = all_of.get_schema_from_reference(&self.spec, true)?;

            // Ensure the type is an object.
            if let SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
                for (k, v) in o.properties.iter() {
                    properties.insert(k.clone(), v.clone());
                }
                required.extend(o.required.iter().cloned());
            } else if let SchemaKind::AllOf { all_of } = &schema.schema_kind {
                // Recurse.
                let (p, r) = self.get_all_of_properties(name, all_of)?;
                properties.extend(p);
                required.extend(r);
            } else {
                anyhow::bail!(
                    "The all of {} is not an object, it is a {:?}",
                    name,
                    schema.schema_kind
                );
            }
        }

        Ok((properties, required))
    }

    /// All of validates the value against all the subschemas.
    fn render_all_of(
        &mut self,
        name: &str,
        all_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
        data: &openapiv3::SchemaData,
    ) -> Result<()> {
        // If it's an all of with length 1, just use the type name.
        if all_ofs.len() == 1 {
            let first = if let openapiv3::ReferenceOr::Item(i) = &all_ofs[0] {
                i.clone()
            } else {
                all_ofs[0].get_schema_from_reference(&self.spec, true)?
            };

            // Return the all_of type.
            return self.render_schema(name, &first);
        }

        // The all of needs to be an object with all the values.
        // We want to iterate over each of the subschemas and combine all of the types.
        // We assume all of the subschemas are objects.
        let (properties, required) = match self.get_all_of_properties(name, all_ofs) {
            Ok(p) => p,
            Err(err) => {
                if err.to_string().contains("not an object") {
                    // We got something that is not an object.
                    // Therefore we need to render this as a one of instead.
                    // Since it includes primitive types, we need to render this as a one of.
                    return self.render_one_of(name, all_ofs, data);
                }

                return Err(err);
            }
        };

        // Let's render the object.
        self.render_object(
            name,
            &openapiv3::ObjectType {
                properties,
                required,
                ..Default::default()
            },
            data,
        )
    }

    /// Any of validates the value against any (one or more) of the subschemas.
    fn render_any_of(
        &mut self,
        name: &str,
        any_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
        data: &openapiv3::SchemaData,
    ) -> Result<()> {
        // If it's an any of with length 1, just use the type name.
        if any_ofs.len() == 1 {
            let first = any_ofs[0].item()?;
            // Return the any_of type.
            return self.render_schema(name, first);
        }

        // The any of needs to be an object with optional values since it can be any (one or more) of multiple types.
        // We want to iterate over each of the subschemas and combine all of the types.
        // We assume all of the subschemas are objects.
        let mut properties: IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>> =
            IndexMap::new();
        for any_of in any_ofs {
            // Get the schema for this any of.
            let schema = any_of.get_schema_from_reference(&self.spec, true)?;

            // Ensure the type is an object.
            if let SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
                for (k, v) in o.properties.iter() {
                    properties.insert(k.clone(), v.clone());
                }
            } else {
                // We got something that is not an object.
                // Therefore we need to render this as a one of instead.
                // Since it includes primitive types, we need to render this as a one of.
                return self.render_one_of(name, any_ofs, data);
            }
        }

        // Let's render the object.
        self.render_object(
            name,
            &openapiv3::ObjectType {
                properties,
                ..Default::default()
            },
            data,
        )
    }

    /// Render a one_of that is a bunch of nested objects.
    fn render_one_of_nested_object(
        &mut self,
        name: &str,
        one_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
        data: &openapiv3::SchemaData,
    ) -> Result<()> {
        // Get the proper name version of the type.
        let one_of_name = get_type_name(name, data)?;

        let description = if let Some(d) = &data.description {
            let d_sanitized = sanitize_indents(d, one_of_name.to_string());
            quote!(#[doc = #d_sanitized])
        } else {
            quote!()
        };

        let mut values = quote!();
        for one_of in one_ofs {
            // Get the first property in the object.
            let schema = one_of.get_schema_from_reference(&self.spec, true)?;
            if let SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
                if o.properties.len() == 1 {
                    // Check if the property is a nested object.
                    for (inner_name, property) in o.properties.iter() {
                        // Let the name be the struct name.
                        let inner_name_ident = format_ident!("{}", proper_name(inner_name));
                        let property_schema =
                            property.get_schema_from_reference(&self.spec, true)?;
                        if let SchemaKind::Type(openapiv3::Type::Object(o)) =
                            &property_schema.schema_kind
                        {
                            let inner_values =
                                self.get_object_values(&inner_name_ident, o, false, None)?;
                            values = quote! {
                                #values
                                #inner_name_ident {
                                    #inner_values
                                },
                            };
                        }
                    }
                }
            }
        }
        let rendered = quote! {
            #description
            #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, schemars::JsonSchema)]
            pub enum #one_of_name {
                #values
            }
        };

        // Add the type to our type space.
        self.add_to_rendered(
            &rendered,
            (
                one_of_name.to_string(),
                openapiv3::Schema {
                    schema_data: data.clone(),
                    schema_kind: SchemaKind::OneOf {
                        one_of: one_ofs.clone(),
                    },
                },
            ),
        )?;

        Ok(())
    }

    /// Render the full type for a one of.
    fn render_one_of(
        &mut self,
        name: &str,
        one_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
        data: &openapiv3::SchemaData,
    ) -> Result<()> {
        // Get the proper name version of the type.
        let one_of_name = get_type_name(name, data)?;

        let description = if let Some(d) = &data.description {
            let d_sanitized = sanitize_indents(d, one_of_name.to_string());
            quote!(#[doc = #d_sanitized])
        } else {
            quote!()
        };

        // Check if this is a one_of with only one enum in each.
        let mut is_enum_with_docs = false;
        let mut enum_docs: Vec<String> = Default::default();
        let mut enum_schema = openapiv3::StringType {
            enumeration: Default::default(),
            ..Default::default()
        };
        for one_of in one_ofs {
            let schema = one_of.get_schema_from_reference(&self.spec, true)?;
            if let SchemaKind::Type(openapiv3::Type::String(s)) = &schema.schema_kind {
                if s.enumeration.len() == 1 {
                    // This is an enum with only one value.
                    // Add the description to our array of descriptions.
                    is_enum_with_docs = true;
                    if let Some(description) = &schema.schema_data.description {
                        enum_docs.push(description.clone());
                    } else {
                        enum_docs.push("".to_string());
                    }
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

        if is_enum_with_docs {
            return self.render_enum(name, &enum_schema, data, enum_docs);
        }

        // Check if we only have objects with 1 item and a nested object.
        let mut is_one_of_nested_object = false;
        for one_of in one_ofs {
            let schema = one_of.get_schema_from_reference(&self.spec, true)?;
            if let SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
                if o.properties.len() == 1 {
                    // Check if the property is a nested object.
                    for (_, property) in o.properties.iter() {
                        let property_schema =
                            property.get_schema_from_reference(&self.spec, true)?;
                        if let SchemaKind::Type(openapiv3::Type::Object(_)) =
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
            return self.render_one_of_nested_object(name, one_ofs, data);
        }

        // Check if this this a one_of with a single item.
        if one_ofs.len() == 1 {
            let first = one_ofs[0].item()?;
            // Return the one_of type.
            return self.render_schema(name, first);
        }

        let tag_result = get_one_of_tag(one_ofs, &self.spec)?;

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

        let (_, values) = self.get_one_of_values(name, one_ofs, &tag_result, true)?;

        let rendered = quote! {
            #description
            #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone, schemars::JsonSchema)]
            #[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
            #serde_options
            pub enum #one_of_name {
                #values
            }
        };

        // Add the type to our type space.
        self.add_to_rendered(
            &rendered,
            (
                one_of_name.to_string(),
                openapiv3::Schema {
                    schema_data: data.clone(),
                    schema_kind: SchemaKind::OneOf {
                        one_of: one_ofs.clone(),
                    },
                },
            ),
        )?;

        Ok(())
    }

    /// Render the full type for any.
    fn render_any(
        &mut self,
        name: &str,
        any: &openapiv3::AnySchema,
        data: &openapiv3::SchemaData,
    ) -> Result<()> {
        if let Some(s) = get_schema_from_any(data, any) {
            return self.render_schema(name, &s);
        }

        // This is a serde_json::Value.
        // We don't need to render it.
        Ok(())
    }

    /// Render the full type for an object.
    fn render_object(
        &mut self,
        name: &str,
        o: &openapiv3::ObjectType,
        data: &openapiv3::SchemaData,
    ) -> Result<()> {
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

        // Get the proper name version of the name of the object.
        let struct_name = get_type_name(name, data)?;

        let description = if let Some(d) = &data.description {
            let d_sanitized = sanitize_indents(d, struct_name.to_string());
            quote!(#[doc = #d_sanitized])
        } else {
            quote!()
        };

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
                    openapiv3::AdditionalProperties::Schema(schema) => match schema.item() {
                        Ok(item) => {
                            return self.render_schema(name, item);
                        }
                        Err(_) => {
                            // We have a reference, ignore this, since it is a reference we don't
                            // need to render it.
                            return Ok(());
                        }
                    },
                }
            }
        }

        let values = self.get_object_values(&struct_name, o, true, None)?;

        // Implement pagination for this type if we should.
        let mut pagination = quote!();
        let pagination_properties = PaginationProperties::from_object(o, &self.spec)?;
        if pagination_properties.can_paginate() {
            let page_item = pagination_properties.item_type(true)?;
            let item_ident = pagination_properties.item_ident()?;
            let next_page_str = pagination_properties.next_page_str()?;
            let next_page_ident = format_ident!("{}", next_page_str);

            if next_page_str == "next_link" {
                pagination = quote!(
                    #[cfg(feature = "requests")]
                    impl crate::types::paginate::Pagination for #struct_name {
                        type Item = #page_item;

                        fn has_more_pages(&self) -> bool {
                            self.#next_page_ident.is_some()
                        }

                        fn next_page_token(&self) -> Option<String> {
                            self.#next_page_ident.clone()
                        }

                        fn next_page(&self, req: reqwest::Request) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
                            let mut req = req.try_clone().ok_or_else(|| crate::types::error::Error::InvalidRequest(format!("failed to clone request: {:?}", req)))?;
                            *req.url_mut() = url::Url::parse( self.next_link.as_deref().unwrap_or(""))
                                .map_err(|_| crate::types::error::Error::InvalidRequest(format!("failed to parse url: {:?}", self.next_link)))?;

                            Ok(req)
                        }

                        fn items(&self) -> Vec<Self::Item> {
                            self.#item_ident.clone()
                        }
                    }
                );
            } else {
                pagination = quote!(
                    #[cfg(feature = "requests")]
                    impl crate::types::paginate::Pagination for #struct_name {
                        type Item = #page_item;

                        fn has_more_pages(&self) -> bool {
                            self.#next_page_ident.is_some()
                        }

                        fn next_page_token(&self) -> Option<String> {
                            self.#next_page_ident.clone()
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
            headers.push(quote!(#prop.into()));

            // Get the schema for the property.
            let inner_schema = if let openapiv3::ReferenceOr::Item(i) = v {
                let s = &**i;
                s.clone()
            } else {
                v.get_schema_from_reference(&self.spec, true)?
            };

            // Get the type name for the schema.
            let type_name = get_type_name_for_schema(&prop, &inner_schema, &self.spec, true)?;
            // Check if this type is required.
            let required = o.required.contains(k)
                || is_default_property(&type_name, &inner_schema.schema_data)?;
            if required && type_name.is_string()? {
                fields.push(quote!(
                    self.#prop_ident.clone().into()
                ));
            } else if !required && type_name.rendered()? != "phone_number::PhoneNumber" {
                fields.push(quote!(
                    if let Some(#prop_ident) = &self.#prop_ident {
                        format!("{:?}", #prop_ident).into()
                    } else {
                        String::new().into()
                    }
                ));
            } else if type_name.rendered()? == "PhoneNumber" {
                fields.push(quote!(
                    self.#prop_ident.to_string().into()
                ));
            } else {
                fields.push(quote!(format!("{:?}", self.#prop_ident).into()));
            }
        }

        let tabled = quote! {
            #[cfg(feature = "tabled")]
            impl tabled::Tabled for #struct_name {
                const LENGTH: usize = #length;

                fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
                    vec![
                        #(#fields),*
                    ]
                }
                fn headers() -> Vec<std::borrow::Cow<'static, str>> {
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

        // Add the type to the list of types, if it doesn't already exist.
        self.add_to_rendered(
            &rendered,
            (
                struct_name.to_string(),
                openapiv3::Schema {
                    schema_data: data.clone(),
                    schema_kind: SchemaKind::Type(openapiv3::Type::Object(o.clone())),
                },
            ),
        )?;

        Ok(())
    }

    fn get_object_values(
        &mut self,
        struct_name: &proc_macro2::Ident,
        o: &openapiv3::ObjectType,
        is_pub: bool,
        ignore_key: Option<&str>,
    ) -> Result<proc_macro2::TokenStream> {
        let mut values = quote!();
        for (k, v) in &o.properties {
            if let Some(ignore_key) = ignore_key {
                if k == ignore_key {
                    continue;
                }
            }
            let prop = clean_property_name(k);

            // Get the schema for the property.
            let inner_schema = if let openapiv3::ReferenceOr::Item(i) = v {
                let s = &**i;
                s.clone()
            } else {
                v.get_schema_from_reference(&self.spec, true)?
            };

            let prop_desc = if let Some(d) = &inner_schema.schema_data.description {
                let d_sanitized = sanitize_indents(d, prop.to_string());
                quote!(#[doc = #d_sanitized])
            } else {
                quote!()
            };

            // Get the type name for the schema.
            let mut type_name = if v.should_render()? {
                // Check if the name for the property is already taken.
                // Make sure there isn't an existing reference with this name.
                let mut t = if let Some(components) = &self.spec.components {
                    if components.schemas.contains_key(&proper_name(&prop)) {
                        proper_name(&format!("{} {}", struct_name, prop))
                    } else {
                        proper_name(&prop)
                    }
                } else {
                    proper_name(&prop)
                };

                let mut should_render = true;

                // Check if the name is already taken.
                if let Some(rendered) = self.types.get(&t) {
                    let mut compare_inner_schema = Box::new(inner_schema.clone());
                    if let SchemaKind::Type(openapiv3::Type::Array(inner_array)) =
                        &inner_schema.schema_kind
                    {
                        if let Some(openapiv3::ReferenceOr::Item(item_schema)) = &inner_array.items
                        {
                            compare_inner_schema = item_schema.clone();
                        }
                    }
                    if *rendered != *compare_inner_schema {
                        // The name is already taken, so we need to make a new name.
                        t = proper_name(&format!("{} {}", struct_name, prop));
                    } else {
                        // When the schema exists but it is equal to the current schema,
                        // we don't need to render it. AND we can use the existing name.
                        should_render = false;
                    }
                }

                if should_render {
                    // Render the schema.
                    self.render_schema(&t, &inner_schema)?;
                }

                get_type_name_for_schema(&t, &inner_schema, &self.spec, true)?
            } else if let openapiv3::ReferenceOr::Item(i) = v {
                get_type_name_for_schema(&prop, i, &self.spec, true)?
            } else {
                get_type_name_from_reference(&v.reference()?, &self.spec, true)?
            };

            if *struct_name == type_name.rendered()? && is_pub {
                // We have a self reference.
                // We need to box it.
                type_name = quote!(Box<#type_name>);
            }

            // Check if this type is required.
            let required = o.required.contains(k)
                || is_default_property(&type_name, &inner_schema.schema_data)?;
            if !required && !type_name.is_option()? {
                // Make the type optional.
                type_name = quote!(Option<#type_name>);
            }
            let prop_ident = format_ident!("{}", prop);

            let prop_value = if is_pub {
                quote!(
                    pub #prop_ident: #type_name,
                )
            } else {
                quote!(
                    #prop_ident: #type_name,
                )
            };

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
            if !o.required.contains(k)
                && is_default_property(&type_name, &inner_schema.schema_data)?
                && !type_name.is_option()?
            {
                serde_props.push(quote!(default));
            }

            if type_name.rendered()? == "Vec<u8>" {
                serde_props.push(quote!(
                    serialize_with = "serde_bytes::serialize",
                    deserialize_with = "serde_bytes::deserialize"
                ));
            }

            // If we have a custom date format  and this is a datetime we need to override deserialize_with
            if self.opts.date_time_format.is_some() {
                if let SchemaKind::Type(openapiv3::Type::String(s)) = inner_schema.schema_kind {
                    if s.format
                        == openapiv3::VariantOrUnknownOrEmpty::Item(
                            openapiv3::StringFormat::DateTime,
                        )
                    {
                        if type_name.is_option()? {
                            serde_props.push(quote!(
                                deserialize_with =
                                    "crate::utils::nullable_date_time_format::deserialize"
                            ));
                        } else {
                            serde_props.push(quote!(
                                deserialize_with = "crate::utils::date_time_format::deserialize"
                            ));
                        }
                    }
                }
            }

            let serde_full = if serde_props.is_empty() {
                quote!()
            } else {
                quote!(#[serde(#(#serde_props),*)])
            };

            let deprecated = if inner_schema.schema_data.deprecated {
                quote!(#[deprecated])
            } else {
                quote!()
            };

            values = quote!(
                #values

                #prop_desc
                #serde_full
                #deprecated
                #prop_value
            );
        }

        Ok(values)
    }

    /// Render a string type.
    fn render_string_type(
        &mut self,
        name: &str,
        s: &openapiv3::StringType,
        data: &openapiv3::SchemaData,
    ) -> Result<()> {
        if !s.enumeration.is_empty() {
            return self.render_enum(name, s, data, vec![]);
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
        Ok(())
    }

    /// Render the full type for an enum.
    fn render_enum(
        &mut self,
        name: &str,
        s: &openapiv3::StringType,
        data: &openapiv3::SchemaData,
        // The additional doc strings for the enum if they exist.
        additional_docs: Vec<String>,
    ) -> Result<()> {
        if s.enumeration.is_empty() {
            anyhow::bail!("Cannot render empty string enumeration: {}", name);
        }

        // Get the proper name version of the name of the enum.
        let enum_name = get_type_name(name, data)?;

        let description = if let Some(d) = &data.description {
            let d_sanitized = sanitize_indents(d, enum_name.to_string());
            quote!(#[doc = #d_sanitized])
        } else {
            quote!()
        };

        let mut values = quote!();
        for (index, e) in s.enumeration.iter().enumerate() {
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

            // Check if we have a description for the enum.
            if let Some(description) = additional_docs.get(index) {
                if !description.is_empty() {
                    let description_sanitized = sanitize_indents(description, proper_name(&e));
                    e_value = quote!(
                        #[doc = #description_sanitized]
                        #e_value
                    );
                }
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
                        #enum_name::#default
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
            #[derive(serde::Serialize, serde::Deserialize, PartialEq, Hash, Debug, Clone, schemars::JsonSchema, parse_display::FromStr, parse_display::Display)]
            #[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
            #[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
            pub enum #enum_name {
                #values
            }

            #default
        };

        // Add the type to the list of types, if it doesn't already exist.
        self.add_to_rendered(
            &rendered,
            (
                enum_name.to_string(),
                openapiv3::Schema {
                    schema_data: data.clone(),
                    schema_kind: SchemaKind::Type(openapiv3::Type::String(s.clone())),
                },
            ),
        )?;

        Ok(())
    }

    // Render the internal enum type for an object.
    fn render_enum_object_internal(
        &mut self,
        name: &str,
        o: &openapiv3::ObjectType,
        ignore_key: &str,
    ) -> Result<proc_macro2::TokenStream> {
        let proper_name = proper_name(name);
        let struct_name = format_ident!("{}", proper_name);

        // Check if we have an existing object with this name.
        if let Some(components) = &self.spec.components {
            if let Some(schema) = components.schemas.get(name) {
                if let SchemaKind::Type(openapiv3::Type::Object(existing)) =
                    &schema.expand(&self.spec)?.schema_kind
                {
                    let mut modified_properties = o.properties.clone();
                    modified_properties.shift_remove(ignore_key);
                    // Check if we have the same properties.
                    if modified_properties == existing.properties {
                        // We have the same properties.
                        // We can just use the existing object.
                        return Ok(quote!(#struct_name(#struct_name)));
                    }
                }
            }
        }

        let inner_values = self.get_object_values(&struct_name, o, false, Some(ignore_key))?;
        let rendered = quote! {
            #struct_name {
                #inner_values
            }
        };

        Ok(rendered)
    }

    fn get_one_of_values(
        &mut self,
        name: &str,
        one_ofs: &Vec<openapiv3::ReferenceOr<openapiv3::Schema>>,
        tag_result: &TagContent,
        should_render: bool,
    ) -> Result<(
        BTreeMap<String, openapiv3::ReferenceOr<openapiv3::Schema>>,
        proc_macro2::TokenStream,
    )> {
        let mut values: BTreeMap<String, openapiv3::ReferenceOr<openapiv3::Schema>> =
            Default::default();
        let mut rendered_value = quote!();
        let (mut name, original_name) = (name.to_string(), name);

        // If we have a tag and/or content this is pretty simple.
        if let Some(tag) = &tag_result.tag {
            for one_of in one_ofs {
                // Get the schema for this OneOf.
                let schema = one_of.get_schema_from_reference(&self.spec, true)?;
                let mut description = if let Some(d) = &schema.schema_data.description {
                    let d_sanitized = sanitize_indents(d, name.to_string());
                    quote!(#[doc = #d_sanitized])
                } else {
                    quote!()
                };

                if let SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
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
                        tag_schema.get_schema_from_reference(&self.spec, true)?
                    };

                    let tag_name = if let SchemaKind::Type(openapiv3::Type::String(s)) =
                        inner_schema.schema_kind
                    {
                        if s.enumeration.len() == 1 {
                            s.enumeration[0]
                                .as_ref()
                                .map(|s| s.to_string())
                                .unwrap_or_default()
                        } else {
                            anyhow::bail!(
                                "enumeration for tag `{}` is not a single value: {:?}",
                                tag,
                                one_of
                            );
                        }
                    } else {
                        anyhow::bail!("enumeration for tag `{}` is not a string", tag);
                    };
                    let p = proper_name(&tag_name);
                    let n = format_ident!("{}", p);

                    if description.is_empty() {
                        description = if let Some(d) = &inner_schema.schema_data.description {
                            let d_sanitized = sanitize_indents(d, p.to_string());
                            quote!(#[doc = #d_sanitized])
                        } else {
                            quote!()
                        };
                    }

                    if let Some(content) = &tag_result.content {
                        let content_type_name = proper_name(&format!("{}_{}", tag_name, content));
                        // Get the value of the content.
                        let content_schema = match o.properties.get(content) {
                            Some(v) => v,
                            None => {
                                anyhow::bail!(
                                    "no property `{}` in object, even through we thought we had \
                                     content",
                                    content
                                );
                            }
                        };

                        let mut enum_object_internal = None;

                        // Get the single value from the enum.
                        let content_name = if let openapiv3::ReferenceOr::Item(i) = content_schema {
                            let s = &**i;
                            if let SchemaKind::Type(openapiv3::Type::Object(o)) = &s.schema_kind {
                                enum_object_internal =
                                    Some(self.render_enum_object_internal(&p, o, tag)?);
                            }
                            get_type_name_for_schema(&content_type_name, s, &self.spec, true)?
                        } else {
                            get_type_name_from_reference(
                                &content_schema.reference()?,
                                &self.spec,
                                true,
                            )?
                        };

                        // Get the type name for this value.
                        values.insert(p.to_string(), one_of.clone());

                        if let Some(enum_object_internal) = enum_object_internal {
                            if p != tag_name {
                                rendered_value = quote!(
                                    #rendered_value

                                    #description
                                    #[serde(rename = #tag_name)]
                                    #enum_object_internal,
                                );
                            } else {
                                rendered_value = quote!(
                                    #rendered_value

                                    #description
                                    #enum_object_internal,
                                );
                            }
                        } else if p != tag_name {
                            // Rename serde to the correct tag name.
                            rendered_value = quote!(
                                #rendered_value

                                #description
                                #[serde(rename = #tag_name)]
                                #n(#content_name),
                            );
                        } else {
                            rendered_value = quote!(
                                #rendered_value

                                #description
                                #n(#content_name),
                            );
                        }
                    } else {
                        // Render this object.
                        let content_name = self.render_enum_object_internal(&tag_name, o, tag)?;
                        // Get the type name for this value.
                        values.insert(p.to_string(), one_of.clone());

                        if p != tag_name {
                            // Rename serde to the correct tag name.
                            rendered_value = quote!(
                                #rendered_value

                                #description
                                #[serde(rename = #tag_name)]
                                #content_name,
                            );
                        } else {
                            rendered_value = quote!(
                                #rendered_value

                                #description
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
            let expanded_one_of = one_of.expand(&self.spec)?;
            if one_of.should_render()? && should_render {
                // Render the schema.
                name = match &expanded_one_of.schema_kind {
                    // Enums with no fields, e.g. MyEnum::SimpleVariant
                    SchemaKind::Type(openapiv3::Type::String(s)) => {
                        if let Some(Some(variant_name)) = s.enumeration.first() {
                            format!("{original_name}_{variant_name}")
                        } else {
                            log::warn!("Weird string oneof with no enum for the name: {s:?}");
                            continue;
                        }
                    }
                    // Enums with named fields, e.g. MyEnum::Variant{field: String}
                    // In this case,
                    // Enum variants should be named after their nested object.
                    // E.g. instead of ModelingCmd::ModelingCmd, it should be
                    // ModelingCmd::ModelingCmdCameraDragStart.
                    SchemaKind::Type(openapiv3::Type::Object(o)) => {
                        if let Some(prop_name) = o.properties.first().map(|(k, _v)| k.to_owned()) {
                            format!("{original_name}_{prop_name}")
                        } else {
                            log::warn!("Weird object oneof with no enum for the name: {o:?}");
                            continue;
                        }
                    }
                    SchemaKind::Type(openapiv3::Type::Array(a)) => match &a.items {
                        Some(openapiv3::ReferenceOr::Reference { .. }) => {
                            get_type_name_from_reference(
                                &a.items.clone().unwrap().reference()?,
                                &self.spec,
                                true,
                            )?
                            .rendered()?
                        }
                        Some(openapiv3::ReferenceOr::Item(s)) => {
                            get_type_name_for_schema(original_name, s, &self.spec, true)?
                                .rendered()?
                        }
                        None => {
                            log::warn!("Weird array oneof with no item for the name: {a:?}");
                            continue;
                        }
                    },
                    other => {
                        log::warn!("Weird oneof whose type isn't handled: {other:?}");
                        continue;
                    }
                };
                if let Some(title) = &expanded_one_of.schema_data.title {
                    name = title.to_string();
                }
                self.render_schema(&name, &one_of.item()?.clone())?;
            }

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
                let rendered_type =
                    get_type_name_for_schema(&name, &expanded_one_of, &self.spec, true)?;

                let n = if let Some(title) = &expanded_one_of.schema_data.title {
                    let p = proper_name(title);
                    p.parse().map_err(|e| anyhow::anyhow!("{}", e))?
                } else {
                    let t = inflector::cases::classcase::to_class_case(
                        &rendered_type.strip_option()?.strip_vec()?.rendered()?,
                    );
                    let t = format_ident!("{}", t);
                    quote!(#t)
                };

                let rendered = n.rendered()?;

                values.insert(rendered, one_of.clone());

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
}

/// Return the type name for a schema.
pub fn get_type_name_for_schema(
    name: &str,
    schema: &openapiv3::Schema,
    spec: &openapiv3::OpenAPI,
    in_crate: bool,
) -> Result<proc_macro2::TokenStream> {
    let t = match &schema.schema_kind {
        SchemaKind::Type(openapiv3::Type::String(s)) => {
            get_type_name_for_string(name, s, &schema.schema_data, in_crate)?
        }
        SchemaKind::Type(openapiv3::Type::Number(n)) => get_type_name_for_number(n)?,
        SchemaKind::Type(openapiv3::Type::Integer(i)) => get_type_name_for_integer(i)?,
        SchemaKind::Type(openapiv3::Type::Object(o)) => {
            get_type_name_for_object(name, o, &schema.schema_data, spec, in_crate)?
        }
        SchemaKind::Type(openapiv3::Type::Array(a)) => {
            get_type_name_for_array(name, a, spec, in_crate)?
        }
        SchemaKind::Type(openapiv3::Type::Boolean { .. }) => quote!(bool),
        SchemaKind::OneOf { one_of } => {
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
        SchemaKind::AllOf { all_of } => {
            get_type_name_for_all_of(name, all_of, &schema.schema_data, spec, in_crate)?
        }
        SchemaKind::AnyOf { any_of: _ } => get_type_name_for_object(
            name,
            &openapiv3::ObjectType::default(),
            &schema.schema_data,
            spec,
            in_crate,
        )?,
        SchemaKind::Not { not: _ } => {
            anyhow::bail!("XXX not not supported yet");
        }
        SchemaKind::Any(any) => {
            if let Some(s) = get_schema_from_any(&schema.schema_data, any) {
                return get_type_name_for_schema(name, &s, spec, in_crate);
            }
            log::warn!("got any schema kind `{}`: {:?}", name, any);
            quote!(serde_json::Value)
        }
    };

    if schema.schema_data.nullable && !t.is_option()? {
        Ok(quote!(Option<#t>))
    } else {
        Ok(t)
    }
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
            "ip" => quote!(std::net::IpAddr),
            "uri" => quote!(String),
            "uri-template" => quote!(String),
            "url" => quote!(String),
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
            "money-usd" => quote!(bigdecimal::BigDecimal),
            "id" => quote!(String),
            f => {
                anyhow::bail!("XXX unknown string format {} for {}", f, name)
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
    all_ofs: &[openapiv3::ReferenceOr<openapiv3::Schema>],
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

fn is_default_property(
    type_name: &proc_macro2::TokenStream,
    data: &openapiv3::SchemaData,
) -> Result<bool> {
    Ok(data.default.is_some()
        && (type_name.rendered()? == "bool" || type_name.rendered()?.starts_with("Vec<")))
}

/// A holder for our tag and content for enums.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
        if let SchemaKind::Type(openapiv3::Type::Object(o)) = schema.schema_kind {
            // If the object contains a property that is an enum of 1, then that is the tag.
            for (k, v) in &o.properties {
                // Get the schema for the property.
                let inner_schema = if let openapiv3::ReferenceOr::Item(i) = v {
                    let s = &**i;
                    s.clone()
                } else {
                    v.get_schema_from_reference(spec, true)?
                };

                if let SchemaKind::Type(openapiv3::Type::String(s)) = inner_schema.schema_kind {
                    if s.enumeration.len() == 1
                        && (result.tag.is_none() || result.tag == Some(k.to_string()))
                    {
                        result.tag = Some(k.to_string());
                    } else if result.tag == Some(k.to_string()) {
                        // The enum must be of length 1 for this to work.
                        // We thought it was but it isn't.
                        result.tag = None;
                        // We can't do anything with this.
                        return Ok(result);
                    }
                }
            }

            if result.tag.is_none() {
                // We couldn't find a tag.
                // We can't do anything with this.
                return Ok(result);
            }
        }
    }

    let mut has_content = false;
    if let Some(tag) = &result.tag {
        // Check if we also have content.
        // This would be true if the objects only have 2 properties, one of which is the tag and the other is the content.
        for one_of in one_ofs {
            // Get the schema for this OneOf.
            let schema = one_of.get_schema_from_reference(spec, true)?;
            // Determine if we can do anything fancy with the resulting enum and flatten it.
            if let SchemaKind::Type(openapiv3::Type::Object(o)) = schema.schema_kind {
                if o.properties.len() == 2 {
                    for (k, _) in &o.properties {
                        if tag != k {
                            // Make sure they all equal each other.
                            if has_content {
                                if Some(k.to_string()) != result.content {
                                    result.content = None;
                                    // Return early since we have a mismatch.
                                    return Ok(result);
                                }
                            } else {
                                has_content = true;
                                // This is the content.
                                result.content = Some(k.to_string());
                            }
                        }
                    }
                } else {
                    result.content = None;
                    // Return early since we have a mismatch.
                    return Ok(result);
                }
            } else {
                result.content = None;
                // Return early since we have a mismatch.
                return Ok(result);
            }
        }
    }

    Ok(result)
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
    } else if prop == "_links" {
        // Account for any weird types.
        // For the front API this makes sure there is not another "links" type in the object.
        prop = "underscore_links".to_string()
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
        || prop == "in"
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

    // Fixes for MailChimp, probably a better way to do this.
    // They have enums like:
    // 18-24
    // 55+
    let s = if s == "18-24" {
        "EighteenToTwentyFour".to_string()
    } else if s == "25-34" {
        "TwentyFiveToThirtyFour".to_string()
    } else if s == "35-44" {
        "ThirtyFiveToFourtyFour".to_string()
    } else if s == "45-54" {
        "FourtyFiveToFiftyFour".to_string()
    } else if s == "35-54" {
        "ThirtyFiveToFiftyFour".to_string()
    } else if s == "55-64" {
        "FiftyFiveToSixtyFour".to_string()
    } else if s == "55+" {
        "FiftyFivePlus".to_string()
    } else if s == "65+" {
        "SixtyFivePlus".to_string()
    } else if s == "-" {
        "Dash".to_string()
    } else {
        s
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
            s
        }
    } else {
        s
    };

    inflector::cases::pascalcase::to_pascal_case(&s)
        .trim_start_matches("CrateTypes")
        .trim_start_matches("VecCrateTypes")
        .trim_start_matches("OptionCrateTypes")
        .replace("V1", "")
}

/// Return the name for a type based on a name if passed or the title of the schema data.
fn get_type_name(name: &str, data: &openapiv3::SchemaData) -> Result<proc_macro2::Ident> {
    let t = if !name.is_empty() {
        proper_name(name)
    } else if let Some(title) = &data.title {
        proper_name(title)
    } else {
        anyhow::bail!("Cannot get type name without name or title: {:?}", data);
    };

    Ok(format_ident!("{t}"))
}

fn clean_text(s: &str) -> String {
    // Add newlines after end-braces at <= two levels of indentation.
    if cfg!(not(windows)) {
        let regex = regex::Regex::new(r"(})(\n\s{0,8}[^} ])").unwrap();
        regex.replace_all(s, "$1\n$2").to_string()
    } else {
        let regex = regex::Regex::new(r"(})(\r\n\s{0,8}[^} ])").unwrap();
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

fn get_multipart_mod() -> Result<proc_macro2::TokenStream> {
    let file = include_str!("multipart.rs");
    let stream = proc_macro2::TokenStream::from_str(file).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(quote!(
        pub mod multipart {
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
            if let SchemaKind::Type(openapiv3::Type::Object(o)) = &schema.schema_kind {
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

        anyhow::bail!("No item type found, {:?}", self)
    }

    /// Get the item ident for this object.
    pub fn item_ident(&self) -> Result<proc_macro2::Ident> {
        if let Some((k, _v)) = &self.items {
            return Ok(format_ident!("{}", k));
        }

        anyhow::bail!("No item type found, {:?}", self)
    }

    /// Get the item type for this object.
    pub fn next_page_str(&self) -> Result<String> {
        if let Some((k, _v)) = &self.next_page {
            return Ok(k.to_string());
        }

        anyhow::bail!("No next page property found, {:?}", self)
    }

    /// Get the item type for this object.
    pub fn page_param_str(&self) -> Result<String> {
        if let Some((k, _v)) = &self.page_param {
            return Ok(k.to_string());
        }

        anyhow::bail!("No page param property found: {:?}", self)
    }
}

fn is_pagination_property_next_page(s: &str) -> bool {
    ["next_page", "next", "next_link"].contains(&s)
}

fn is_pagination_property_param_page(s: &str) -> bool {
    ["page_token", "page", "cursor"].contains(&s)
}

fn is_pagination_property_items(s: &str, t: &proc_macro2::TokenStream) -> Result<bool> {
    Ok(["items", "data", "results"].contains(&s) && get_text(t)?.starts_with("Vec<"))
}

pub(crate) fn get_schema_from_any(data: &SchemaData, any: &AnySchema) -> Option<Schema> {
    if any.additional_properties.is_some() || !any.properties.is_empty() {
        // Let's assume this is an object.

        // Send back through as an object.
        return Some(openapiv3::Schema {
            schema_data: data.clone(),
            schema_kind: SchemaKind::Type(openapiv3::Type::Object(openapiv3::ObjectType {
                properties: any.properties.clone(),
                required: any.required.clone(),
                additional_properties: any.additional_properties.clone(),
                min_properties: any.min_properties,
                max_properties: any.max_properties,
            })),
        });
    } else if !any.one_of.is_empty() {
        return Some(openapiv3::Schema {
            schema_data: data.clone(),
            schema_kind: SchemaKind::OneOf {
                one_of: any.one_of.clone(),
            },
        });
    } else if !any.all_of.is_empty() {
        return Some(openapiv3::Schema {
            schema_data: data.clone(),
            schema_kind: SchemaKind::AllOf {
                all_of: any.all_of.clone(),
            },
        });
    } else if let Some(typ) = &any.typ {
        if typ == "object" {
            if let Some(format) = &any.format {
                if format == "uri-map" {
                    return Some(openapiv3::Schema {
                        schema_data: data.clone(),
                        schema_kind: SchemaKind::Type(openapiv3::Type::Object(
                            openapiv3::ObjectType {
                                properties: any.properties.clone(),
                                required: any.required.clone(),
                                additional_properties: Some(openapiv3::AdditionalProperties::Any(
                                    true,
                                )),
                                min_properties: any.min_properties,
                                max_properties: any.max_properties,
                            },
                        )),
                    });
                }
            }
        } else if typ == "array" {
            return Some(openapiv3::Schema {
                schema_data: data.clone(),
                schema_kind: SchemaKind::Type(openapiv3::Type::Array(openapiv3::ArrayType {
                    items: any.items.clone(),
                    min_items: any.min_items,
                    max_items: any.max_items,
                    unique_items: any.unique_items.unwrap_or(false),
                })),
            });
        }

        return Some(openapiv3::Schema {
            schema_data: data.clone(),
            schema_kind: SchemaKind::Type(openapiv3::Type::String(openapiv3::StringType {
                format: openapiv3::VariantOrUnknownOrEmpty::Unknown(typ.to_string()),
                ..Default::default()
            })),
        });
    }

    None
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_generate_kittycad_types() {
        let result = super::generate_types(
            &crate::load_json_spec(include_str!("../../../spec.json")).unwrap(),
            Default::default(),
        )
        .unwrap();
        expectorate::assert_contents("tests/types/kittycad.rs.gen", &result.render().unwrap());
    }

    #[test]
    #[ignore] // Some sort of circular loop.
    fn test_generate_github_types() {
        let result = super::generate_types(
            &crate::load_json_spec(include_str!("../../tests/api.github.com.json")).unwrap(),
            Default::default(),
        )
        .unwrap();
        expectorate::assert_contents("tests/types/github.rs.gen", &result.render().unwrap());
    }

    #[test]
    fn test_generate_oxide_types() {
        let result = super::generate_types(
            &crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
            Default::default(),
        )
        .unwrap();
        expectorate::assert_contents("tests/types/oxide.rs.gen", &result.render().unwrap());
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
        let schema = include_str!("../../tests/types/input/RouterRoute.json");

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
            rendered: quote!(),
            opts: Default::default(),
        };

        type_space.render_schema("RouterRoute", &schema).unwrap();

        expectorate::assert_contents(
            "tests/types/oxide.router-route.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
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

        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
            rendered: quote!(),
            opts: Default::default(),
        };

        type_space.render_schema("IpNet", &schema).unwrap();

        expectorate::assert_contents(
            "tests/types/oxide.ip-net.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
        );
    }

    #[test]
    fn test_schema_parsing_one_of_with_tag_content() {
        let schema = include_str!("../../tests/types/input/VpcFirewallRuleTarget.json");

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
            rendered: quote!(),
            opts: Default::default(),
        };

        type_space
            .render_schema("VpcFirewallRuleTarget", &schema)
            .unwrap();

        expectorate::assert_contents(
            "tests/types/oxide.vpc-filewall-rule-target.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
        );
    }

    #[test]
    fn test_schema_parsing_one_of_with_tag_no_content() {
        let schema = include_str!("../../tests/types/input/AsyncApiCallOutput.json");

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../../spec.json")).unwrap(),
            rendered: quote!(),
            opts: Default::default(),
        };

        type_space
            .render_schema("AsyncApiCallOutput", &schema)
            .unwrap();

        expectorate::assert_contents(
            "tests/types/kittycad.async-api-call-output.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
        );
    }

    #[test]
    fn test_schema_parsing_one_of_enum_needs_gen() {
        let schema = r#"{
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "enum": [
                  "sha256"
                ]
              },
              "value": {
                "type": "string"
              }
            },
            "required": [
              "type",
              "value"
            ]
          }
        ]
        }"#;

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../tests/oxide.json")).unwrap(),
            rendered: quote!(),
            opts: Default::default(),
        };

        type_space.render_schema("Digest", &schema).unwrap();

        expectorate::assert_contents(
            "tests/types/oxide.digest.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
        );
    }

    #[test]
    fn test_websocket() {
        let schema = include_str!("../../tests/types/input/websocket.json");
        let spec: openapiv3::OpenAPI = serde_json::from_str(schema).unwrap();
        let mut type_space = super::generate_types(&spec, Default::default()).unwrap();

        let files = crate::functions::generate_files(&mut type_space, &Default::default())
            .unwrap()
            .0;

        // The Rust source code for the websocket endpoint.
        let source_code = files
            .iter()
            .find(|(k, _)| k == &"default")
            .unwrap()
            .1
            .to_string();

        assert!(source_code.contains("pub async fn example_api_websocket_counter"));
        expectorate::assert_contents(
            "tests/types/websocket.rs.gen",
            &rustfmt_wrapper::rustfmt(&source_code).unwrap(),
        );
    }

    #[test]
    fn test_render_object_with_custom_date_format() {
        let schema = include_str!("../../tests/types/input/FileDensity.json");

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let opts = crate::Opts {
            date_time_format: Some("%Y-%m-%dT%H:%M:%S".to_string()),
            ..Default::default()
        };
        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../../spec.json")).unwrap(),
            rendered: quote!(),
            opts,
        };

        type_space.render_schema("FileDensity", &schema).unwrap();

        expectorate::assert_contents(
            "tests/types/kittycad.file-density-date-time-override-output.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
        );
    }

    #[test]
    fn test_render_one_of_enum_types() {
        let schema = include_str!("../../tests/types/input/AccountProvider.json");

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../../spec.json")).unwrap(),
            rendered: quote!(),
            opts: Default::default(),
        };

        type_space.render_schema("AccountProvier", &schema).unwrap();

        expectorate::assert_contents(
            "tests/types/kittycad.account-provider-output.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
        );
    }

    #[test]
    fn test_render_sum_enum_types() {
        let schema = include_str!("../../tests/types/input/ModelingCmd.json");

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../../spec.json")).unwrap(),
            rendered: quote!(),
            opts: Default::default(),
        };

        type_space.render_schema("ModelingCmd", &schema).unwrap();

        expectorate::assert_contents(
            "tests/types/kittycad.drawing-cmd-output.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
        );
    }

    #[test]
    fn test_render_sum_enum_some_one_some_more() {
        let schema = include_str!("../../tests/types/input/SubscriptionTierType.json");

        let schema = serde_json::from_str::<openapiv3::Schema>(schema).unwrap();

        let mut type_space = super::TypeSpace {
            types: indexmap::map::IndexMap::new(),
            spec: crate::load_json_spec(include_str!("../../../spec.json")).unwrap(),
            rendered: quote!(),
            opts: Default::default(),
        };

        type_space
            .render_schema("SubscriptionTierType", &schema)
            .unwrap();

        expectorate::assert_contents(
            "tests/types/kittycad.subscription-tier-type.rs.gen",
            &super::get_text_fmt(&type_space.rendered).unwrap(),
        );
    }
}
