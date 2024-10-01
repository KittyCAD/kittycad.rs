//! Extension traits for OpenAPI types that are nice to have.

use anyhow::Result;

/// A trait for types that house a `Schema`.
pub trait SchemaExt {
    /// Returns the schema for the type.
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema>;

    /// Returns a cleaned reference name for the type.
    fn clean_reference_name(s: &str) -> String;

    /// Returns the type for the reference.
    fn get_reference(name: &str, spec: &openapiv3::OpenAPI) -> Result<openapiv3::ReferenceOr<Self>>
    where
        Self: Sized;
}

impl SchemaExt for openapiv3::Schema {
    // If there is an allOf with only one item, we can just return that.
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        if let openapiv3::SchemaKind::AllOf { all_of } = &self.schema_kind {
            if all_of.len() == 1 {
                let first = all_of[0].clone();

                let r = match first {
                    openapiv3::ReferenceOr::Item(i) => i,
                    openapiv3::ReferenceOr::Reference { reference } => {
                        Self::get_reference(&reference, spec)?.expand(spec)?
                    }
                };

                return Ok(r);
            }
        }

        Ok(self.clone())
    }

    fn clean_reference_name(s: &str) -> String {
        s.trim_start_matches("#/components/schemas/").to_string()
    }

    fn get_reference(
        name: &str,
        spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        if let Some(components) = &spec.components {
            if let Some(schema) = components.schemas.get(&Self::clean_reference_name(name)) {
                return Ok(schema.clone());
            }
        }

        anyhow::bail!("schema does not exist: {}", name)
    }
}

impl SchemaExt for Box<openapiv3::Schema> {
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        if let openapiv3::SchemaKind::AllOf { all_of } = &self.schema_kind {
            if all_of.len() == 1 {
                let first = all_of[0].clone();

                let r = match first {
                    openapiv3::ReferenceOr::Item(i) => i,
                    openapiv3::ReferenceOr::Reference { reference } => {
                        openapiv3::Schema::get_reference(&reference, spec)?.expand(spec)?
                    }
                };

                return Ok(r);
            }
        }

        let s = &**self;
        Ok(s.clone())
    }

    fn clean_reference_name(s: &str) -> String {
        s.trim_start_matches("#/components/schemas/").to_string()
    }

    fn get_reference(
        _name: &str,
        _spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        anyhow::bail!("`get_reference` not implemented for `Box<openapiv3::Schema>`")
    }
}

impl SchemaExt for openapiv3::PathItem {
    fn recurse(&self, _spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        anyhow::bail!("`recurse` not implemented for `PathItem`")
    }

    fn clean_reference_name(_s: &str) -> String {
        "`clean_reference_name` not implemented for `PathItem`".to_string()
    }

    fn get_reference(
        _name: &str,
        _spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        anyhow::bail!("`get_reference` not implemented for `PathItem`")
    }
}

impl SchemaExt for openapiv3::RequestBody {
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        // Iterate over all the media types and return the first request.
        for (_name, content) in &self.content {
            if let Some(s) = &content.schema {
                return s.recurse(spec);
            }
        }

        anyhow::bail!("RequestBody does not have a schema: {:?}", self)
    }

    fn clean_reference_name(s: &str) -> String {
        s.trim_start_matches("#/components/requestBodies/")
            .to_string()
    }

    fn get_reference(
        name: &str,
        spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        if let Some(components) = &spec.components {
            if let Some(request_body) = components
                .request_bodies
                .get(&Self::clean_reference_name(name))
            {
                return Ok(request_body.clone());
            }
        }

        anyhow::bail!("request body does not exist: {}", name)
    }
}

impl SchemaExt for openapiv3::Parameter {
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        // Get the parameter data.
        let data = self.data()?;
        // Get the parameter schema.
        let schema = data.format.schema()?;
        // Recurse the schema.
        schema.recurse(spec)
    }

    fn clean_reference_name(s: &str) -> String {
        s.trim_start_matches("#/components/parameters/").to_string()
    }

    fn get_reference(
        name: &str,
        spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        if let Some(components) = &spec.components {
            if let Some(parameter) = components.parameters.get(&Self::clean_reference_name(name)) {
                return Ok(parameter.clone());
            }
        }

        anyhow::bail!("parameter does not exist: {}", name)
    }
}

impl SchemaExt for openapiv3::Response {
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        // Iterate over all the media types and return the first response.
        for (_name, content) in &self.content {
            if let Some(s) = &content.schema {
                return s.recurse(spec);
            }
        }

        anyhow::bail!("Response does not have a schema: {:?}", self)
    }

    fn clean_reference_name(s: &str) -> String {
        s.trim_start_matches("#/components/responses/").to_string()
    }

    fn get_reference(
        name: &str,
        spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        if let Some(components) = &spec.components {
            if let Some(response) = components.responses.get(&Self::clean_reference_name(name)) {
                return Ok(response.clone());
            }
        }

        anyhow::bail!("response does not exist: {}", name)
    }
}

/// A trait for types that have a `Schema`.
pub trait ReferenceOrExt<T> {
    /// Get the item for the ReferenceOr.
    /// This returns an error if the ReferenceOr is a Reference.
    fn item(&self) -> Result<&T>;

    /// Recurse the schemas.
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema>;

    /// Get the reference for the ReferenceOr.
    /// This returns an error if the ReferenceOr is an item.
    fn reference(&self) -> Result<String>;

    /// Get the schema from the ReferenceOr.
    /// This will recurse any references and get the underlying schemas for those.
    fn get_schema_from_reference(
        &self,
        spec: &openapiv3::OpenAPI,
        recursive: bool,
    ) -> Result<openapiv3::Schema>;

    /// Get the type from a ReferenceOr.
    fn expand(&self, spec: &openapiv3::OpenAPI) -> Result<T>;
}

impl<T: SchemaExt + Clone + std::fmt::Debug> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item(&self) -> Result<&T> {
        match self {
            openapiv3::ReferenceOr::Item(i) => Ok(i),
            openapiv3::ReferenceOr::Reference { reference } => {
                anyhow::bail!("reference not supported here: {}", reference);
            }
        }
    }

    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        match self {
            openapiv3::ReferenceOr::Item(i) => Ok(i.recurse(spec)?),
            openapiv3::ReferenceOr::Reference { reference } => {
                anyhow::bail!("reference not supported here: {}", reference);
            }
        }
    }

    fn reference(&self) -> Result<String> {
        match self {
            openapiv3::ReferenceOr::Item(..) => {
                anyhow::bail!("item not supported here");
            }
            openapiv3::ReferenceOr::Reference { reference } => {
                Ok(T::clean_reference_name(reference))
            }
        }
    }

    fn get_schema_from_reference(
        &self,
        spec: &openapiv3::OpenAPI,
        recursive: bool,
    ) -> Result<openapiv3::Schema> {
        if let Ok(name) = self.reference() {
            let components = spec
                .components
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("components not found in spec"))?;

            let schema = components
                .schemas
                .get(&name)
                .ok_or_else(|| anyhow::anyhow!("could not find schema with name {}", name))?;

            match schema.item() {
                Ok(s) => Ok(s.clone()),
                Err(_) => schema.get_schema_from_reference(spec, recursive),
            }
        } else if !recursive {
            anyhow::bail!("item not supported here");
        } else {
            self.recurse(spec)
        }
    }

    fn expand(&self, spec: &openapiv3::OpenAPI) -> Result<T> {
        match self {
            openapiv3::ReferenceOr::Item(i) => Ok(i.clone()),
            openapiv3::ReferenceOr::Reference { reference } => {
                let ref_or = T::get_reference(reference, spec)?;
                ref_or.expand(spec)
            }
        }
    }
}

/// A trait for types that can be converted to a `ReferenceOr<Schema>`.
pub trait ParameterSchemaOrContentExt {
    /// Return the `ReferenceOr<Schema>` for the type.
    fn schema(&self) -> Result<openapiv3::ReferenceOr<openapiv3::Schema>>;
}

impl ParameterSchemaOrContentExt for openapiv3::ParameterSchemaOrContent {
    fn schema(&self) -> Result<openapiv3::ReferenceOr<openapiv3::Schema>> {
        match self {
            openapiv3::ParameterSchemaOrContent::Schema(s) => Ok(s.clone()),
            openapiv3::ParameterSchemaOrContent::Content(..) => {
                anyhow::bail!("content not supported here");
            }
        }
    }
}

/// A trait for types that have `ParameterData`.
pub trait ParameterExt {
    /// Return the `ParameterData` for the type.
    fn data(&self) -> Result<openapiv3::ParameterData>;
}

impl ParameterExt for &openapiv3::Parameter {
    fn data(&self) -> Result<openapiv3::ParameterData> {
        match self {
            openapiv3::Parameter::Path {
                parameter_data,
                style: _,
            } => Ok(parameter_data.clone()),
            openapiv3::Parameter::Header {
                parameter_data,
                style: openapiv3::HeaderStyle::Simple,
            } => Ok(parameter_data.clone()),
            openapiv3::Parameter::Cookie {
                parameter_data,
                style: openapiv3::CookieStyle::Form,
            } => Ok(parameter_data.clone()),
            openapiv3::Parameter::Query {
                parameter_data,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => Ok(parameter_data.clone()),
        }
    }
}

/// A trait for utility functions on status codes.
pub trait StatusCodeExt {
    /// Returns `true` if the HTTP status code was successful.
    fn is_success(&self) -> bool;
}

impl StatusCodeExt for openapiv3::StatusCode {
    fn is_success(&self) -> bool {
        match self {
            openapiv3::StatusCode::Code(c) => (&200..&300).contains(&c),
            openapiv3::StatusCode::Range(r) => r.to_string().starts_with('2'),
        }
    }
}

/// A trait for utility functions on token streams.
pub trait TokenStreamExt {
    /// Return `true` if the token stream is a Vec.
    fn is_vec(&self) -> Result<bool>;

    /// Return `true` if the token stream is an Option wrapping a Vec.
    fn is_option_vec(&self) -> Result<bool>;

    /// Return `true` if the token stream is already an option.
    fn is_option(&self) -> Result<bool>;

    /// Remove the Option<> from the type.
    fn strip_option(&self) -> Result<proc_macro2::TokenStream>;

    /// Remove the Vec<> from the type.
    fn strip_vec(&self) -> Result<proc_macro2::TokenStream>;

    /// Return `true` if the token stream is a string.
    fn is_string(&self) -> Result<bool>;

    /// Strip the `.to_string()` from the token such that it becomes a `&str`.
    fn strip_to_string(&self) -> Result<proc_macro2::TokenStream>;

    /// Render the token stream as a string.
    fn rendered(&self) -> Result<String>;

    /// Render and `rustfmt` the token stream as a string.
    fn rendered_fmt(&self) -> Result<String>;

    /// Get the value of an argument from it's type.
    fn get_parameter_value(&self) -> Result<proc_macro2::TokenStream>;
}

impl TokenStreamExt for proc_macro2::TokenStream {
    fn is_vec(&self) -> Result<bool> {
        let rendered = self.rendered()?;
        Ok(rendered.starts_with("Vec<"))
    }

    fn is_option_vec(&self) -> Result<bool> {
        let rendered = self.rendered()?;
        Ok(rendered.starts_with("Option<Vec<"))
    }

    fn is_option(&self) -> Result<bool> {
        let rendered = self.rendered()?;
        // The phone number type is also a nested option.
        Ok(rendered.starts_with("Option<") || rendered.ends_with("phone_number::PhoneNumber"))
    }

    fn strip_option(&self) -> Result<proc_macro2::TokenStream> {
        if !self.is_option()? {
            return Ok(self.clone());
        }

        let rendered = self.rendered()?;
        let rendered = rendered.trim_start_matches("Option<").trim_end_matches('>');
        rendered.parse().map_err(|e| anyhow::anyhow!("{}", e))
    }

    fn strip_vec(&self) -> Result<proc_macro2::TokenStream> {
        if !self.is_vec()? {
            return Ok(self.clone());
        }

        let rendered = self.rendered()?;
        let rendered = rendered.trim_start_matches("Vec<").trim_end_matches('>');
        rendered.parse().map_err(|e| anyhow::anyhow!("{}", e))
    }

    fn is_string(&self) -> Result<bool> {
        let rendered = self.rendered()?;
        Ok(rendered == "String"
            || rendered == "&str"
            || rendered == "&'astr"
            || rendered == "&'_str")
    }

    fn strip_to_string(&self) -> Result<proc_macro2::TokenStream> {
        let rendered = self.rendered()?;
        let rendered = rendered.trim_end_matches(".to_string()");
        rendered.parse().map_err(|e| anyhow::anyhow!("{}", e))
    }

    fn rendered(&self) -> Result<String> {
        crate::types::get_text(self)
    }

    fn rendered_fmt(&self) -> Result<String> {
        crate::types::get_text_fmt(self)
    }

    /// Get the value of an argument from it's type.
    fn get_parameter_value(&self) -> Result<proc_macro2::TokenStream> {
        let rendered = self.rendered()?;
        if rendered == "String" {
            return Ok(quote!(&'a str));
        }

        Ok(self.clone())
    }
}

/// A trait for types that have a `Schema`.
pub trait SchemaRenderExt {
    /// Returns if the schema is a type that should be rendered.
    /// For example, an enum that is not a reference, an object that is not a reference.
    fn should_render(&self) -> Result<bool>;
}

impl SchemaRenderExt for openapiv3::ReferenceOr<openapiv3::Schema> {
    fn should_render(&self) -> Result<bool> {
        match self {
            openapiv3::ReferenceOr::Item(i) => {
                let boxed = openapiv3::ReferenceOr::Item(Box::new(i.clone()));
                boxed.should_render()
            }
            openapiv3::ReferenceOr::Reference { reference: _ } => Ok(false),
        }
    }
}

impl SchemaRenderExt for openapiv3::ReferenceOr<Box<openapiv3::Schema>> {
    fn should_render(&self) -> Result<bool> {
        match self {
            openapiv3::ReferenceOr::Item(i) => {
                // Check if the type is an enum.
                match &i.schema_kind {
                    openapiv3::SchemaKind::Type(openapiv3::Type::String(s)) => {
                        if !s.enumeration.is_empty() {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    }
                    openapiv3::SchemaKind::Type(openapiv3::Type::Object(o)) => {
                        if o.properties.is_empty() {
                            Ok(false)
                        } else {
                            Ok(true)
                        }
                    }
                    // If it is an array, we might need to render the inner type.
                    openapiv3::SchemaKind::Type(openapiv3::Type::Array(_)) => Ok(true),
                    // If it is a OneOf, we might need to render the inner types.
                    openapiv3::SchemaKind::OneOf { one_of: _ } => Ok(true),
                    openapiv3::SchemaKind::Any(any) => {
                        if !any.properties.is_empty() || any.additional_properties.is_some() {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    }
                    _ => Ok(false),
                }
            }
            openapiv3::ReferenceOr::Reference { reference: _ } => Ok(false),
        }
    }
}

/// A trait for `Operation`s.
pub trait OperationExt {
    /// Returns what the function name should be for the operation.
    fn get_fn_name(&self) -> Result<String>;
    /// Get the function name as an ident.
    fn get_fn_name_ident(&self) -> Result<proc_macro2::Ident>;
    /// Get the first tag for the operation.
    fn get_tag(&self) -> Result<String>;
}

impl OperationExt for openapiv3::Operation {
    fn get_tag(&self) -> Result<String> {
        match self.tags.first() {
            Some(tag) => Ok(crate::clean_tag_name(tag)),
            None => Ok("default".to_string()),
        }
    }

    fn get_fn_name(&self) -> Result<String> {
        let tag = self.get_tag()?;

        let mut name = self
            .operation_id
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("operation has no operation_id: {:?}", self))?
            .to_string();

        // Convert to snake case.
        name = inflector::cases::snakecase::to_snake_case(&name);

        // Remove any stutters with the tag name.
        name = remove_stutters(&name, &tag);
        // Remove any stutters with the singular tag name.
        name = remove_stutters(&name, &singular(&tag))
            .replace("_v_1_", "_")
            .trim_end_matches("_v_1")
            .to_string();

        Ok(name)
    }

    fn get_fn_name_ident(&self) -> Result<proc_macro2::Ident> {
        let fn_name = self.get_fn_name()?;
        Ok(format_ident!("{}", fn_name))
    }
}

/// Remove any stutters with a string.
fn remove_stutters(whole: &str, s: &str) -> String {
    let mut whole = whole.to_string();
    if whole.starts_with(&format!("{}_", s)) {
        whole = whole.trim_start_matches(&format!("{}_", s)).to_string();
    }
    if whole.ends_with(&format!("_{}", s)) {
        whole = whole.trim_end_matches(&format!("_{}", s)).to_string();
    }
    if whole.contains(&format!("_{}_", s)) {
        whole = whole.replace(&format!("_{}_", s), "_");
    }

    whole
}

/// Return the singular version of a string (if it plural).
fn singular(s: &str) -> String {
    if let Some(b) = s.strip_suffix('s') {
        return b.to_string();
    }

    s.to_string()
}
