use anyhow::Result;

trait SchemaExt {
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema>;
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
}

impl SchemaExt for Box<openapiv3::Schema> {
    fn recurse(&self, _spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        anyhow::bail!("`recurse` not implemented for `Box<openapiv3::Schema>`")
    }
}

impl SchemaExt for openapiv3::PathItem {
    fn recurse(&self, _spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        anyhow::bail!("`recurse` not implemented for `PathItem`")
    }
}

impl SchemaExt for openapiv3::RequestBody {
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        // Get the content type.
        let content = self.content.get("application/json").ok_or_else(|| {
            anyhow::anyhow!("RequestBody does not have a content type of `application/json`")
        })?;

        if content.schema.is_none() {
            anyhow::bail!("RequestBody does not have a schema")
        }

        let schema = content.schema.as_ref().unwrap();

        // Recurse the schema.
        schema.recurse(spec)
    }
}

impl SchemaExt for openapiv3::Parameter {
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema> {
        // Get the parameter data.
        let data = self
            .data()
            .ok_or_else(|| anyhow::anyhow!("Parameter does not have data"))?;
        // Get the parameter schema.
        let schema = data.format.schema()?;
        // Recurse the schema.
        schema.recurse(spec)
    }
}

pub trait ReferenceOrExt<T> {
    fn item(&self) -> Result<&T>;
    fn recurse(&self, spec: &openapiv3::OpenAPI) -> Result<openapiv3::Schema>;
    fn reference(&self) -> Result<String>;
    fn get_schema_from_reference(
        &self,
        spec: &openapiv3::OpenAPI,
        recursive: bool,
    ) -> Result<openapiv3::Schema>;
}

impl<T: SchemaExt> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
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
}

pub trait ParameterSchemaOrContentExt {
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

pub trait ParameterExt {
    fn data(&self) -> Option<openapiv3::ParameterData>;
}

impl ParameterExt for &openapiv3::Parameter {
    fn data(&self) -> Option<openapiv3::ParameterData> {
        match self {
            openapiv3::Parameter::Path {
                parameter_data,
                style: _,
            } => Some(parameter_data.clone()),
            openapiv3::Parameter::Header {
                parameter_data,
                style: openapiv3::HeaderStyle::Simple,
            } => Some(parameter_data.clone()),
            openapiv3::Parameter::Cookie {
                parameter_data,
                style: openapiv3::CookieStyle::Form,
            } => Some(parameter_data.clone()),
            openapiv3::Parameter::Query {
                parameter_data,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => Some(parameter_data.clone()),
        }
    }
}
