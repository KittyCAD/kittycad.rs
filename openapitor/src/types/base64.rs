//! Base64 data that encodes to url safe base64, but can decode from multiple
//! base64 implementations to account for various clients and libraries. Compatible
//! with serde and JsonSchema.

use std::{convert::TryFrom, fmt};

use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

static ALLOWED_DECODING_FORMATS: &[data_encoding::Encoding] = &[
    data_encoding::BASE64,
    data_encoding::BASE64URL,
    data_encoding::BASE64URL_NOPAD,
    data_encoding::BASE64_MIME,
    data_encoding::BASE64_NOPAD,
];

#[derive(Debug, Clone, PartialEq, Eq)]
/// A container for binary that should be base64 encoded in serialisation. In reverse
/// when deserializing, will decode from many different types of base64 possible.
pub struct Base64Data(pub Vec<u8>);

impl Base64Data {
    /// Return is the data is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for Base64Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", data_encoding::BASE64URL_NOPAD.encode(&self.0))
    }
}

impl From<Base64Data> for Vec<u8> {
    fn from(data: Base64Data) -> Vec<u8> {
        data.0
    }
}

impl From<Vec<u8>> for Base64Data {
    fn from(data: Vec<u8>) -> Base64Data {
        Base64Data(data)
    }
}

impl AsRef<[u8]> for Base64Data {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&str> for Base64Data {
    type Error = anyhow::Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        for config in ALLOWED_DECODING_FORMATS {
            if let Ok(data) = config.decode(v.as_bytes()) {
                return Ok(Base64Data(data));
            }
        }

        anyhow::bail!("Could not decode base64 data: {}", v);
    }
}

struct Base64DataVisitor;

impl Visitor<'_> for Base64DataVisitor {
    type Value = Base64Data;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a base64 encoded string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        // Forgive alt base64 decoding formats
        for config in ALLOWED_DECODING_FORMATS {
            if let Ok(data) = config.decode(v.as_bytes()) {
                return Ok(Base64Data(data));
            }
        }

        Err(serde::de::Error::invalid_value(Unexpected::Str(v), &self))
    }
}

impl<'de> Deserialize<'de> for Base64Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Base64DataVisitor)
    }
}

impl Serialize for Base64Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = data_encoding::BASE64URL_NOPAD.encode(&self.0);
        serializer.serialize_str(&encoded)
    }
}

impl schemars::JsonSchema for Base64Data {
    fn schema_name() -> String {
        "Base64Data".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut obj = gen.root_schema_for::<String>().schema;
        // From: https://swagger.io/specification/#data-types
        obj.format = Some("byte".to_string());
        schemars::schema::Schema::Object(obj)
    }

    fn is_referenceable() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Base64Data;

    #[test]
    fn test_base64_try_from() {
        assert!(Base64Data::try_from("aGVsbG8=").is_ok());
        assert!(Base64Data::try_from("abcdefghij").is_err());
    }
}
