//! Extension traits for OpenAPI types that are nice to have.

use anyhow::Result;

/// A trait for types that house a `Schema`.
pub trait SchemaExt {
    /// Returns the schema for the type.
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema>;

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
                    openapiv3::ReferenceOr::Reference { reference: _ } => {
                        first.get_schema_from_reference(spec, true)?
                    }
                };

                return Ok(r);
            }
        }

        Ok(self.clone())
    }

    fn get_reference(
        name: &str,
        spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        if let Some(components) = &spec.components {
            if let Some(schema) = components
                .schemas
                .get(name.trim_start_matches("#/components/schemas/"))
            {
                return Ok(schema.clone());
            }
        }

        anyhow::bail!("schema does not exist: {}", name)
    }
}

impl SchemaExt for Box<openapiv3::Schema> {
    fn recurse(&self, _spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        anyhow::bail!("`recurse` not implemented for `Box<openapiv3::Schema>`")
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

    fn get_reference(
        name: &str,
        spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        if let Some(components) = &spec.components {
            if let Some(request_body) = components
                .request_bodies
                .get(name.trim_start_matches("#/components/request_bodies/"))
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

    fn get_reference(
        name: &str,
        spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        if let Some(components) = &spec.components {
            if let Some(parameter) = components
                .parameters
                .get(name.trim_start_matches("#/components/parameters/"))
            {
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

    fn get_reference(
        name: &str,
        spec: &openapiv3::OpenAPI,
    ) -> Result<openapiv3::ReferenceOr<Self>> {
        if let Some(components) = &spec.components {
            if let Some(response) = components
                .responses
                .get(name.trim_start_matches("#/components/responses/"))
            {
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

impl<T: SchemaExt + Clone> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
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
            // TODO: use the function on the reference shit above, so this works for all types.
            openapiv3::ReferenceOr::Reference { reference } => Ok(reference
                .trim_start_matches("#/components/schemas/")
                .to_string()),
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
            match self.recurse(spec) {
                Ok(s) => Ok(s),
                Err(_) => self.get_schema_from_reference(spec, recursive),
            }
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
