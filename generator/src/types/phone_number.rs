//! A library to implement phone numbers for JSON serialization and deserialization.

use std::str::FromStr;

use schemars::JsonSchema;

/// A phone number.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct PhoneNumber(pub phonenumber::PhoneNumber);

impl From<phonenumber::PhoneNumber> for PhoneNumber {
    fn from(phone: phonenumber::PhoneNumber) -> PhoneNumber {
        PhoneNumber(phone)
    }
}

impl AsRef<phonenumber::PhoneNumber> for PhoneNumber {
    fn as_ref(&self) -> &phonenumber::PhoneNumber {
        &self.0
    }
}

impl std::ops::Deref for PhoneNumber {
    type Target = phonenumber::PhoneNumber;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde::ser::Serialize for PhoneNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for PhoneNumber {
    fn deserialize<D>(deserializer: D) -> Result<PhoneNumber, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).unwrap_or_default();
        PhoneNumber::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl std::str::FromStr for PhoneNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(anyhow::anyhow!("phone number cannot be empty"));
        }

        let s = if !s.trim().starts_with('+') {
            format!("+1{}", s)
                .replace('-', "")
                .replace('(', "")
                .replace(')', "")
                .replace(' ', "")
        } else {
            s.replace('-', "")
                .replace('(', "")
                .replace(')', "")
                .replace(' ', "")
        };

        Ok(PhoneNumber(phonenumber::parse(None, &s).map_err(|e| {
            anyhow::anyhow!("invalid phone number `{}`: {}", s, e)
        })?))
    }
}

impl std::string::ToString for PhoneNumber {
    fn to_string(&self) -> String {
        self.format()
            .mode(phonenumber::Mode::International)
            .to_string()
    }
}

impl JsonSchema for PhoneNumber {
    fn schema_name() -> String {
        "PhoneNumber".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut obj = gen.root_schema_for::<String>().schema;
        obj.format = Some("phone".to_string());
        schemars::schema::Schema::Object(obj)
    }

    fn is_referenceable() -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::PhoneNumber;

    #[test]
    fn test_parse_phone_number() {
        let mut phone = "+1-555-555-5555";
        let mut phone_parsed: PhoneNumber =
            serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
        let mut expected = PhoneNumber(phonenumber::parse(None, phone).unwrap());
        assert_eq!(phone_parsed, expected);
        let mut expected_str = "+1 555-555-5555";
        assert_eq!(expected_str, serde_json::json!(phone_parsed));

        // Try with no country code.
        phone = "555-555-5555";
        phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
        assert_eq!(phone_parsed, expected);
        assert_eq!(expected_str, serde_json::json!(phone_parsed));

        // Try with space & country code.
        phone = "+1 555-555-5555";
        phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
        assert_eq!(phone_parsed, expected);
        assert_eq!(expected_str, serde_json::json!(phone_parsed));

        // Try with no dashes.
        phone = "5555555555";
        phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
        assert_eq!(phone_parsed, expected);
        assert_eq!(expected_str, serde_json::json!(phone_parsed));

        // Try with parens and spaces.
        phone = "(510) 864-1234";
        phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
        expected = PhoneNumber(phonenumber::parse(None, "+15108641234").unwrap());
        assert_eq!(phone_parsed, expected);
        expected_str = "+1 510-864-1234";
        assert_eq!(expected_str, serde_json::json!(phone_parsed));

        // Try with only parens.
        phone = "(510)8641234";
        phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
        assert_eq!(phone_parsed, expected);
        expected_str = "+1 510-864-1234";
        assert_eq!(expected_str, serde_json::json!(phone_parsed));

        // Try empty.
        assert_eq!(
            serde_json::from_str::<PhoneNumber>(r#""""#)
                .err()
                .unwrap()
                .to_string(),
            "phone number cannot be empty"
        );

        // Europe.
        phone = "+49 30  1234 1234";
        phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
        expected = PhoneNumber(phonenumber::parse(None, phone).unwrap());
        assert_eq!(phone_parsed, expected);
        expected_str = "+49 30 12341234";
        assert_eq!(expected_str, serde_json::json!(phone_parsed));
    }
}
