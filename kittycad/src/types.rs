#![doc = r" This module contains the generated types for the library."]
#[cfg(feature = "tabled")]
use tabled::Tabled;
pub mod base64 {
    #![doc = " Base64 data that encodes to url safe base64, but can decode from multiple"]
    #![doc = " base64 implementations to account for various clients and libraries. Compatible"]
    #![doc = " with serde and JsonSchema."]
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
    #[doc = " A container for binary that should be base64 encoded in serialisation. In reverse"]
    #[doc = " when deserializing, will decode from many different types of base64 possible."]
    pub struct Base64Data(pub Vec<u8>);
    impl Base64Data {
        #[doc = " Return is the data is empty."]
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
    impl<'de> Visitor<'de> for Base64DataVisitor {
        type Value = Base64Data;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a base64 encoded string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
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
}

#[cfg(feature = "requests")]
pub mod paginate {
    #![doc = " Utility functions used for pagination."]
    use anyhow::Result;
    #[doc = " A trait for types that allow pagination."]
    pub trait Pagination {
        #[doc = " The item that is paginated."]
        type Item: serde::de::DeserializeOwned;
        #[doc = " Returns true if the response has more pages."]
        fn has_more_pages(&self) -> bool;
        #[doc = " Modify a request to get the next page."]
        fn next_page(
            &self,
            req: reqwest::Request,
        ) -> Result<reqwest::Request, crate::types::error::Error>;
        #[doc = " Get the items from a page."]
        fn items(&self) -> Vec<Self::Item>;
    }
}

pub mod phone_number {
    #![doc = " A library to implement phone numbers for our database and JSON serialization and \
              deserialization."]
    use std::str::FromStr;

    use schemars::JsonSchema;
    #[doc = " A phone number."]
    #[derive(Debug, Default, Clone, PartialEq, Hash, Eq)]
    pub struct PhoneNumber(pub Option<phonenumber::PhoneNumber>);
    impl From<phonenumber::PhoneNumber> for PhoneNumber {
        fn from(id: phonenumber::PhoneNumber) -> PhoneNumber {
            PhoneNumber(Some(id))
        }
    }

    impl AsRef<Option<phonenumber::PhoneNumber>> for PhoneNumber {
        fn as_ref(&self) -> &Option<phonenumber::PhoneNumber> {
            &self.0
        }
    }

    impl std::ops::Deref for PhoneNumber {
        type Target = Option<phonenumber::PhoneNumber>;
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
                return Ok(PhoneNumber(None));
            }
            let s = if !s.trim().starts_with('+') {
                format!("+1{s}")
            } else {
                s.to_string()
            }
            .replace(['-', '(', ')', ' '], "");
            Ok(PhoneNumber(Some(phonenumber::parse(None, &s).map_err(
                |e| anyhow::anyhow!("invalid phone number `{}`: {}", s, e),
            )?)))
        }
    }

    impl std::fmt::Display for PhoneNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = if let Some(phone) = &self.0 {
                phone
                    .format()
                    .mode(phonenumber::Mode::International)
                    .to_string()
            } else {
                String::new()
            };
            write!(f, "{}", s)
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
            let mut expected = PhoneNumber(Some(phonenumber::parse(None, phone).unwrap()));
            assert_eq!(phone_parsed, expected);
            let mut expected_str = "+1 555-555-5555";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "555-555-5555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "+1 555-555-5555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "5555555555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "(510) 864-1234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            expected = PhoneNumber(Some(phonenumber::parse(None, "+15108641234").unwrap()));
            assert_eq!(phone_parsed, expected);
            expected_str = "+1 510-864-1234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "(510)8641234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            expected_str = "+1 510-864-1234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, PhoneNumber(None));
            assert_eq!("", serde_json::json!(phone_parsed));
            phone = "+49 30  1234 1234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            expected = PhoneNumber(Some(phonenumber::parse(None, phone).unwrap()));
            assert_eq!(phone_parsed, expected);
            expected_str = "+49 30 12341234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
        }
    }
}

#[cfg(feature = "requests")]
pub mod error {
    #![doc = " Error methods."]
    #[doc = " Error produced by generated client methods."]
    pub enum Error {
        #[doc = " The request did not conform to API requirements."]
        InvalidRequest(String),
        #[cfg(feature = "retry")]
        #[doc = " A server error either due to the data, or with the connection."]
        CommunicationError(reqwest_middleware::Error),
        #[doc = " A request error, caused when building the request."]
        RequestError(reqwest::Error),
        #[doc = " An expected response whose deserialization failed."]
        SerdeError {
            #[doc = " The error."]
            error: format_serde_error::SerdeError,
            #[doc = " The response status."]
            status: reqwest::StatusCode,
        },
        #[doc = " An expected error response."]
        InvalidResponsePayload {
            #[cfg(feature = "retry")]
            #[doc = " The error."]
            error: reqwest_middleware::Error,
            #[cfg(not(feature = "retry"))]
            #[doc = " The error."]
            error: reqwest::Error,
            #[doc = " The full response."]
            response: reqwest::Response,
        },
        #[doc = " A response not listed in the API description. This may represent a"]
        #[doc = " success or failure response; check `status().is_success()`."]
        UnexpectedResponse(reqwest::Response),
    }

    impl Error {
        #[doc = " Returns the status code, if the error was generated from a response."]
        pub fn status(&self) -> Option<reqwest::StatusCode> {
            match self {
                Error::InvalidRequest(_) => None,
                Error::RequestError(e) => e.status(),
                #[cfg(feature = "retry")]
                Error::CommunicationError(reqwest_middleware::Error::Reqwest(e)) => e.status(),
                #[cfg(feature = "retry")]
                Error::CommunicationError(reqwest_middleware::Error::Middleware(_)) => None,
                Error::SerdeError { error: _, status } => Some(*status),
                Error::InvalidResponsePayload { error: _, response } => Some(response.status()),
                Error::UnexpectedResponse(r) => Some(r.status()),
            }
        }

        #[doc = " Creates a new error from a response status and a serde error."]
        pub fn from_serde_error(
            e: format_serde_error::SerdeError,
            status: reqwest::StatusCode,
        ) -> Self {
            Self::SerdeError { error: e, status }
        }
    }

    #[cfg(feature = "retry")]
    impl From<reqwest_middleware::Error> for Error {
        fn from(e: reqwest_middleware::Error) -> Self {
            Self::CommunicationError(e)
        }
    }

    impl From<reqwest::Error> for Error {
        fn from(e: reqwest::Error) -> Self {
            Self::RequestError(e)
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::InvalidRequest(s) => {
                    write!(f, "Invalid Request: {}", s)
                }
                #[cfg(feature = "retry")]
                Error::CommunicationError(e) => {
                    write!(f, "Communication Error: {}", e)
                }
                Error::RequestError(e) => {
                    write!(f, "Request Error: {}", e)
                }
                Error::SerdeError { error, status: _ } => {
                    write!(f, "Serde Error: {}", error)
                }
                Error::InvalidResponsePayload { error, response: _ } => {
                    write!(f, "Invalid Response Payload: {}", error)
                }
                Error::UnexpectedResponse(r) => {
                    write!(f, "Unexpected Response: {:?}", r)
                }
            }
        }
    }

    trait ErrorFormat {
        fn fmt_info(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(self, f)
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                #[cfg(feature = "retry")]
                Error::CommunicationError(e) => Some(e),
                Error::SerdeError { error, status: _ } => Some(error),
                Error::InvalidResponsePayload { error, response: _ } => Some(error),
                _ => None,
            }
        }
    }
}

#[doc = "An account provider."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum AccountProvider {
    #[doc = "The Google account provider."]
    #[serde(rename = "google")]
    #[display("google")]
    Google,
    #[doc = "The GitHub account provider."]
    #[serde(rename = "github")]
    #[display("github")]
    Github,
}

#[doc = "AI plugin api information."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AiPluginApi {
    #[doc = "If the API is authenticated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_user_authenticated: Option<bool>,
    #[doc = "The type of API."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AiPluginApiType>,
    #[doc = "The url to the API's schema."]
    pub url: String,
}

impl std::fmt::Display for AiPluginApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AiPluginApi {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(is_user_authenticated) = &self.is_user_authenticated {
                format!("{:?}", is_user_authenticated).into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            self.url.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["is_user_authenticated".into(), "type_".into(), "url".into()]
    }
}

#[doc = "AI plugin api type."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[derive(Default)]
pub enum AiPluginApiType {
    #[doc = "An OpenAPI specification."]
    #[serde(rename = "openapi")]
    #[display("openapi")]
    #[default]
    Openapi,
}



#[doc = "AI plugin auth information."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AiPluginAuth {
    #[doc = "The type of http authorization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<AiPluginHttpAuthType>,
    #[doc = "The type of authentication."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AiPluginAuthType>,
}

impl std::fmt::Display for AiPluginAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AiPluginAuth {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(authorization_type) = &self.authorization_type {
                format!("{:?}", authorization_type).into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["authorization_type".into(), "type_".into()]
    }
}

#[doc = "AI plugin auth type."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum AiPluginAuthType {
    #[doc = "None."]
    #[serde(rename = "none")]
    #[display("none")]
    None,
    #[doc = "User http."]
    #[serde(rename = "user_http")]
    #[display("user_http")]
    UserHttp,
    #[doc = "Service http."]
    #[serde(rename = "service_http")]
    #[display("service_http")]
    ServiceHttp,
    #[doc = "OAuth."]
    #[serde(rename = "oauth")]
    #[display("oauth")]
    Oauth,
}

#[doc = "AI plugin http auth type."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum AiPluginHttpAuthType {
    #[doc = "Basic."]
    #[serde(rename = "basic")]
    #[display("basic")]
    Basic,
    #[doc = "Bearer."]
    #[serde(rename = "bearer")]
    #[display("bearer")]
    Bearer,
}

#[doc = "AI plugin manifest.\n\nThis is used for OpenAI's ChatGPT plugins. You can read more about them [here](https://platform.openai.com/docs/plugins/getting-started/plugin-manifest)."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AiPluginManifest {
    #[doc = "API specification."]
    pub api: AiPluginApi,
    #[doc = "Authentication schema."]
    pub auth: AiPluginAuth,
    #[doc = "Email contact for safety/moderation reachout, support, and deactivation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    #[doc = "Human-readable description of the plugin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_for_human: Option<String>,
    #[doc = "Description better tailored to the model, such as token context length \
             considerations or keyword usage for improved plugin prompting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_for_model: Option<String>,
    #[doc = "Redirect URL for users to view plugin information."]
    pub legal_info_url: String,
    #[doc = "URL used to fetch the plugin's logo."]
    pub logo_url: String,
    #[doc = "Human-readable name, such as the full company name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_for_human: Option<String>,
    #[doc = "Name the model will used to target the plugin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_for_model: Option<String>,
    #[doc = "Manifest schema version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

impl std::fmt::Display for AiPluginManifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AiPluginManifest {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.api).into(),
            format!("{:?}", self.auth).into(),
            if let Some(contact_email) = &self.contact_email {
                format!("{:?}", contact_email).into()
            } else {
                String::new().into()
            },
            if let Some(description_for_human) = &self.description_for_human {
                format!("{:?}", description_for_human).into()
            } else {
                String::new().into()
            },
            if let Some(description_for_model) = &self.description_for_model {
                format!("{:?}", description_for_model).into()
            } else {
                String::new().into()
            },
            self.legal_info_url.clone().into(),
            self.logo_url.clone().into(),
            if let Some(name_for_human) = &self.name_for_human {
                format!("{:?}", name_for_human).into()
            } else {
                String::new().into()
            },
            if let Some(name_for_model) = &self.name_for_model {
                format!("{:?}", name_for_model).into()
            } else {
                String::new().into()
            },
            if let Some(schema_version) = &self.schema_version {
                format!("{:?}", schema_version).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "api".into(),
            "auth".into(),
            "contact_email".into(),
            "description_for_human".into(),
            "description_for_model".into(),
            "legal_info_url".into(),
            "logo_url".into(),
            "name_for_human".into(),
            "name_for_model".into(),
            "schema_version".into(),
        ]
    }
}

#[doc = "An angle, with a specific unit."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Angle {
    #[doc = "What unit is the measurement?"]
    pub unit: UnitAngle,
    #[doc = "The size of the angle, measured in the chosen unit."]
    pub value: f64,
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Angle {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.unit).into(),
            format!("{:?}", self.value).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["unit".into(), "value".into()]
    }
}

#[doc = "Annotation line end type"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum AnnotationLineEnd {
    #[serde(rename = "none")]
    #[display("none")]
    None,
    #[serde(rename = "arrow")]
    #[display("arrow")]
    Arrow,
}

#[doc = "Options for annotation text"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnnotationLineEndOptions {
    #[doc = "How to style the end of the annotation line."]
    pub end: AnnotationLineEnd,
    #[doc = "How to style the start of the annotation line."]
    pub start: AnnotationLineEnd,
}

impl std::fmt::Display for AnnotationLineEndOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AnnotationLineEndOptions {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.end).into(),
            format!("{:?}", self.start).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["end".into(), "start".into()]
    }
}

#[doc = "Options for annotations"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnnotationOptions {
    #[doc = "Color to render the annotation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[doc = "How to style the start and end of the line"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_ends: Option<AnnotationLineEndOptions>,
    #[doc = "Width of the annotation's line"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_width: Option<f64>,
    #[doc = "Position to put the annotation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Point3D>,
    #[doc = "Text displayed on the annotation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<AnnotationTextOptions>,
}

impl std::fmt::Display for AnnotationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AnnotationOptions {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(color) = &self.color {
                format!("{:?}", color).into()
            } else {
                String::new().into()
            },
            if let Some(line_ends) = &self.line_ends {
                format!("{:?}", line_ends).into()
            } else {
                String::new().into()
            },
            if let Some(line_width) = &self.line_width {
                format!("{:?}", line_width).into()
            } else {
                String::new().into()
            },
            if let Some(position) = &self.position {
                format!("{:?}", position).into()
            } else {
                String::new().into()
            },
            if let Some(text) = &self.text {
                format!("{:?}", text).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "color".into(),
            "line_ends".into(),
            "line_width".into(),
            "position".into(),
            "text".into(),
        ]
    }
}

#[doc = "Horizontal Text aligment"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum AnnotationTextAlignmentX {
    #[serde(rename = "left")]
    #[display("left")]
    Left,
    #[serde(rename = "center")]
    #[display("center")]
    Center,
    #[serde(rename = "right")]
    #[display("right")]
    Right,
}

#[doc = "Vertical Text aligment"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum AnnotationTextAlignmentY {
    #[serde(rename = "bottom")]
    #[display("bottom")]
    Bottom,
    #[serde(rename = "center")]
    #[display("center")]
    Center,
    #[serde(rename = "top")]
    #[display("top")]
    Top,
}

#[doc = "Options for annotation text"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnnotationTextOptions {
    #[doc = "Text font's point size"]
    pub point_size: u32,
    #[doc = "Text displayed on the annotation"]
    pub text: String,
    #[doc = "Alignment along the X axis"]
    pub x: AnnotationTextAlignmentX,
    #[doc = "Alignment along the Y axis"]
    pub y: AnnotationTextAlignmentY,
}

impl std::fmt::Display for AnnotationTextOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AnnotationTextOptions {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.point_size).into(),
            self.text.clone().into(),
            format!("{:?}", self.x).into(),
            format!("{:?}", self.y).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["point_size".into(), "text".into(), "x".into(), "y".into()]
    }
}

#[doc = "The type of annotation"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum AnnotationType {
    #[doc = "2D annotation type (screen or planar space)"]
    #[serde(rename = "t2d")]
    #[display("t2d")]
    T2D,
    #[doc = "3D annotation type"]
    #[serde(rename = "t3d")]
    #[display("t3d")]
    T3D,
}

#[doc = "A response for a query on the API call table that is grouped by something."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCallQueryGroup {
    pub count: i64,
    pub query: String,
}

impl std::fmt::Display for ApiCallQueryGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCallQueryGroup {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            self.query.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["count".into(), "query".into()]
    }
}

#[doc = "The field of an API call to group by."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ApiCallQueryGroupBy {
    #[doc = "The email of the user that requested the API call."]
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[doc = "The HTTP method of the API call."]
    #[serde(rename = "method")]
    #[display("method")]
    Method,
    #[doc = "The endpoint of the API call."]
    #[serde(rename = "endpoint")]
    #[display("endpoint")]
    Endpoint,
    #[doc = "The user ID of the user that requested the API call."]
    #[serde(rename = "user_id")]
    #[display("user_id")]
    UserId,
    #[doc = "The origin of the API call. This is parsed from the `Origin` header."]
    #[serde(rename = "origin")]
    #[display("origin")]
    Origin,
    #[doc = "The IP address of the user making the API call."]
    #[serde(rename = "ip_address")]
    #[display("ip_address")]
    IpAddress,
}

#[doc = "The status of an async API call."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ApiCallStatus {
    #[doc = "The async API call is queued."]
    #[serde(rename = "queued")]
    #[display("queued")]
    Queued,
    #[doc = "The async API call was uploaded to be converted."]
    #[serde(rename = "uploaded")]
    #[display("uploaded")]
    Uploaded,
    #[doc = "The async API call is in progress."]
    #[serde(rename = "in_progress")]
    #[display("in_progress")]
    InProgress,
    #[doc = "The async API call has completed."]
    #[serde(rename = "completed")]
    #[display("completed")]
    Completed,
    #[doc = "The async API call has failed."]
    #[serde(rename = "failed")]
    #[display("failed")]
    Failed,
}

#[doc = "An API call with the price.\n\nThis is a join of the `ApiCall` and `ApiCallPrice` tables."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCallWithPrice {
    #[doc = "The date and time the API call completed billing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The date and time the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The duration of the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[doc = "The user's email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The endpoint requested by the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The unique identifier for the API call."]
    pub id: uuid::Uuid,
    #[doc = "The ip address of the origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<std::net::IpAddr>,
    #[doc = "If the API call was spawned from the litterbox or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub litterbox: Option<bool>,
    #[doc = "The HTTP method requsted by the API call."]
    pub method: Method,
    #[doc = "The number of minutes the API call was billed for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i32>,
    #[doc = "The origin of the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The price of the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<bigdecimal::BigDecimal>,
    #[doc = "The request body sent by the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_body: Option<String>,
    #[doc = "The request query params sent by the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_query_params: Option<String>,
    #[doc = "The response body returned by the API call. We do not store this information if it \
             is above a certain size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_body: Option<String>,
    #[doc = "The date and time the API call started billing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status code returned by the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[doc = "The Stripe invoice item ID of the API call if it is billable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stripe_invoice_item_id: Option<String>,
    #[doc = "The API token that made the API call."]
    pub token: uuid::Uuid,
    #[doc = "The date and time the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user agent of the request."]
    pub user_agent: String,
    #[doc = "The ID of the user that made the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for ApiCallWithPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCallWithPrice {
    const LENGTH: usize = 22;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(duration) = &self.duration {
                format!("{:?}", duration).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(endpoint) = &self.endpoint {
                format!("{:?}", endpoint).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(ip_address) = &self.ip_address {
                format!("{:?}", ip_address).into()
            } else {
                String::new().into()
            },
            if let Some(litterbox) = &self.litterbox {
                format!("{:?}", litterbox).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.method).into(),
            if let Some(minutes) = &self.minutes {
                format!("{:?}", minutes).into()
            } else {
                String::new().into()
            },
            if let Some(origin) = &self.origin {
                format!("{:?}", origin).into()
            } else {
                String::new().into()
            },
            if let Some(price) = &self.price {
                format!("{:?}", price).into()
            } else {
                String::new().into()
            },
            if let Some(request_body) = &self.request_body {
                format!("{:?}", request_body).into()
            } else {
                String::new().into()
            },
            if let Some(request_query_params) = &self.request_query_params {
                format!("{:?}", request_query_params).into()
            } else {
                String::new().into()
            },
            if let Some(response_body) = &self.response_body {
                format!("{:?}", response_body).into()
            } else {
                String::new().into()
            },
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            if let Some(status_code) = &self.status_code {
                format!("{:?}", status_code).into()
            } else {
                String::new().into()
            },
            if let Some(stripe_invoice_item_id) = &self.stripe_invoice_item_id {
                format!("{:?}", stripe_invoice_item_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.token).into(),
            format!("{:?}", self.updated_at).into(),
            self.user_agent.clone().into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "duration".into(),
            "email".into(),
            "endpoint".into(),
            "id".into(),
            "ip_address".into(),
            "litterbox".into(),
            "method".into(),
            "minutes".into(),
            "origin".into(),
            "price".into(),
            "request_body".into(),
            "request_query_params".into(),
            "response_body".into(),
            "started_at".into(),
            "status_code".into(),
            "stripe_invoice_item_id".into(),
            "token".into(),
            "updated_at".into(),
            "user_agent".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "A single page of results"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCallWithPriceResultsPage {
    #[doc = "list of items on this page of results"]
    pub items: Vec<ApiCallWithPrice>,
    #[doc = "token used to fetch the next page of results (if any)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

impl std::fmt::Display for ApiCallWithPriceResultsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ApiCallWithPriceResultsPage {
    type Item = ApiCallWithPrice;
    fn has_more_pages(&self) -> bool {
        self.next_page.is_some()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        req.url_mut()
            .query_pairs_mut()
            .append_pair("next_page", self.next_page.as_deref().unwrap_or(""));
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.items.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCallWithPriceResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.items).into(),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["items".into(), "next_page".into()]
    }
}

#[doc = "An error."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiError {
    #[doc = "The error code."]
    pub error_code: ErrorCode,
    #[doc = "The error message."]
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiError {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.error_code).into(),
            self.message.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["error_code".into(), "message".into()]
    }
}

#[doc = "An API token.\n\nThese are used to authenticate users with Bearer authentication."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiToken {
    #[doc = "The date and time the API token was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The unique identifier for the API token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "If the token is valid. We never delete API tokens, but we can mark them as invalid. \
             We save them for ever to preserve the history of the API token."]
    pub is_valid: bool,
    #[doc = "The API token itself."]
    pub token: uuid::Uuid,
    #[doc = "The date and time the API token was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The ID of the user that owns the API token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for ApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiToken {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.created_at).into(),
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_valid).into(),
            format!("{:?}", self.token).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "id".into(),
            "is_valid".into(),
            "token".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "A single page of results"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTokenResultsPage {
    #[doc = "list of items on this page of results"]
    pub items: Vec<ApiToken>,
    #[doc = "token used to fetch the next page of results (if any)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

impl std::fmt::Display for ApiTokenResultsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ApiTokenResultsPage {
    type Item = ApiToken;
    fn has_more_pages(&self) -> bool {
        self.next_page.is_some()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        req.url_mut()
            .query_pairs_mut()
            .append_pair("next_page", self.next_page.as_deref().unwrap_or(""));
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.items.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTokenResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.items).into(),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["items".into(), "next_page".into()]
    }
}

#[doc = "Information about a third party app client."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AppClientInfo {
    #[doc = "The URL for consent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl std::fmt::Display for AppClientInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AppClientInfo {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(url) = &self.url {
            format!("{:?}", url).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["url".into()]
    }
}

#[doc = "An async API call."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AsyncApiCall {
    #[doc = "The time and date the async API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the async API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the async API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The JSON input for the API call. These are determined by the endpoint that is run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[doc = "The JSON output for the API call. These are determined by the endpoint that is run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    #[doc = "The time and date the async API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the async API call."]
    pub status: ApiCallStatus,
    #[doc = "The type of async API call."]
    #[serde(rename = "type")]
    pub type_: AsyncApiCallType,
    #[doc = "The time and date the async API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the async API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The worker node that is performing or performed the async API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<String>,
}

impl std::fmt::Display for AsyncApiCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AsyncApiCall {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.type_).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "output".into(),
            "started_at".into(),
            "status".into(),
            "type_".into(),
            "updated_at".into(),
            "user_id".into(),
            "worker".into(),
        ]
    }
}

#[doc = "The output from the async API call."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type")]
pub enum AsyncApiCallOutput {
    #[serde(rename = "file_conversion")]
    FileConversion {
        #[doc = "The time and date the API call was completed."]
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The time and date the API call was created."]
        created_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The error the function returned, if any."]
        error: Option<String>,
        #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
        id: uuid::Uuid,
        #[doc = "The output format of the file conversion."]
        output_format: FileExportFormat,
        #[doc = "The output format options of the file conversion."]
        output_format_options: Option<OutputFormat>,
        #[doc = "The converted files (if multiple file conversion), if completed, base64 encoded. \
                 The key of the map is the path of the output file."]
        outputs: Option<std::collections::HashMap<String, base64::Base64Data>>,
        #[doc = "The source format of the file conversion."]
        src_format: FileImportFormat,
        #[doc = "The source format options of the file conversion."]
        src_format_options: Option<InputFormat>,
        #[doc = "The time and date the API call was started."]
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The status of the API call."]
        status: ApiCallStatus,
        #[doc = "The time and date the API call was last updated."]
        updated_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The user ID of the user who created the API call."]
        user_id: Option<String>,
    },
    #[serde(rename = "file_center_of_mass")]
    FileCenterOfMass {
        #[doc = "The resulting center of mass."]
        center_of_mass: Option<Point3D>,
        #[doc = "The time and date the API call was completed."]
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The time and date the API call was created."]
        created_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The error the function returned, if any."]
        error: Option<String>,
        #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
        id: uuid::Uuid,
        #[doc = "The output unit for the center of mass."]
        output_unit: UnitLength,
        #[doc = "The source format of the file."]
        src_format: FileImportFormat,
        #[doc = "The time and date the API call was started."]
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The status of the API call."]
        status: ApiCallStatus,
        #[doc = "The time and date the API call was last updated."]
        updated_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The user ID of the user who created the API call."]
        user_id: Option<String>,
    },
    #[serde(rename = "file_mass")]
    FileMass {
        #[doc = "The time and date the API call was completed."]
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The time and date the API call was created."]
        created_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The error the function returned, if any."]
        error: Option<String>,
        #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
        id: uuid::Uuid,
        #[doc = "The resulting mass."]
        mass: Option<f64>,
        #[doc = "The material density as denoted by the user."]
        material_density: Option<f64>,
        #[doc = "The material density unit."]
        material_density_unit: UnitDensity,
        #[doc = "The output unit for the mass."]
        output_unit: UnitMass,
        #[doc = "The source format of the file."]
        src_format: FileImportFormat,
        #[doc = "The time and date the API call was started."]
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The status of the API call."]
        status: ApiCallStatus,
        #[doc = "The time and date the API call was last updated."]
        updated_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The user ID of the user who created the API call."]
        user_id: Option<String>,
    },
    #[serde(rename = "file_volume")]
    FileVolume {
        #[doc = "The time and date the API call was completed."]
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The time and date the API call was created."]
        created_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The error the function returned, if any."]
        error: Option<String>,
        #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
        id: uuid::Uuid,
        #[doc = "The output unit for the volume."]
        output_unit: UnitVolume,
        #[doc = "The source format of the file."]
        src_format: FileImportFormat,
        #[doc = "The time and date the API call was started."]
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The status of the API call."]
        status: ApiCallStatus,
        #[doc = "The time and date the API call was last updated."]
        updated_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The user ID of the user who created the API call."]
        user_id: Option<String>,
        #[doc = "The resulting volume."]
        volume: Option<f64>,
    },
    #[serde(rename = "file_density")]
    FileDensity {
        #[doc = "The time and date the API call was completed."]
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The time and date the API call was created."]
        created_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The resulting density."]
        density: Option<f64>,
        #[doc = "The error the function returned, if any."]
        error: Option<String>,
        #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
        id: uuid::Uuid,
        #[doc = "The material mass as denoted by the user."]
        material_mass: Option<f64>,
        #[doc = "The material mass unit."]
        material_mass_unit: UnitMass,
        #[doc = "The output unit for the density."]
        output_unit: UnitDensity,
        #[doc = "The source format of the file."]
        src_format: FileImportFormat,
        #[doc = "The time and date the API call was started."]
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The status of the API call."]
        status: ApiCallStatus,
        #[doc = "The time and date the API call was last updated."]
        updated_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The user ID of the user who created the API call."]
        user_id: Option<String>,
    },
    #[serde(rename = "file_surface_area")]
    FileSurfaceArea {
        #[doc = "The time and date the API call was completed."]
        completed_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The time and date the API call was created."]
        created_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The error the function returned, if any."]
        error: Option<String>,
        #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
        id: uuid::Uuid,
        #[doc = "The output unit for the surface area."]
        output_unit: UnitArea,
        #[doc = "The source format of the file."]
        src_format: FileImportFormat,
        #[doc = "The time and date the API call was started."]
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        #[doc = "The status of the API call."]
        status: ApiCallStatus,
        #[doc = "The resulting surface area."]
        surface_area: Option<f64>,
        #[doc = "The time and date the API call was last updated."]
        updated_at: chrono::DateTime<chrono::Utc>,
        #[doc = "The user ID of the user who created the API call."]
        user_id: Option<String>,
    },
}

#[doc = "A single page of results"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AsyncApiCallResultsPage {
    #[doc = "list of items on this page of results"]
    pub items: Vec<AsyncApiCall>,
    #[doc = "token used to fetch the next page of results (if any)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

impl std::fmt::Display for AsyncApiCallResultsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for AsyncApiCallResultsPage {
    type Item = AsyncApiCall;
    fn has_more_pages(&self) -> bool {
        self.next_page.is_some()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        req.url_mut()
            .query_pairs_mut()
            .append_pair("next_page", self.next_page.as_deref().unwrap_or(""));
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.items.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AsyncApiCallResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.items).into(),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["items".into(), "next_page".into()]
    }
}

#[doc = "The type of async API call."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum AsyncApiCallType {
    #[doc = "File conversion."]
    #[serde(rename = "file_conversion")]
    #[display("file_conversion")]
    FileConversion,
    #[doc = "File volume."]
    #[serde(rename = "file_volume")]
    #[display("file_volume")]
    FileVolume,
    #[doc = "File center of mass."]
    #[serde(rename = "file_center_of_mass")]
    #[display("file_center_of_mass")]
    FileCenterOfMass,
    #[doc = "File mass."]
    #[serde(rename = "file_mass")]
    #[display("file_mass")]
    FileMass,
    #[doc = "File density."]
    #[serde(rename = "file_density")]
    #[display("file_density")]
    FileDensity,
    #[doc = "File surface area."]
    #[serde(rename = "file_surface_area")]
    #[display("file_surface_area")]
    FileSurfaceArea,
}

#[doc = "Co-ordinate axis specifier.\n\nSee [cglearn.eu] for background reading.\n\n[cglearn.eu]: https://cglearn.eu/pub/computer-graphics/introduction-to-geometry#material-coordinate-systems-1"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Axis {
    #[doc = "'Y' axis."]
    #[serde(rename = "y")]
    #[display("y")]
    Y,
    #[doc = "'Z' axis."]
    #[serde(rename = "z")]
    #[display("z")]
    Z,
}

#[doc = "An [`Axis`] paired with a [`Direction`]."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AxisDirectionPair {
    #[doc = "Axis specifier."]
    pub axis: Axis,
    #[doc = "Specifies which direction the axis is pointing."]
    pub direction: Direction,
}

impl std::fmt::Display for AxisDirectionPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AxisDirectionPair {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.axis).into(),
            format!("{:?}", self.direction).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["axis".into(), "direction".into()]
    }
}

#[doc = "The billing information for payments."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BillingInfo {
    #[doc = "The address of the customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<NewAddress>,
    #[doc = "The name of the customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The phone for the customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: phone_number::PhoneNumber,
}

impl std::fmt::Display for BillingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BillingInfo {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(address) = &self.address {
                format!("{:?}", address).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.phone).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["address".into(), "name".into(), "phone".into()]
    }
}

#[doc = "Metadata about our cache.\n\nThis is mostly used for internal purposes and debugging."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CacheMetadata {
    #[doc = "If the cache returned an ok response from ping."]
    pub ok: bool,
}

impl std::fmt::Display for CacheMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CacheMetadata {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.ok).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["ok".into()]
    }
}

#[doc = "The type of camera drag interaction."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum CameraDragInteractionType {
    #[doc = "Camera pan"]
    #[serde(rename = "pan")]
    #[display("pan")]
    Pan,
    #[doc = "Camera rotate (revolve/orbit)"]
    #[serde(rename = "rotate")]
    #[display("rotate")]
    Rotate,
    #[doc = "Camera zoom (increase or decrease distance to reference point center)"]
    #[serde(rename = "zoom")]
    #[display("zoom")]
    Zoom,
}

#[doc = "The card details of a payment method."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardDetails {
    #[doc = "Card brand.\n\nCan be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, \
             `visa`, or `unknown`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[doc = "Checks on Card address and CVC if provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<PaymentMethodCardChecks>,
    #[doc = "Two-letter ISO code representing the country of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Two-digit number representing the card's expiration month."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[doc = "Four-digit number representing the card's expiration year."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    #[doc = "Uniquely identifies this particular card number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Card funding type.\n\nCan be `credit`, `debit`, `prepaid`, or `unknown`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[doc = "The last four digits of the card."]
    #[serde(rename = "last4", default, skip_serializing_if = "Option::is_none")]
    pub last_4: Option<String>,
}

impl std::fmt::Display for CardDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardDetails {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(brand) = &self.brand {
                format!("{:?}", brand).into()
            } else {
                String::new().into()
            },
            if let Some(checks) = &self.checks {
                format!("{:?}", checks).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(exp_month) = &self.exp_month {
                format!("{:?}", exp_month).into()
            } else {
                String::new().into()
            },
            if let Some(exp_year) = &self.exp_year {
                format!("{:?}", exp_year).into()
            } else {
                String::new().into()
            },
            if let Some(fingerprint) = &self.fingerprint {
                format!("{:?}", fingerprint).into()
            } else {
                String::new().into()
            },
            if let Some(funding) = &self.funding {
                format!("{:?}", funding).into()
            } else {
                String::new().into()
            },
            if let Some(last_4) = &self.last_4 {
                format!("{:?}", last_4).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "brand".into(),
            "checks".into(),
            "country".into(),
            "exp_month".into(),
            "exp_year".into(),
            "fingerprint".into(),
            "funding".into(),
            "last_4".into(),
        ]
    }
}

#[doc = "The center of mass response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CenterOfMass {
    #[doc = "The center of mass."]
    pub center_of_mass: Point3D,
    #[doc = "The output unit for the center of mass."]
    pub output_unit: UnitLength,
}

impl std::fmt::Display for CenterOfMass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CenterOfMass {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.center_of_mass).into(),
            format!("{:?}", self.output_unit).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["center_of_mass".into(), "output_unit".into()]
    }
}

#[doc = "ClientMetrics contains information regarding the state of the peer."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ClientMetrics {
    #[doc = "Counter of the number of WebRTC frames that the client has decoded during this \
             session."]
    pub rtc_frames_decoded: u64,
    #[doc = "Counter of the number of WebRTC frames the client has dropped during this session."]
    pub rtc_frames_dropped: u32,
    #[doc = "Current number of frames being rendered per second. A good target is 60 frames per \
             second, but it can fluctuate depending on network conditions."]
    pub rtc_frames_per_second: u8,
    #[doc = "Counter of the number of WebRTC frames that the client has received during this \
             session."]
    pub rtc_frames_received: u64,
    #[doc = "Number of times the WebRTC playback has frozen. This is usually due to network \
             conditions."]
    pub rtc_freeze_count: u32,
    #[doc = "Amount of \"jitter\" in the WebRTC session. Network latency is the time it takes a \
             packet to traverse the network. The amount that the latency varies is the jitter. \
             Video latency is the time it takes to render a frame sent by the server (including \
             network latency). A low jitter means the video latency can be reduced without \
             impacting smooth playback. High jitter means clients will increase video latency to \
             ensure smooth playback."]
    pub rtc_jitter_sec: f64,
    #[doc = "Number of \"key frames\" decoded in the underlying h.264 stream. A key frame is an \
             expensive (bandwidth-wise) \"full image\" of the video frame. Data after the \
             keyframe become -- effectively -- \"diff\" operations on that key frame. The Engine \
             will only send a keyframe if required, which is an indication that some of the \
             \"diffs\" have been lost, usually an indication of poor network conditions. We like \
             this metric to understand times when the connection has had to recover."]
    pub rtc_keyframes_decoded: u32,
    #[doc = "Number of seconds of frozen video the user has been subjected to."]
    pub rtc_total_freezes_duration_sec: f64,
}

impl std::fmt::Display for ClientMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ClientMetrics {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.rtc_frames_decoded).into(),
            format!("{:?}", self.rtc_frames_dropped).into(),
            format!("{:?}", self.rtc_frames_per_second).into(),
            format!("{:?}", self.rtc_frames_received).into(),
            format!("{:?}", self.rtc_freeze_count).into(),
            format!("{:?}", self.rtc_jitter_sec).into(),
            format!("{:?}", self.rtc_keyframes_decoded).into(),
            format!("{:?}", self.rtc_total_freezes_duration_sec).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "rtc_frames_decoded".into(),
            "rtc_frames_dropped".into(),
            "rtc_frames_per_second".into(),
            "rtc_frames_received".into(),
            "rtc_freeze_count".into(),
            "rtc_jitter_sec".into(),
            "rtc_keyframes_decoded".into(),
            "rtc_total_freezes_duration_sec".into(),
        ]
    }
}

#[doc = "Cluster information."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Cluster {
    #[doc = "The IP address of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[doc = "The auth timeout of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_timeout: Option<i64>,
    #[doc = "The port of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_port: Option<i64>,
    #[doc = "The name of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The TLS timeout for the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_timeout: Option<i64>,
    #[doc = "The urls of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl std::fmt::Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Cluster {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(addr) = &self.addr {
                format!("{:?}", addr).into()
            } else {
                String::new().into()
            },
            if let Some(auth_timeout) = &self.auth_timeout {
                format!("{:?}", auth_timeout).into()
            } else {
                String::new().into()
            },
            if let Some(cluster_port) = &self.cluster_port {
                format!("{:?}", cluster_port).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(tls_timeout) = &self.tls_timeout {
                format!("{:?}", tls_timeout).into()
            } else {
                String::new().into()
            },
            if let Some(urls) = &self.urls {
                format!("{:?}", urls).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "addr".into(),
            "auth_timeout".into(),
            "cluster_port".into(),
            "name".into(),
            "tls_timeout".into(),
            "urls".into(),
        ]
    }
}

#[doc = "The language code is written in."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum CodeLanguage {
    #[doc = "The `go` programming language."]
    #[serde(rename = "go")]
    #[display("go")]
    Go,
    #[doc = "The `python` programming language."]
    #[serde(rename = "python")]
    #[display("python")]
    Python,
    #[doc = "The `node` programming language."]
    #[serde(rename = "node")]
    #[display("node")]
    Node,
}

#[doc = "Output of the code being executed."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CodeOutput {
    #[doc = "The contents of the files requested if they were passed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_files: Option<Vec<OutputFile>>,
    #[doc = "The stderr of the code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    #[doc = "The stdout of the code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
}

impl std::fmt::Display for CodeOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CodeOutput {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(output_files) = &self.output_files {
                format!("{:?}", output_files).into()
            } else {
                String::new().into()
            },
            if let Some(stderr) = &self.stderr {
                format!("{:?}", stderr).into()
            } else {
                String::new().into()
            },
            if let Some(stdout) = &self.stdout {
                format!("{:?}", stdout).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["output_files".into(), "stderr".into(), "stdout".into()]
    }
}

#[doc = "An RGBA color"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Color {
    #[doc = "Alpha"]
    pub a: f64,
    #[doc = "Blue"]
    pub b: f64,
    #[doc = "Green"]
    pub g: f64,
    #[doc = "Red"]
    pub r: f64,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Color {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.a).into(),
            format!("{:?}", self.b).into(),
            format!("{:?}", self.g).into(),
            format!("{:?}", self.r).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["a".into(), "b".into(), "g".into(), "r".into()]
    }
}

#[doc = "Metadata about a pub-sub connection.\n\nThis is mostly used for internal purposes and \
         debugging."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Connection {
    #[doc = "The auth timeout of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_timeout: Option<i64>,
    #[doc = "Information about the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
    #[doc = "The time the configuration was loaded."]
    pub config_load_time: chrono::DateTime<chrono::Utc>,
    #[doc = "The number of connections to the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connections: Option<i64>,
    #[doc = "The CPU core usage of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<i64>,
    #[doc = "The CPU usage of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[doc = "Information about the gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gateway>,
    #[doc = "The git commit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    #[doc = "The go version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub go: Option<String>,
    #[doc = "`GOMAXPROCS` of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gomaxprocs: Option<i64>,
    #[doc = "The host of the server."]
    pub host: std::net::IpAddr,
    #[doc = "The http base path of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_base_path: Option<String>,
    #[doc = "The http host of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_host: Option<String>,
    #[doc = "The http port of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[doc = "HTTP request statistics."]
    pub http_req_stats: std::collections::HashMap<String, i64>,
    #[doc = "The https port of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[doc = "The count of inbound bytes for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_bytes: Option<i64>,
    #[doc = "The number of inbound messages for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_msgs: Option<i64>,
    #[doc = "Jetstream information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jetstream: Option<Jetstream>,
    #[doc = "Information about leaf nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaf: Option<LeafNode>,
    #[doc = "The number of leaf nodes for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leafnodes: Option<i64>,
    #[doc = "The max connections of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    #[doc = "The max control line of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_control_line: Option<i64>,
    #[doc = "The max payload of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_payload: Option<i64>,
    #[doc = "The max pending of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_pending: Option<i64>,
    #[doc = "The memory usage of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem: Option<i64>,
    #[doc = "The time now."]
    pub now: chrono::DateTime<chrono::Utc>,
    #[doc = "The count of outbound bytes for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_bytes: Option<i64>,
    #[doc = "The number of outbound messages for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_msgs: Option<i64>,
    #[doc = "The ping interval of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ping_interval: Option<i64>,
    #[doc = "The ping max of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ping_max: Option<i64>,
    #[doc = "The port of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "The protocol version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proto: Option<i64>,
    #[doc = "The number of remotes for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remotes: Option<i64>,
    #[doc = "The number of routes for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<i64>,
    #[doc = "The server ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[doc = "The server name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "The number of slow consumers for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slow_consumers: Option<i64>,
    #[doc = "When the server was started."]
    pub start: chrono::DateTime<chrono::Utc>,
    #[doc = "The number of subscriptions for the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<i64>,
    #[doc = "The system account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_account: Option<String>,
    #[doc = "The TLS timeout of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_timeout: Option<i64>,
    #[doc = "The total number of connections to the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_connections: Option<i64>,
    #[doc = "The uptime of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
    #[doc = "The version of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The write deadline of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_deadline: Option<i64>,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Connection {
    const LENGTH: usize = 46;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(auth_timeout) = &self.auth_timeout {
                format!("{:?}", auth_timeout).into()
            } else {
                String::new().into()
            },
            if let Some(cluster) = &self.cluster {
                format!("{:?}", cluster).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.config_load_time).into(),
            if let Some(connections) = &self.connections {
                format!("{:?}", connections).into()
            } else {
                String::new().into()
            },
            if let Some(cores) = &self.cores {
                format!("{:?}", cores).into()
            } else {
                String::new().into()
            },
            if let Some(cpu) = &self.cpu {
                format!("{:?}", cpu).into()
            } else {
                String::new().into()
            },
            if let Some(gateway) = &self.gateway {
                format!("{:?}", gateway).into()
            } else {
                String::new().into()
            },
            if let Some(git_commit) = &self.git_commit {
                format!("{:?}", git_commit).into()
            } else {
                String::new().into()
            },
            if let Some(go) = &self.go {
                format!("{:?}", go).into()
            } else {
                String::new().into()
            },
            if let Some(gomaxprocs) = &self.gomaxprocs {
                format!("{:?}", gomaxprocs).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.host).into(),
            if let Some(http_base_path) = &self.http_base_path {
                format!("{:?}", http_base_path).into()
            } else {
                String::new().into()
            },
            if let Some(http_host) = &self.http_host {
                format!("{:?}", http_host).into()
            } else {
                String::new().into()
            },
            if let Some(http_port) = &self.http_port {
                format!("{:?}", http_port).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.http_req_stats).into(),
            if let Some(https_port) = &self.https_port {
                format!("{:?}", https_port).into()
            } else {
                String::new().into()
            },
            if let Some(in_bytes) = &self.in_bytes {
                format!("{:?}", in_bytes).into()
            } else {
                String::new().into()
            },
            if let Some(in_msgs) = &self.in_msgs {
                format!("{:?}", in_msgs).into()
            } else {
                String::new().into()
            },
            if let Some(jetstream) = &self.jetstream {
                format!("{:?}", jetstream).into()
            } else {
                String::new().into()
            },
            if let Some(leaf) = &self.leaf {
                format!("{:?}", leaf).into()
            } else {
                String::new().into()
            },
            if let Some(leafnodes) = &self.leafnodes {
                format!("{:?}", leafnodes).into()
            } else {
                String::new().into()
            },
            if let Some(max_connections) = &self.max_connections {
                format!("{:?}", max_connections).into()
            } else {
                String::new().into()
            },
            if let Some(max_control_line) = &self.max_control_line {
                format!("{:?}", max_control_line).into()
            } else {
                String::new().into()
            },
            if let Some(max_payload) = &self.max_payload {
                format!("{:?}", max_payload).into()
            } else {
                String::new().into()
            },
            if let Some(max_pending) = &self.max_pending {
                format!("{:?}", max_pending).into()
            } else {
                String::new().into()
            },
            if let Some(mem) = &self.mem {
                format!("{:?}", mem).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.now).into(),
            if let Some(out_bytes) = &self.out_bytes {
                format!("{:?}", out_bytes).into()
            } else {
                String::new().into()
            },
            if let Some(out_msgs) = &self.out_msgs {
                format!("{:?}", out_msgs).into()
            } else {
                String::new().into()
            },
            if let Some(ping_interval) = &self.ping_interval {
                format!("{:?}", ping_interval).into()
            } else {
                String::new().into()
            },
            if let Some(ping_max) = &self.ping_max {
                format!("{:?}", ping_max).into()
            } else {
                String::new().into()
            },
            if let Some(port) = &self.port {
                format!("{:?}", port).into()
            } else {
                String::new().into()
            },
            if let Some(proto) = &self.proto {
                format!("{:?}", proto).into()
            } else {
                String::new().into()
            },
            if let Some(remotes) = &self.remotes {
                format!("{:?}", remotes).into()
            } else {
                String::new().into()
            },
            if let Some(routes) = &self.routes {
                format!("{:?}", routes).into()
            } else {
                String::new().into()
            },
            if let Some(server_id) = &self.server_id {
                format!("{:?}", server_id).into()
            } else {
                String::new().into()
            },
            if let Some(server_name) = &self.server_name {
                format!("{:?}", server_name).into()
            } else {
                String::new().into()
            },
            if let Some(slow_consumers) = &self.slow_consumers {
                format!("{:?}", slow_consumers).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.start).into(),
            if let Some(subscriptions) = &self.subscriptions {
                format!("{:?}", subscriptions).into()
            } else {
                String::new().into()
            },
            if let Some(system_account) = &self.system_account {
                format!("{:?}", system_account).into()
            } else {
                String::new().into()
            },
            if let Some(tls_timeout) = &self.tls_timeout {
                format!("{:?}", tls_timeout).into()
            } else {
                String::new().into()
            },
            if let Some(total_connections) = &self.total_connections {
                format!("{:?}", total_connections).into()
            } else {
                String::new().into()
            },
            if let Some(uptime) = &self.uptime {
                format!("{:?}", uptime).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(write_deadline) = &self.write_deadline {
                format!("{:?}", write_deadline).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "auth_timeout".into(),
            "cluster".into(),
            "config_load_time".into(),
            "connections".into(),
            "cores".into(),
            "cpu".into(),
            "gateway".into(),
            "git_commit".into(),
            "go".into(),
            "gomaxprocs".into(),
            "host".into(),
            "http_base_path".into(),
            "http_host".into(),
            "http_port".into(),
            "http_req_stats".into(),
            "https_port".into(),
            "in_bytes".into(),
            "in_msgs".into(),
            "jetstream".into(),
            "leaf".into(),
            "leafnodes".into(),
            "max_connections".into(),
            "max_control_line".into(),
            "max_payload".into(),
            "max_pending".into(),
            "mem".into(),
            "now".into(),
            "out_bytes".into(),
            "out_msgs".into(),
            "ping_interval".into(),
            "ping_max".into(),
            "port".into(),
            "proto".into(),
            "remotes".into(),
            "routes".into(),
            "server_id".into(),
            "server_name".into(),
            "slow_consumers".into(),
            "start".into(),
            "subscriptions".into(),
            "system_account".into(),
            "tls_timeout".into(),
            "total_connections".into(),
            "uptime".into(),
            "version".into(),
            "write_deadline".into(),
        ]
    }
}

#[doc = "The resource representing a Coupon."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Coupon {
    #[doc = "Amount (in the `currency` specified) that will be taken off the subtotal of any \
             invoices for this customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<bigdecimal::BigDecimal>,
    #[doc = "Always true for a deleted object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[doc = "Unique identifier for the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Percent that will be taken off the subtotal of any invoices for this customer for \
             the duration of the coupon.\n\nFor example, a coupon with percent_off of 50 will \
             make a %s100 invoice %s50 instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,
}

impl std::fmt::Display for Coupon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Coupon {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(amount_off) = &self.amount_off {
                format!("{:?}", amount_off).into()
            } else {
                String::new().into()
            },
            if let Some(deleted) = &self.deleted {
                format!("{:?}", deleted).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(percent_off) = &self.percent_off {
                format!("{:?}", percent_off).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount_off".into(),
            "deleted".into(),
            "id".into(),
            "percent_off".into(),
        ]
    }
}

#[doc = "Supported set of sort modes for scanning by created_at only.\n\nCurrently, we only \
         support scanning in ascending order."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum CreatedAtSortMode {
    #[doc = "Sort in increasing order of \"created_at\"."]
    #[serde(rename = "created_at_ascending")]
    #[display("created_at_ascending")]
    CreatedAtAscending,
    #[doc = "Sort in decreasing order of \"created_at\"."]
    #[serde(rename = "created_at_descending")]
    #[display("created_at_descending")]
    CreatedAtDescending,
}

#[doc = "The response from the `CurveGetControlPoints` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CurveGetControlPoints {
    #[doc = "Control points in the curve."]
    pub control_points: Vec<Point3D>,
}

impl std::fmt::Display for CurveGetControlPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CurveGetControlPoints {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.control_points).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["control_points".into()]
    }
}

#[doc = "Endpoints of a curve"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CurveGetEndPoints {
    #[doc = "End"]
    pub end: Point3D,
    #[doc = "Start"]
    pub start: Point3D,
}

impl std::fmt::Display for CurveGetEndPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CurveGetEndPoints {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.end).into(),
            format!("{:?}", self.start).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["end".into(), "start".into()]
    }
}

#[doc = "The response from the `CurveGetType` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CurveGetType {
    #[doc = "Curve type"]
    pub curve_type: CurveType,
}

impl std::fmt::Display for CurveGetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CurveGetType {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.curve_type).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["curve_type".into()]
    }
}

#[doc = "The type of Curve (embedded within path)"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum CurveType {
    #[serde(rename = "line")]
    #[display("line")]
    Line,
    #[serde(rename = "arc")]
    #[display("arc")]
    Arc,
    #[serde(rename = "nurbs")]
    #[display("nurbs")]
    Nurbs,
}

#[doc = "The resource representing a payment \"Customer\"."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Customer {
    #[doc = "The customer's address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<NewAddress>,
    #[doc = "Current balance, if any, being stored on the customer in the payments service.\n\nIf \
             negative, the customer has credit to apply to their next invoice. If positive, the \
             customer has an amount owed that will be added to their next invoice. The balance \
             does not refer to any unpaid invoices; it solely takes into account amounts that \
             have yet to be successfully applied to any invoice. This balance is only taken into \
             account as invoices are finalized."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<bigdecimal::BigDecimal>,
    #[doc = "Time at which the object was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "Three-letter ISO code for the currency the customer can be charged in for recurring \
             billing purposes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "When the customer's latest invoice is billed by charging automatically, `delinquent` \
             is `true` if the invoice's latest charge failed.\n\nWhen the customer's latest \
             invoice is billed by sending an invoice, `delinquent` is `true` if the invoice isn't \
             paid by its due date.  If an invoice is marked uncollectible by dunning, \
             `delinquent` doesn't get reset to `false`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delinquent: Option<bool>,
    #[doc = "The customer's email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Set of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The customer's full name or business name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The customer's phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: phone_number::PhoneNumber,
}

impl std::fmt::Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Customer {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(address) = &self.address {
                format!("{:?}", address).into()
            } else {
                String::new().into()
            },
            if let Some(balance) = &self.balance {
                format!("{:?}", balance).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            if let Some(delinquent) = &self.delinquent {
                format!("{:?}", delinquent).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.phone).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "address".into(),
            "balance".into(),
            "created_at".into(),
            "currency".into(),
            "delinquent".into(),
            "email".into(),
            "id".into(),
            "metadata".into(),
            "name".into(),
            "phone".into(),
        ]
    }
}

#[doc = "A balance for a user.\n\nThis holds information about the financial balance for the user."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomerBalance {
    #[doc = "The date and time the balance was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The unique identifier for the balance."]
    pub id: uuid::Uuid,
    #[doc = "The monthy credits remaining in the balance. This gets re-upped every month, but if \
             the credits are not used for a month they do not carry over to the next month. It is \
             a stable amount granted to the user per month."]
    pub monthly_credits_remaining: bigdecimal::BigDecimal,
    #[doc = "The amount of pre-pay cash remaining in the balance. This number goes down as the \
             user uses their pre-paid credits. The reason we track this amount is if a user ever \
             wants to withdraw their pre-pay cash, we can use this amount to determine how much \
             to give them. Say a user has $100 in pre-paid cash, their bill is worth, $50 after \
             subtracting any other credits (like monthly etc.) Their bill is $50, their pre-pay \
             cash remaining will be subtracted by 50 to pay the bill and their \
             `pre_pay_credits_remaining` will be subtracted by 50 to pay the bill. This way if \
             they want to withdraw money after, they can only withdraw $50 since that is the \
             amount of cash they have remaining."]
    pub pre_pay_cash_remaining: bigdecimal::BigDecimal,
    #[doc = "The amount of credits remaining in the balance. This is typically the amount of cash \
             * some multiplier they get for pre-paying their account. This number lowers every \
             time a bill is paid with the balance. This number increases every time a user adds \
             funds to their balance. This may be through a subscription or a one off payment."]
    pub pre_pay_credits_remaining: bigdecimal::BigDecimal,
    #[doc = "This includes any outstanding, draft, or open invoices and any pending invoice \
             items. This does not include any credits the user has on their account."]
    pub total_due: bigdecimal::BigDecimal,
    #[doc = "The date and time the balance was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID the balance belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for CustomerBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomerBalance {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.created_at).into(),
            format!("{:?}", self.id).into(),
            format!("{:?}", self.monthly_credits_remaining).into(),
            format!("{:?}", self.pre_pay_cash_remaining).into(),
            format!("{:?}", self.pre_pay_credits_remaining).into(),
            format!("{:?}", self.total_due).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "id".into(),
            "monthly_credits_remaining".into(),
            "pre_pay_cash_remaining".into(),
            "pre_pay_credits_remaining".into(),
            "total_due".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The density response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Density {
    #[doc = "The density."]
    pub density: f64,
    #[doc = "The output unit for the density."]
    pub output_unit: UnitDensity,
}

impl std::fmt::Display for Density {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Density {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.density).into(),
            format!("{:?}", self.output_unit).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["density".into(), "output_unit".into()]
    }
}

#[doc = "The form for a device access token request."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeviceAccessTokenRequestForm {
    #[doc = "The client ID."]
    pub client_id: uuid::Uuid,
    #[doc = "The device code."]
    pub device_code: uuid::Uuid,
    #[doc = "The grant type."]
    pub grant_type: Oauth2GrantType,
}

impl std::fmt::Display for DeviceAccessTokenRequestForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeviceAccessTokenRequestForm {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.client_id).into(),
            format!("{:?}", self.device_code).into(),
            format!("{:?}", self.grant_type).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "client_id".into(),
            "device_code".into(),
            "grant_type".into(),
        ]
    }
}

#[doc = "The request parameters for the OAuth 2.0 Device Authorization Grant flow."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeviceAuthRequestForm {
    #[doc = "The client ID."]
    pub client_id: uuid::Uuid,
}

impl std::fmt::Display for DeviceAuthRequestForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeviceAuthRequestForm {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.client_id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["client_id".into()]
    }
}

#[doc = "The request parameters to verify the `user_code` for the OAuth 2.0 Device Authorization \
         Grant."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeviceAuthVerifyParams {
    #[doc = "The user code."]
    pub user_code: String,
}

impl std::fmt::Display for DeviceAuthVerifyParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeviceAuthVerifyParams {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.user_code.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user_code".into()]
    }
}

#[doc = "Specifies the sign of a co-ordinate axis."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Direction {
    #[doc = "Increasing numbers."]
    #[serde(rename = "positive")]
    #[display("positive")]
    Positive,
    #[doc = "Decreasing numbers."]
    #[serde(rename = "negative")]
    #[display("negative")]
    Negative,
}

#[doc = "The resource representing a Discount."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Discount {
    #[doc = "The coupon that applied to create this discount."]
    pub coupon: Coupon,
}

impl std::fmt::Display for Discount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Discount {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.coupon).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["coupon".into()]
    }
}

#[doc = "The body of the form for email authentication."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmailAuthenticationForm {
    #[doc = "The URL to redirect back to after we have authenticated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[doc = "The user's email."]
    pub email: String,
}

impl std::fmt::Display for EmailAuthenticationForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmailAuthenticationForm {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(callback_url) = &self.callback_url {
                format!("{:?}", callback_url).into()
            } else {
                String::new().into()
            },
            self.email.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["callback_url".into(), "email".into()]
    }
}

#[doc = "The response from the `EntityGetAllChildUuids` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EntityGetAllChildUuids {
    #[doc = "The UUIDs of the child entities."]
    pub entity_ids: Vec<uuid::Uuid>,
}

impl std::fmt::Display for EntityGetAllChildUuids {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EntityGetAllChildUuids {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.entity_ids).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_ids".into()]
    }
}

#[doc = "The response from the `EntityGetChildUuid` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EntityGetChildUuid {
    #[doc = "The UUID of the child entity."]
    pub entity_id: uuid::Uuid,
}

impl std::fmt::Display for EntityGetChildUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EntityGetChildUuid {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.entity_id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_id".into()]
    }
}

#[doc = "The response from the `EntityGetNumChildren` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EntityGetNumChildren {
    #[doc = "The number of children the entity has."]
    pub num: u32,
}

impl std::fmt::Display for EntityGetNumChildren {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EntityGetNumChildren {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.num).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["num".into()]
    }
}

#[doc = "The response from the `EntityGetParentId` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EntityGetParentId {
    #[doc = "The UUID of the parent entity."]
    pub entity_id: uuid::Uuid,
}

impl std::fmt::Display for EntityGetParentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EntityGetParentId {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.entity_id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_id".into()]
    }
}

#[doc = "The type of entity"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum EntityType {
    #[serde(rename = "entity")]
    #[display("entity")]
    Entity,
    #[serde(rename = "object")]
    #[display("object")]
    Object,
    #[serde(rename = "path")]
    #[display("path")]
    Path,
    #[serde(rename = "curve")]
    #[display("curve")]
    Curve,
    #[serde(rename = "solid2d")]
    #[display("solid2d")]
    Solid2D,
    #[serde(rename = "solid3d")]
    #[display("solid3d")]
    Solid3D,
    #[serde(rename = "edge")]
    #[display("edge")]
    Edge,
    #[serde(rename = "face")]
    #[display("face")]
    Face,
    #[serde(rename = "plane")]
    #[display("plane")]
    Plane,
}

#[doc = "The environment the server is running in."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Environment {
    #[doc = "The development environment. This is for running locally."]
    #[serde(rename = "DEVELOPMENT")]
    #[display("DEVELOPMENT")]
    Development,
    #[doc = "The preview environment. This is when PRs are created and a service is deployed for \
             testing."]
    #[serde(rename = "PREVIEW")]
    #[display("PREVIEW")]
    Preview,
    #[doc = "The production environment."]
    #[serde(rename = "PRODUCTION")]
    #[display("PRODUCTION")]
    Production,
}

#[doc = "Error information from a response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    pub message: String,
    pub request_id: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Error {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(error_code) = &self.error_code {
                format!("{:?}", error_code).into()
            } else {
                String::new().into()
            },
            self.message.clone().into(),
            self.request_id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["error_code".into(), "message".into(), "request_id".into()]
    }
}

#[doc = "The type of error sent by the KittyCAD API."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ErrorCode {
    #[doc = "Graphics engine failed to complete request, consider retrying"]
    #[serde(rename = "internal_engine")]
    #[display("internal_engine")]
    InternalEngine,
    #[doc = "API failed to complete request, consider retrying"]
    #[serde(rename = "internal_api")]
    #[display("internal_api")]
    InternalApi,
    #[doc = "User requested something geometrically or graphically impossible. Don't retry this \
             request, as it's inherently impossible. Instead, read the error message and change \
             your request."]
    #[serde(rename = "bad_request")]
    #[display("bad_request")]
    BadRequest,
    #[doc = "Client sent invalid JSON."]
    #[serde(rename = "invalid_json")]
    #[display("invalid_json")]
    InvalidJson,
    #[doc = "Client sent invalid BSON."]
    #[serde(rename = "invalid_bson")]
    #[display("invalid_bson")]
    InvalidBson,
    #[doc = "Client sent a message which is not accepted over this protocol."]
    #[serde(rename = "wrong_protocol")]
    #[display("wrong_protocol")]
    WrongProtocol,
    #[doc = "Problem sending data between client and KittyCAD API."]
    #[serde(rename = "connection_problem")]
    #[display("connection_problem")]
    ConnectionProblem,
    #[doc = "Client sent a Websocket message type which the KittyCAD API does not handle."]
    #[serde(rename = "message_type_not_accepted")]
    #[display("message_type_not_accepted")]
    MessageTypeNotAccepted,
    #[doc = "Client sent a Websocket message intended for WebRTC but it was configured as a \
             WebRTC connection."]
    #[serde(rename = "message_type_not_accepted_for_web_r_t_c")]
    #[display("message_type_not_accepted_for_web_r_t_c")]
    MessageTypeNotAcceptedForWebRTC,
}

#[doc = "The response from the `Export` endpoint."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Export {
    #[doc = "The files that were exported."]
    pub files: Vec<ExportFile>,
}

impl std::fmt::Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Export {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.files).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["files".into()]
    }
}

#[doc = "A file to be exported to the client."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ExportFile {
    #[doc = "The contents of the file, base64 encoded."]
    pub contents: base64::Base64Data,
    #[doc = "The name of the file."]
    pub name: String,
}

impl std::fmt::Display for ExportFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ExportFile {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.contents).into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["contents".into(), "name".into()]
    }
}

#[doc = "Extended user information.\n\nThis is mostly used for internal purposes. It returns a \
         mapping of the user's information, including that of our third party services we use for \
         users: MailChimp, Stripe, and Front"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ExtendedUser {
    #[doc = "The user's company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "The date and time the user was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user's Discord handle."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[doc = "The email address of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The date and time the email address was verified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The user's first name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The user's Front ID. This is mostly used for internal mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub front_id: Option<String>,
    #[doc = "The user's GitHub handle."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[doc = "The unique identifier for the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The image avatar for the user. This is a URL."]
    pub image: String,
    #[doc = "The user's last name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The user's MailChimp ID. This is mostly used for internal mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailchimp_id: Option<String>,
    #[doc = "The name of the user. This is auto populated at first from the authentication \
             provider (if there was a name). It can be updated by the user by updating their \
             `first_name` and `last_name` fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The user's phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: phone_number::PhoneNumber,
    #[doc = "The user's Stripe ID. This is mostly used for internal mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stripe_id: Option<String>,
    #[doc = "The date and time the user was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for ExtendedUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ExtendedUser {
    const LENGTH: usize = 16;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(company) = &self.company {
                format!("{:?}", company).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(discord) = &self.discord {
                format!("{:?}", discord).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(email_verified) = &self.email_verified {
                format!("{:?}", email_verified).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(front_id) = &self.front_id {
                format!("{:?}", front_id).into()
            } else {
                String::new().into()
            },
            if let Some(github) = &self.github {
                format!("{:?}", github).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            self.image.clone().into(),
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(mailchimp_id) = &self.mailchimp_id {
                format!("{:?}", mailchimp_id).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.phone).into(),
            if let Some(stripe_id) = &self.stripe_id {
                format!("{:?}", stripe_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.updated_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company".into(),
            "created_at".into(),
            "discord".into(),
            "email".into(),
            "email_verified".into(),
            "first_name".into(),
            "front_id".into(),
            "github".into(),
            "id".into(),
            "image".into(),
            "last_name".into(),
            "mailchimp_id".into(),
            "name".into(),
            "phone".into(),
            "stripe_id".into(),
            "updated_at".into(),
        ]
    }
}

#[doc = "A single page of results"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ExtendedUserResultsPage {
    #[doc = "list of items on this page of results"]
    pub items: Vec<ExtendedUser>,
    #[doc = "token used to fetch the next page of results (if any)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

impl std::fmt::Display for ExtendedUserResultsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ExtendedUserResultsPage {
    type Item = ExtendedUser;
    fn has_more_pages(&self) -> bool {
        self.next_page.is_some()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        req.url_mut()
            .query_pairs_mut()
            .append_pair("next_page", self.next_page.as_deref().unwrap_or(""));
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.items.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ExtendedUserResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.items).into(),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["items".into(), "next_page".into()]
    }
}

#[doc = "Unsuccessful Websocket response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FailureWebSocketResponse {
    #[doc = "The errors that occurred."]
    pub errors: Vec<ApiError>,
    #[doc = "Which request this is a response to. If the request was a modeling command, this is \
             the modeling command ID. If no request ID was sent, this will be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<uuid::Uuid>,
    #[doc = "Always false"]
    pub success: bool,
}

impl std::fmt::Display for FailureWebSocketResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FailureWebSocketResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.errors).into(),
            if let Some(request_id) = &self.request_id {
                format!("{:?}", request_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.success).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["errors".into(), "request_id".into(), "success".into()]
    }
}

#[doc = "Describes the storage format of an FBX file."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum FbxStorage {
    #[doc = "ASCII FBX encoding."]
    #[serde(rename = "ascii")]
    #[display("ascii")]
    Ascii,
    #[doc = "Binary FBX encoding."]
    #[serde(rename = "binary")]
    #[display("binary")]
    Binary,
}

#[doc = "A file center of mass result."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileCenterOfMass {
    #[doc = "The resulting center of mass."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center_of_mass: Option<Point3D>,
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The output unit for the center of mass."]
    pub output_unit: UnitLength,
    #[doc = "The source format of the file."]
    pub src_format: FileImportFormat,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for FileCenterOfMass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FileCenterOfMass {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(center_of_mass) = &self.center_of_mass {
                format!("{:?}", center_of_mass).into()
            } else {
                String::new().into()
            },
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            format!("{:?}", self.output_unit).into(),
            format!("{:?}", self.src_format).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "center_of_mass".into(),
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "output_unit".into(),
            "src_format".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "A file conversion."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The output format of the file conversion."]
    pub output_format: FileExportFormat,
    #[doc = "The output format options of the file conversion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_format_options: Option<OutputFormat>,
    #[doc = "The converted files (if multiple file conversion), if completed, base64 encoded. The \
             key of the map is the path of the output file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<std::collections::HashMap<String, base64::Base64Data>>,
    #[doc = "The source format of the file conversion."]
    pub src_format: FileImportFormat,
    #[doc = "The source format options of the file conversion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src_format_options: Option<InputFormat>,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for FileConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FileConversion {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            format!("{:?}", self.output_format).into(),
            if let Some(output_format_options) = &self.output_format_options {
                format!("{:?}", output_format_options).into()
            } else {
                String::new().into()
            },
            if let Some(outputs) = &self.outputs {
                format!("{:?}", outputs).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.src_format).into(),
            if let Some(src_format_options) = &self.src_format_options {
                format!("{:?}", src_format_options).into()
            } else {
                String::new().into()
            },
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "output_format".into(),
            "output_format_options".into(),
            "outputs".into(),
            "src_format".into(),
            "src_format_options".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "A file density result."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileDensity {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The resulting density."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The material mass as denoted by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_mass: Option<f64>,
    #[doc = "The material mass unit."]
    pub material_mass_unit: UnitMass,
    #[doc = "The output unit for the density."]
    pub output_unit: UnitDensity,
    #[doc = "The source format of the file."]
    pub src_format: FileImportFormat,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for FileDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FileDensity {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(density) = &self.density {
                format!("{:?}", density).into()
            } else {
                String::new().into()
            },
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(material_mass) = &self.material_mass {
                format!("{:?}", material_mass).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.material_mass_unit).into(),
            format!("{:?}", self.output_unit).into(),
            format!("{:?}", self.src_format).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "density".into(),
            "error".into(),
            "id".into(),
            "material_mass".into(),
            "material_mass_unit".into(),
            "output_unit".into(),
            "src_format".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of output file formats."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum FileExportFormat {
    #[doc = "Autodesk Filmbox (FBX) format. <https://en.wikipedia.org/wiki/FBX>"]
    #[serde(rename = "fbx")]
    #[display("fbx")]
    Fbx,
    #[doc = "Binary glTF 2.0.\n\nThis is a single binary with .glb extension.\n\nThis is better \
             if you want a compressed format as opposed to the human readable glTF that lacks \
             compression."]
    #[serde(rename = "glb")]
    #[display("glb")]
    Glb,
    #[doc = "glTF 2.0. Embedded glTF 2.0 (pretty printed).\n\nSingle JSON file with .gltf \
             extension binary data encoded as base64 data URIs.\n\nThe JSON contents are pretty \
             printed.\n\nIt is human readable, single file, and you can view the diff easily in a \
             git commit."]
    #[serde(rename = "gltf")]
    #[display("gltf")]
    Gltf,
    #[doc = "The OBJ file format. <https://en.wikipedia.org/wiki/Wavefront_.obj_file> It may or \
             may not have an an attached material (mtl // mtllib) within the file, but we \
             interact with it as if it does not."]
    #[serde(rename = "obj")]
    #[display("obj")]
    Obj,
    #[doc = "The PLY file format. <https://en.wikipedia.org/wiki/PLY_(file_format)>"]
    #[serde(rename = "ply")]
    #[display("ply")]
    Ply,
    #[doc = "The STEP file format. <https://en.wikipedia.org/wiki/ISO_10303-21>"]
    #[serde(rename = "step")]
    #[display("step")]
    Step,
    #[doc = "The STL file format. <https://en.wikipedia.org/wiki/STL_(file_format)>"]
    #[serde(rename = "stl")]
    #[display("stl")]
    Stl,
}

#[doc = "The valid types of source file formats."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum FileImportFormat {
    #[doc = "Autodesk Filmbox (FBX) format. <https://en.wikipedia.org/wiki/FBX>"]
    #[serde(rename = "fbx")]
    #[display("fbx")]
    Fbx,
    #[doc = "glTF 2.0."]
    #[serde(rename = "gltf")]
    #[display("gltf")]
    Gltf,
    #[doc = "The OBJ file format. <https://en.wikipedia.org/wiki/Wavefront_.obj_file> It may or \
             may not have an an attached material (mtl // mtllib) within the file, but we \
             interact with it as if it does not."]
    #[serde(rename = "obj")]
    #[display("obj")]
    Obj,
    #[doc = "The PLY file format. <https://en.wikipedia.org/wiki/PLY_(file_format)>"]
    #[serde(rename = "ply")]
    #[display("ply")]
    Ply,
    #[doc = "SolidWorks part (SLDPRT) format."]
    #[serde(rename = "sldprt")]
    #[display("sldprt")]
    Sldprt,
    #[doc = "The STEP file format. <https://en.wikipedia.org/wiki/ISO_10303-21>"]
    #[serde(rename = "step")]
    #[display("step")]
    Step,
    #[doc = "The STL file format. <https://en.wikipedia.org/wiki/STL_(file_format)>"]
    #[serde(rename = "stl")]
    #[display("stl")]
    Stl,
}

#[doc = "A file mass result."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileMass {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The resulting mass."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mass: Option<f64>,
    #[doc = "The material density as denoted by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_density: Option<f64>,
    #[doc = "The material density unit."]
    pub material_density_unit: UnitDensity,
    #[doc = "The output unit for the mass."]
    pub output_unit: UnitMass,
    #[doc = "The source format of the file."]
    pub src_format: FileImportFormat,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for FileMass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FileMass {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(mass) = &self.mass {
                format!("{:?}", mass).into()
            } else {
                String::new().into()
            },
            if let Some(material_density) = &self.material_density {
                format!("{:?}", material_density).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.material_density_unit).into(),
            format!("{:?}", self.output_unit).into(),
            format!("{:?}", self.src_format).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "mass".into(),
            "material_density".into(),
            "material_density_unit".into(),
            "output_unit".into(),
            "src_format".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "A file surface area result."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileSurfaceArea {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The output unit for the surface area."]
    pub output_unit: UnitArea,
    #[doc = "The source format of the file."]
    pub src_format: FileImportFormat,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The resulting surface area."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_area: Option<f64>,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for FileSurfaceArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FileSurfaceArea {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            format!("{:?}", self.output_unit).into(),
            format!("{:?}", self.src_format).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            if let Some(surface_area) = &self.surface_area {
                format!("{:?}", surface_area).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "output_unit".into(),
            "src_format".into(),
            "started_at".into(),
            "status".into(),
            "surface_area".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "Metadata about our file system.\n\nThis is mostly used for internal purposes and \
         debugging."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileSystemMetadata {
    #[doc = "If the file system passed a sanity check."]
    pub ok: bool,
}

impl std::fmt::Display for FileSystemMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FileSystemMetadata {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.ok).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["ok".into()]
    }
}

#[doc = "A file volume result."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileVolume {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The output unit for the volume."]
    pub output_unit: UnitVolume,
    #[doc = "The source format of the file."]
    pub src_format: FileImportFormat,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The resulting volume."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
}

impl std::fmt::Display for FileVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FileVolume {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            format!("{:?}", self.output_unit).into(),
            format!("{:?}", self.src_format).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(volume) = &self.volume {
                format!("{:?}", volume).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "output_unit".into(),
            "src_format".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
            "volume".into(),
        ]
    }
}

#[doc = "Gateway information."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Gateway {
    #[doc = "The auth timeout of the gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_timeout: Option<i64>,
    #[doc = "The host of the gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The name of the gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The port of the gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "The TLS timeout for the gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_timeout: Option<i64>,
}

impl std::fmt::Display for Gateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Gateway {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(auth_timeout) = &self.auth_timeout {
                format!("{:?}", auth_timeout).into()
            } else {
                String::new().into()
            },
            if let Some(host) = &self.host {
                format!("{:?}", host).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(port) = &self.port {
                format!("{:?}", port).into()
            } else {
                String::new().into()
            },
            if let Some(tls_timeout) = &self.tls_timeout {
                format!("{:?}", tls_timeout).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "auth_timeout".into(),
            "host".into(),
            "name".into(),
            "port".into(),
            "tls_timeout".into(),
        ]
    }
}

#[doc = "The response from the `GetEntityType` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetEntityType {
    #[doc = "The type of the entity."]
    pub entity_type: EntityType,
}

impl std::fmt::Display for GetEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetEntityType {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.entity_type).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_type".into()]
    }
}

#[doc = "Describes the presentation style of the glTF JSON."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum GltfPresentation {
    #[doc = "Condense the JSON into the smallest possible size."]
    #[serde(rename = "compact")]
    #[display("compact")]
    Compact,
    #[doc = "Expand the JSON into a more human readable format.\n\nThis is the default setting."]
    #[serde(rename = "pretty")]
    #[display("pretty")]
    Pretty,
}

#[doc = "Describes the storage format of a glTF 2.0 scene."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum GltfStorage {
    #[doc = "Binary glTF 2.0.\n\nThis is a single binary with .glb extension."]
    #[serde(rename = "binary")]
    #[display("binary")]
    Binary,
    #[doc = "Standard glTF 2.0.\n\nThis is a JSON file with .gltf extension paired with a \
             separate binary blob file with .bin extension."]
    #[serde(rename = "standard")]
    #[display("standard")]
    Standard,
    #[doc = "Embedded glTF 2.0.\n\nSingle JSON file with .gltf extension binary data encoded as \
             base64 data URIs.\n\nThis is the default setting."]
    #[serde(rename = "embedded")]
    #[display("embedded")]
    Embedded,
}

#[doc = "The response from the `HighlightSetEntity` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct HighlightSetEntity {
    #[doc = "The UUID of the entity that was highlighted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "If the client sent a sequence ID with its request, the backend sends it back."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u32>,
}

impl std::fmt::Display for HighlightSetEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for HighlightSetEntity {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            if let Some(sequence) = &self.sequence {
                format!("{:?}", sequence).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_id".into(), "sequence".into()]
    }
}

#[doc = "Representation of an ICE server used for STUN/TURN Used to initiate WebRTC connections based on <https://developer.mozilla.org/en-US/docs/Web/API/RTCIceServer>"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct IceServer {
    #[doc = "Credentials for a given TURN server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    #[doc = "URLs for a given STUN/TURN server. IceServer urls can either be a string or an array \
             of strings But, we choose to always convert to an array of strings for consistency"]
    pub urls: Vec<String>,
    #[doc = "Username for a given TURN server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl std::fmt::Display for IceServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for IceServer {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(credential) = &self.credential {
                format!("{:?}", credential).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.urls).into(),
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["credential".into(), "urls".into(), "username".into()]
    }
}

#[doc = "Enum containing the variety of image formats snapshots may be exported to."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ImageFormat {
    #[doc = ".png format"]
    #[serde(rename = "png")]
    #[display("png")]
    Png,
    #[doc = ".jpeg format"]
    #[serde(rename = "jpeg")]
    #[display("jpeg")]
    Jpeg,
}

#[doc = "An enumeration."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ImageType {
    #[serde(rename = "png")]
    #[display("png")]
    Png,
    #[serde(rename = "jpg")]
    #[display("jpg")]
    Jpg,
}

#[doc = "File to import into the current model"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ImportFile {
    #[doc = "The raw bytes of the file"]
    #[serde(
        serialize_with = "serde_bytes::serialize",
        deserialize_with = "serde_bytes::deserialize"
    )]
    pub data: Vec<u8>,
    #[doc = "The file's full path, including file extension."]
    pub path: String,
}

impl std::fmt::Display for ImportFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ImportFile {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.data).into(), self.path.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "path".into()]
    }
}

#[doc = "Data from importing the files"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ImportFiles {
    #[doc = "ID of the imported 3D models within the scene."]
    pub object_id: uuid::Uuid,
}

impl std::fmt::Display for ImportFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ImportFiles {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.object_id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["object_id".into()]
    }
}

#[doc = "Input format specifier."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type")]
pub enum InputFormat {
    #[serde(rename = "fbx")]
    Fbx {},
    #[serde(rename = "gltf")]
    Gltf {},
    #[serde(rename = "obj")]
    Obj {
        #[doc = "Co-ordinate system of input data.\n\nDefaults to the [KittyCAD co-ordinate \
                 system].\n\n[KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html"]
        coords: System,
        #[doc = "The units of the input data. This is very important for correct scaling and when \
                 calculating physics properties like mass, etc.\n\nDefaults to meters."]
        units: UnitLength,
    },
    #[serde(rename = "ply")]
    Ply {
        #[doc = "Co-ordinate system of input data.\n\nDefaults to the [KittyCAD co-ordinate \
                 system].\n\n[KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html"]
        coords: System,
        #[doc = "The units of the input data. This is very important for correct scaling and when \
                 calculating physics properties like mass, etc."]
        units: UnitLength,
    },
    #[serde(rename = "sldprt")]
    Sldprt {},
    #[serde(rename = "step")]
    Step {},
    #[serde(rename = "stl")]
    Stl {
        #[doc = "Co-ordinate system of input data.\n\nDefaults to the [KittyCAD co-ordinate \
                 system].\n\n[KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html"]
        coords: System,
        #[doc = "The units of the input data. This is very important for correct scaling and when \
                 calculating physics properties like mass, etc."]
        units: UnitLength,
    },
}

#[doc = "An invoice."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Invoice {
    #[doc = "Final amount due at this time for this invoice.\n\nIf the invoice's total is smaller \
             than the minimum charge amount, for example, or if there is account credit that can \
             be applied to the invoice, the `amount_due` may be 0. If there is a positive \
             `starting_balance` for the invoice (the customer owes money), the `amount_due` will \
             also take that into account. The charge that gets generated for the invoice will be \
             for the amount specified in `amount_due`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<bigdecimal::BigDecimal>,
    #[doc = "The amount, in USD, that was paid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<bigdecimal::BigDecimal>,
    #[doc = "The amount remaining, in USD, that is due."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_remaining: Option<bigdecimal::BigDecimal>,
    #[doc = "Number of payment attempts made for this invoice, from the perspective of the \
             payment retry schedule.\n\nAny payment attempt counts as the first attempt, and \
             subsequently only automatic retries increment the attempt count. In other words, \
             manual payment attempts after the first attempt do not affect the retry schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt_count: Option<u64>,
    #[doc = "Whether an attempt has been made to pay the invoice.\n\nAn invoice is not attempted \
             until 1 hour after the `invoice.created` webhook, for example, so you might not want \
             to display that invoice as unpaid to your users."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempted: Option<bool>,
    #[doc = "Time at which the object was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), \
             in lowercase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The email address for the customer. Until the invoice is finalized, this field will \
             equal customer.email. Once the invoice is finalized, this field will no longer be \
             updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[doc = "Customer ID. The unique identifier for the customer this invoice belongs to. This is \
             the customer ID in the payments service, not our database customer ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "Default payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    #[doc = "Description of the invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The discounts applied to the invoice. This is an array of discount objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<Discount>>,
    #[doc = "Unique identifier for the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The individual line items that make up the invoice.\n\n`lines` is sorted as follows: \
             invoice items in reverse chronological order, followed by the subscription, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<InvoiceLineItem>>,
    #[doc = "Set of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A unique, identifying string that appears on emails sent to the customer for this \
             invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[doc = "Whether payment was successfully collected for this invoice.\n\nAn invoice can be \
             paid (most commonly) with a charge or with credit from the customer's account \
             balance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
    #[doc = "The link to download the PDF for the invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdf: Option<String>,
    #[doc = "This is the transaction number that appears on email receipts sent for this invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[doc = "Extra information about an invoice for the customer's credit card statement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.\n\n[Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,
    #[doc = "Total of all subscriptions, invoice items, and prorations on the invoice before any \
             invoice level discount or tax is applied.\n\nItem discounts are already incorporated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<bigdecimal::BigDecimal>,
    #[doc = "The amount of tax on this invoice.\n\nThis is the sum of all the tax amounts on this \
             invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<bigdecimal::BigDecimal>,
    #[doc = "Total after discounts and taxes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<bigdecimal::BigDecimal>,
    #[doc = "The URL for the hosted invoice page, which allows customers to view and pay an \
             invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl std::fmt::Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Invoice {
    const LENGTH: usize = 25;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(amount_due) = &self.amount_due {
                format!("{:?}", amount_due).into()
            } else {
                String::new().into()
            },
            if let Some(amount_paid) = &self.amount_paid {
                format!("{:?}", amount_paid).into()
            } else {
                String::new().into()
            },
            if let Some(amount_remaining) = &self.amount_remaining {
                format!("{:?}", amount_remaining).into()
            } else {
                String::new().into()
            },
            if let Some(attempt_count) = &self.attempt_count {
                format!("{:?}", attempt_count).into()
            } else {
                String::new().into()
            },
            if let Some(attempted) = &self.attempted {
                format!("{:?}", attempted).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            if let Some(customer_email) = &self.customer_email {
                format!("{:?}", customer_email).into()
            } else {
                String::new().into()
            },
            if let Some(customer_id) = &self.customer_id {
                format!("{:?}", customer_id).into()
            } else {
                String::new().into()
            },
            if let Some(default_payment_method) = &self.default_payment_method {
                format!("{:?}", default_payment_method).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(discounts) = &self.discounts {
                format!("{:?}", discounts).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(lines) = &self.lines {
                format!("{:?}", lines).into()
            } else {
                String::new().into()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata).into()
            } else {
                String::new().into()
            },
            if let Some(number) = &self.number {
                format!("{:?}", number).into()
            } else {
                String::new().into()
            },
            if let Some(paid) = &self.paid {
                format!("{:?}", paid).into()
            } else {
                String::new().into()
            },
            if let Some(pdf) = &self.pdf {
                format!("{:?}", pdf).into()
            } else {
                String::new().into()
            },
            if let Some(receipt_number) = &self.receipt_number {
                format!("{:?}", receipt_number).into()
            } else {
                String::new().into()
            },
            if let Some(statement_descriptor) = &self.statement_descriptor {
                format!("{:?}", statement_descriptor).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(subtotal) = &self.subtotal {
                format!("{:?}", subtotal).into()
            } else {
                String::new().into()
            },
            if let Some(tax) = &self.tax {
                format!("{:?}", tax).into()
            } else {
                String::new().into()
            },
            if let Some(total) = &self.total {
                format!("{:?}", total).into()
            } else {
                String::new().into()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount_due".into(),
            "amount_paid".into(),
            "amount_remaining".into(),
            "attempt_count".into(),
            "attempted".into(),
            "created_at".into(),
            "currency".into(),
            "customer_email".into(),
            "customer_id".into(),
            "default_payment_method".into(),
            "description".into(),
            "discounts".into(),
            "id".into(),
            "lines".into(),
            "metadata".into(),
            "number".into(),
            "paid".into(),
            "pdf".into(),
            "receipt_number".into(),
            "statement_descriptor".into(),
            "status".into(),
            "subtotal".into(),
            "tax".into(),
            "total".into(),
            "url".into(),
        ]
    }
}

#[doc = "An invoice line item."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InvoiceLineItem {
    #[doc = "The amount, in USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<bigdecimal::BigDecimal>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), \
             in lowercase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ID of the invoice item associated with this line item if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    #[doc = "Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach \
             to an object.\n\nSet of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl std::fmt::Display for InvoiceLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InvoiceLineItem {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(invoice_item) = &self.invoice_item {
                format!("{:?}", invoice_item).into()
            } else {
                String::new().into()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount".into(),
            "currency".into(),
            "description".into(),
            "id".into(),
            "invoice_item".into(),
            "metadata".into(),
        ]
    }
}

#[doc = "An enum representing the possible values of an `Invoice`'s `status` field."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum InvoiceStatus {
    #[doc = "Deleted."]
    #[serde(rename = "deleted")]
    #[display("deleted")]
    Deleted,
    #[doc = "Draft."]
    #[serde(rename = "draft")]
    #[display("draft")]
    Draft,
    #[doc = "Open."]
    #[serde(rename = "open")]
    #[display("open")]
    Open,
    #[doc = "Paid."]
    #[serde(rename = "paid")]
    #[display("paid")]
    Paid,
    #[doc = "Uncollectible."]
    #[serde(rename = "uncollectible")]
    #[display("uncollectible")]
    Uncollectible,
    #[doc = "Void."]
    #[serde(rename = "void")]
    #[display("void")]
    Void,
}

#[doc = "Jetstream information."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Jetstream {
    #[doc = "The Jetstream config."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<JetstreamConfig>,
    #[doc = "Meta information about the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<MetaClusterInfo>,
    #[doc = "Jetstream statistics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<JetstreamStats>,
}

impl std::fmt::Display for Jetstream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Jetstream {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(config) = &self.config {
                format!("{:?}", config).into()
            } else {
                String::new().into()
            },
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            if let Some(stats) = &self.stats {
                format!("{:?}", stats).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["config".into(), "meta".into(), "stats".into()]
    }
}

#[doc = "Jetstream API statistics."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JetstreamApiStats {
    #[doc = "The number of errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<i64>,
    #[doc = "The number of inflight requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inflight: Option<i64>,
    #[doc = "The number of requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl std::fmt::Display for JetstreamApiStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JetstreamApiStats {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(errors) = &self.errors {
                format!("{:?}", errors).into()
            } else {
                String::new().into()
            },
            if let Some(inflight) = &self.inflight {
                format!("{:?}", inflight).into()
            } else {
                String::new().into()
            },
            if let Some(total) = &self.total {
                format!("{:?}", total).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["errors".into(), "inflight".into(), "total".into()]
    }
}

#[doc = "Jetstream configuration."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JetstreamConfig {
    #[doc = "The domain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The max memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_memory: Option<i64>,
    #[doc = "The max storage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_storage: Option<i64>,
    #[doc = "The store directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_dir: Option<String>,
}

impl std::fmt::Display for JetstreamConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JetstreamConfig {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(domain) = &self.domain {
                format!("{:?}", domain).into()
            } else {
                String::new().into()
            },
            if let Some(max_memory) = &self.max_memory {
                format!("{:?}", max_memory).into()
            } else {
                String::new().into()
            },
            if let Some(max_storage) = &self.max_storage {
                format!("{:?}", max_storage).into()
            } else {
                String::new().into()
            },
            if let Some(store_dir) = &self.store_dir {
                format!("{:?}", store_dir).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "domain".into(),
            "max_memory".into(),
            "max_storage".into(),
            "store_dir".into(),
        ]
    }
}

#[doc = "Jetstream statistics."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JetstreamStats {
    #[doc = "The number of accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<i64>,
    #[doc = "API stats."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<JetstreamApiStats>,
    #[doc = "The number of HA assets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ha_assets: Option<i64>,
    #[doc = "The memory used by the Jetstream server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    #[doc = "The reserved memory for the Jetstream server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved_memory: Option<i64>,
    #[doc = "The reserved storage for the Jetstream server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved_store: Option<i64>,
    #[doc = "The storage used by the Jetstream server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<i64>,
}

impl std::fmt::Display for JetstreamStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JetstreamStats {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounts) = &self.accounts {
                format!("{:?}", accounts).into()
            } else {
                String::new().into()
            },
            if let Some(api) = &self.api {
                format!("{:?}", api).into()
            } else {
                String::new().into()
            },
            if let Some(ha_assets) = &self.ha_assets {
                format!("{:?}", ha_assets).into()
            } else {
                String::new().into()
            },
            if let Some(memory) = &self.memory {
                format!("{:?}", memory).into()
            } else {
                String::new().into()
            },
            if let Some(reserved_memory) = &self.reserved_memory {
                format!("{:?}", reserved_memory).into()
            } else {
                String::new().into()
            },
            if let Some(reserved_store) = &self.reserved_store {
                format!("{:?}", reserved_store).into()
            } else {
                String::new().into()
            },
            if let Some(store) = &self.store {
                format!("{:?}", store).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounts".into(),
            "api".into(),
            "ha_assets".into(),
            "memory".into(),
            "reserved_memory".into(),
            "reserved_store".into(),
            "store".into(),
        ]
    }
}

#[doc = "Leaf node information."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LeafNode {
    #[doc = "The auth timeout of the leaf node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_timeout: Option<i64>,
    #[doc = "The host of the leaf node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The port of the leaf node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "The TLS timeout for the leaf node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_timeout: Option<i64>,
}

impl std::fmt::Display for LeafNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LeafNode {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(auth_timeout) = &self.auth_timeout {
                format!("{:?}", auth_timeout).into()
            } else {
                String::new().into()
            },
            if let Some(host) = &self.host {
                format!("{:?}", host).into()
            } else {
                String::new().into()
            },
            if let Some(port) = &self.port {
                format!("{:?}", port).into()
            } else {
                String::new().into()
            },
            if let Some(tls_timeout) = &self.tls_timeout {
                format!("{:?}", tls_timeout).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "auth_timeout".into(),
            "host".into(),
            "port".into(),
            "tls_timeout".into(),
        ]
    }
}

#[doc = "The mass response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Mass {
    #[doc = "The mass."]
    pub mass: f64,
    #[doc = "The output unit for the mass."]
    pub output_unit: UnitMass,
}

impl std::fmt::Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Mass {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.mass).into(),
            format!("{:?}", self.output_unit).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["mass".into(), "output_unit".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Mesh {
    pub mesh: String,
}

impl std::fmt::Display for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Mesh {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.mesh.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["mesh".into()]
    }
}

#[doc = "Jetstream statistics."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MetaClusterInfo {
    #[doc = "The size of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<i64>,
    #[doc = "The leader of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader: Option<String>,
    #[doc = "The name of the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for MetaClusterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MetaClusterInfo {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(cluster_size) = &self.cluster_size {
                format!("{:?}", cluster_size).into()
            } else {
                String::new().into()
            },
            if let Some(leader) = &self.leader {
                format!("{:?}", leader).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["cluster_size".into(), "leader".into(), "name".into()]
    }
}

#[doc = "Metadata about our currently running server.\n\nThis is mostly used for internal purposes \
         and debugging."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Metadata {
    #[doc = "Metadata about our cache."]
    pub cache: CacheMetadata,
    #[doc = "The environment we are running in."]
    pub environment: Environment,
    #[doc = "Metadata about our file system."]
    pub fs: FileSystemMetadata,
    #[doc = "The git hash of the server."]
    pub git_hash: String,
    #[doc = "Metadata about our point-e instance."]
    pub point_e: PointEMetadata,
    #[doc = "Metadata about our pub-sub connection."]
    pub pubsub: Connection,
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Metadata {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.cache).into(),
            format!("{:?}", self.environment).into(),
            format!("{:?}", self.fs).into(),
            self.git_hash.clone().into(),
            format!("{:?}", self.point_e).into(),
            format!("{:?}", self.pubsub).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "cache".into(),
            "environment".into(),
            "fs".into(),
            "git_hash".into(),
            "point_e".into(),
            "pubsub".into(),
        ]
    }
}

#[doc = "The Request Method (VERB)\n\nThis type also contains constants for a number of common HTTP methods such as GET, POST, etc.\n\nCurrently includes 8 variants representing the 8 methods defined in [RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH, and an Extension variant for all extensions."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Method {
    #[doc = "The `OPTIONS` method as defined in [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4.2.1)."]
    #[serde(rename = "OPTIONS")]
    #[display("OPTIONS")]
    Options,
    #[doc = "The `GET` method as defined in [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4.3.1)."]
    #[serde(rename = "GET")]
    #[display("GET")]
    Get,
    #[doc = "The `POST` method as defined in [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4.3.1)."]
    #[serde(rename = "POST")]
    #[display("POST")]
    Post,
    #[doc = "The `PUT` method as defined in [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4.3.1)."]
    #[serde(rename = "PUT")]
    #[display("PUT")]
    Put,
    #[doc = "The `DELETE` method as defined in [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4.3.5)."]
    #[serde(rename = "DELETE")]
    #[display("DELETE")]
    Delete,
    #[doc = "The `HEAD` method as defined in [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4.3.2)."]
    #[serde(rename = "HEAD")]
    #[display("HEAD")]
    Head,
    #[doc = "The `TRACE` method as defined in [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4.3)."]
    #[serde(rename = "TRACE")]
    #[display("TRACE")]
    Trace,
    #[doc = "The `CONNECT` method as defined in [RFC 7231](https://tools.ietf.org/html/rfc7231#section-4.3.6)."]
    #[serde(rename = "CONNECT")]
    #[display("CONNECT")]
    Connect,
    #[doc = "The `PATCH` method as defined in [RFC 5789](https://tools.ietf.org/html/rfc5789)."]
    #[serde(rename = "PATCH")]
    #[display("PATCH")]
    Patch,
    #[doc = "A catch all."]
    #[serde(rename = "EXTENSION")]
    #[display("EXTENSION")]
    Extension,
}

#[doc = "Commands that the KittyCAD engine can execute."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type")]
pub enum ModelingCmd {
    #[serde(rename = "start_path")]
    StartPath {},
    #[serde(rename = "move_path_pen")]
    MovePathPen {
        #[doc = "The ID of the command which created the path."]
        path: uuid::Uuid,
        #[doc = "Where the path's pen should be."]
        to: Point3D,
    },
    #[serde(rename = "extend_path")]
    ExtendPath {
        #[doc = "The ID of the command which created the path."]
        path: uuid::Uuid,
        #[doc = "Segment to append to the path. This segment will implicitly begin at the current \
                 \"pen\" location."]
        segment: PathSegment,
    },
    #[serde(rename = "extrude")]
    Extrude {
        #[doc = "Whether to cap the extrusion with a face, or not. If true, the resulting solid \
                 will be closed on all sides, like a dice. If false, it will be open on one side, \
                 like a drinking glass."]
        cap: bool,
        #[doc = "How far off the plane to extrude"]
        distance: f64,
        #[doc = "Which sketch to extrude. Must be a closed 2D solid."]
        target: uuid::Uuid,
    },
    #[serde(rename = "close_path")]
    ClosePath {
        #[doc = "Which path to close."]
        path_id: uuid::Uuid,
    },
    #[serde(rename = "camera_drag_start")]
    CameraDragStart {
        #[doc = "The type of camera drag interaction."]
        interaction: CameraDragInteractionType,
        #[doc = "The initial mouse position."]
        window: Point2D,
    },
    #[serde(rename = "camera_drag_move")]
    CameraDragMove {
        #[doc = "The type of camera drag interaction."]
        interaction: CameraDragInteractionType,
        #[doc = "Logical timestamp. The client should increment this with every event in the \
                 current mouse drag. That way, if the events are being sent over an unordered \
                 channel, the API can ignore the older events."]
        sequence: Option<u32>,
        #[doc = "The current mouse position."]
        window: Point2D,
    },
    #[serde(rename = "camera_drag_end")]
    CameraDragEnd {
        #[doc = "The type of camera drag interaction."]
        interaction: CameraDragInteractionType,
        #[doc = "The final mouse position."]
        window: Point2D,
    },
    #[serde(rename = "default_camera_look_at")]
    DefaultCameraLookAt {
        #[doc = "What the camera is looking at. Center of the camera's field of vision"]
        center: Point3D,
        #[doc = "Which way is \"up\", from the camera's point of view."]
        up: Point3D,
        #[doc = "Where the camera is positioned"]
        vantage: Point3D,
    },
    #[serde(rename = "default_camera_zoom")]
    DefaultCameraZoom {
        #[doc = "Move the camera forward along the vector it's looking at, by this \
                 magnitudedefaultCameraZoom. Basically, how much should the camera move forward \
                 by."]
        magnitude: f64,
    },
    #[serde(rename = "default_camera_enable_sketch_mode")]
    DefaultCameraEnableSketchMode {
        #[doc = "Should we animate or snap for the camera transition?"]
        animated: bool,
        #[doc = "How far to the sketching plane?"]
        distance_to_plane: f64,
        #[doc = "What's the origin of the sketching plane?"]
        origin: Point3D,
        #[doc = "Should the camera use orthographic projection? In other words, should an \
                 object's size in the rendered image stay constant regardless of its distance \
                 from the camera."]
        ortho: bool,
        #[doc = "Which 3D axis of the scene should be the X axis of the sketching plane?"]
        x_axis: Point3D,
        #[doc = "Which 3D axis of the scene should be the Y axis of the sketching plane?"]
        y_axis: Point3D,
    },
    #[serde(rename = "default_camera_disable_sketch_mode")]
    DefaultCameraDisableSketchMode {},
    #[serde(rename = "export")]
    Export {
        #[doc = "IDs of the entities to be exported. If this is empty, then all entities are \
                 exported."]
        entity_ids: Vec<uuid::Uuid>,
        #[doc = "The file format to export to."]
        format: OutputFormat,
    },
    #[serde(rename = "entity_get_parent_id")]
    EntityGetParentId {
        #[doc = "ID of the entity being queried."]
        entity_id: uuid::Uuid,
    },
    #[serde(rename = "entity_get_num_children")]
    EntityGetNumChildren {
        #[doc = "ID of the entity being queried."]
        entity_id: uuid::Uuid,
    },
    #[serde(rename = "entity_get_child_uuid")]
    EntityGetChildUuid {
        #[doc = "Index into the entity's list of children."]
        child_index: u32,
        #[doc = "ID of the entity being queried."]
        entity_id: uuid::Uuid,
    },
    #[serde(rename = "entity_get_all_child_uuids")]
    EntityGetAllChildUuids {
        #[doc = "ID of the entity being queried."]
        entity_id: uuid::Uuid,
    },
    #[serde(rename = "edit_mode_enter")]
    EditModeEnter {
        #[doc = "The edit target"]
        target: uuid::Uuid,
    },
    #[serde(rename = "edit_mode_exit")]
    EditModeExit {},
    #[serde(rename = "select_with_point")]
    SelectWithPoint {
        #[doc = "Where in the window was selected"]
        selected_at_window: Point2D,
        #[doc = "What entity was selected?"]
        selection_type: SceneSelectionType,
    },
    #[serde(rename = "select_clear")]
    SelectClear {},
    #[serde(rename = "select_add")]
    SelectAdd {
        #[doc = "Which entities to select"]
        entities: Vec<uuid::Uuid>,
    },
    #[serde(rename = "select_remove")]
    SelectRemove {
        #[doc = "Which entities to unselect"]
        entities: Vec<uuid::Uuid>,
    },
    #[serde(rename = "select_replace")]
    SelectReplace {
        #[doc = "Which entities to select"]
        entities: Vec<uuid::Uuid>,
    },
    #[serde(rename = "select_get")]
    SelectGet {},
    #[serde(rename = "highlight_set_entity")]
    HighlightSetEntity {
        #[doc = "Coordinates of the window being clicked"]
        selected_at_window: Point2D,
        #[doc = "Logical timestamp. The client should increment this with every event in the \
                 current mouse drag. That way, if the events are being sent over an unordered \
                 channel, the API can ignore the older events."]
        sequence: Option<u32>,
    },
    #[serde(rename = "highlight_set_entities")]
    HighlightSetEntities {
        #[doc = "Highlight these entities."]
        entities: Vec<uuid::Uuid>,
    },
    #[serde(rename = "new_annotation")]
    NewAnnotation {
        #[doc = "What type of annotation to create."]
        annotation_type: AnnotationType,
        #[doc = "If true, any existing drawables within the obj will be replaced (the object will \
                 be reset)"]
        clobber: bool,
        #[doc = "What should the annotation contain?"]
        options: AnnotationOptions,
    },
    #[serde(rename = "update_annotation")]
    UpdateAnnotation {
        #[doc = "Which annotation to update"]
        annotation_id: uuid::Uuid,
        #[doc = "If any of these fields are set, they will overwrite the previous options for the \
                 annotation."]
        options: AnnotationOptions,
    },
    #[serde(rename = "object_visible")]
    ObjectVisible {
        #[doc = "Whether or not the object should be hidden."]
        hidden: bool,
        #[doc = "Which object to change"]
        object_id: uuid::Uuid,
    },
    #[serde(rename = "get_entity_type")]
    GetEntityType {
        #[doc = "ID of the entity being queried."]
        entity_id: uuid::Uuid,
    },
    #[serde(rename = "solid3d_get_all_edge_faces")]
    Solid3DGetAllEdgeFaces {
        #[doc = "Which edge you want the faces of."]
        edge_id: uuid::Uuid,
        #[doc = "Which object is being queried."]
        object_id: uuid::Uuid,
    },
    #[serde(rename = "solid3d_get_all_opposite_edges")]
    Solid3DGetAllOppositeEdges {
        #[doc = "If given, ohnly faces parallel to this vector will be considered."]
        along_vector: Option<Point3D>,
        #[doc = "Which edge you want the opposites of."]
        edge_id: uuid::Uuid,
        #[doc = "Which object is being queried."]
        object_id: uuid::Uuid,
    },
    #[serde(rename = "solid3d_get_opposite_edge")]
    Solid3DGetOppositeEdge {
        #[doc = "Which edge you want the opposite of."]
        edge_id: uuid::Uuid,
        #[doc = "Which face is used to figure out the opposite edge?"]
        face_id: uuid::Uuid,
        #[doc = "Which object is being queried."]
        object_id: uuid::Uuid,
    },
    #[serde(rename = "solid3d_get_next_adjacent_edge")]
    Solid3DGetNextAdjacentEdge {
        #[doc = "Which edge you want the opposite of."]
        edge_id: uuid::Uuid,
        #[doc = "Which face is used to figure out the opposite edge?"]
        face_id: uuid::Uuid,
        #[doc = "Which object is being queried."]
        object_id: uuid::Uuid,
    },
    #[serde(rename = "solid3d_get_prev_adjacent_edge")]
    Solid3DGetPrevAdjacentEdge {
        #[doc = "Which edge you want the opposite of."]
        edge_id: uuid::Uuid,
        #[doc = "Which face is used to figure out the opposite edge?"]
        face_id: uuid::Uuid,
        #[doc = "Which object is being queried."]
        object_id: uuid::Uuid,
    },
    #[serde(rename = "send_object")]
    SendObject {
        #[doc = "Bring to front = true, send to back = false."]
        front: bool,
        #[doc = "Which object is being changed."]
        object_id: uuid::Uuid,
    },
    #[serde(rename = "entity_set_opacity")]
    EntitySetOpacity {
        #[doc = "Which entity is being changed."]
        entity_id: uuid::Uuid,
        #[doc = "How transparent should it be? 0 or lower is totally transparent. 1 or greater is \
                 totally opaque."]
        opacity: f64,
    },
    #[serde(rename = "entity_fade")]
    EntityFade {
        #[doc = "How many seconds the animation should take."]
        duration_seconds: Option<f64>,
        #[doc = "Which entity is being changed."]
        entity_id: uuid::Uuid,
        #[doc = "Fade in = true, fade out = false."]
        fade_in: bool,
    },
    #[serde(rename = "make_plane")]
    MakePlane {
        #[doc = "If true, any existing drawables within the obj will be replaced (the object will \
                 be reset)"]
        clobber: bool,
        #[doc = "Origin of the plane"]
        origin: Point3D,
        #[doc = "What should the plane's span/extent? When rendered visually, this is both the \
                 width and height along X and Y axis respectively."]
        size: f64,
        #[doc = "What should the plane's X axis be?"]
        x_axis: Point3D,
        #[doc = "What should the plane's Y axis be?"]
        y_axis: Point3D,
    },
    #[serde(rename = "plane_set_color")]
    PlaneSetColor {
        #[doc = "What color it should be."]
        color: Color,
        #[doc = "Which plane is being changed."]
        plane_id: uuid::Uuid,
    },
    #[serde(rename = "set_tool")]
    SetTool {
        #[doc = "What tool should be active."]
        tool: SceneToolType,
    },
    #[serde(rename = "mouse_move")]
    MouseMove {
        #[doc = "Logical timestamp. The client should increment this with every event in the \
                 current mouse drag. That way, if the events are being sent over an unordered \
                 channel, the API can ignore the older events."]
        sequence: Option<u32>,
        #[doc = "Where the mouse is"]
        window: Point2D,
    },
    #[serde(rename = "mouse_click")]
    MouseClick {
        #[doc = "Where the mouse is"]
        window: Point2D,
    },
    #[serde(rename = "sketch_mode_enable")]
    SketchModeEnable {
        #[doc = "Animate the transition to sketch mode."]
        animated: bool,
        #[doc = "Use an orthographic camera."]
        ortho: bool,
        #[doc = "Sketch on this plane."]
        plane_id: uuid::Uuid,
    },
    #[serde(rename = "sketch_mode_disable")]
    SketchModeDisable {},
    #[serde(rename = "curve_get_type")]
    CurveGetType {
        #[doc = "Which curve to query."]
        curve_id: uuid::Uuid,
    },
    #[serde(rename = "curve_get_control_points")]
    CurveGetControlPoints {
        #[doc = "Which curve to query."]
        curve_id: uuid::Uuid,
    },
    #[serde(rename = "take_snapshot")]
    TakeSnapshot {
        #[doc = "What image format to return."]
        format: ImageFormat,
    },
    #[serde(rename = "make_axes_gizmo")]
    MakeAxesGizmo {
        #[doc = "If true, any existing drawables within the obj will be replaced (the object will \
                 be reset)"]
        clobber: bool,
        #[doc = "If true, axes gizmo will be placed in the corner of the screen. If false, it \
                 will be placed at the origin of the scene."]
        gizmo_mode: bool,
    },
    #[serde(rename = "path_get_info")]
    PathGetInfo {
        #[doc = "Which path to query"]
        path_id: uuid::Uuid,
    },
    #[serde(rename = "path_get_curve_uuids_for_vertices")]
    PathGetCurveUuidsForVertices {
        #[doc = "Which path to query"]
        path_id: uuid::Uuid,
        #[doc = "IDs of the vertices for which to obtain curve ids from"]
        vertex_ids: Vec<uuid::Uuid>,
    },
    #[serde(rename = "handle_mouse_drag_start")]
    HandleMouseDragStart {
        #[doc = "The mouse position."]
        window: Point2D,
    },
    #[serde(rename = "handle_mouse_drag_move")]
    HandleMouseDragMove {
        #[doc = "Logical timestamp. The client should increment this with every event in the \
                 current mouse drag. That way, if the events are being sent over an unordered \
                 channel, the API can ignore the older events."]
        sequence: Option<u32>,
        #[doc = "The mouse position."]
        window: Point2D,
    },
    #[serde(rename = "handle_mouse_drag_end")]
    HandleMouseDragEnd {
        #[doc = "The mouse position."]
        window: Point2D,
    },
    #[serde(rename = "remove_scene_objects")]
    RemoveSceneObjects {
        #[doc = "Objects to remove."]
        object_ids: Vec<uuid::Uuid>,
    },
    #[serde(rename = "path_tangential_arc_to")]
    PathTangentialArcTo {
        #[doc = "0 will be interpreted as none/null."]
        angle_snap_increment: Option<Angle>,
        #[doc = "Where the arc should end. Must lie in the same plane as the current path pen \
                 position. Must not be colinear with current path pen position."]
        to: Point3D,
    },
    #[serde(rename = "path_tangential_arc")]
    PathTangentialArc {
        #[doc = "Offset of the arc."]
        offset: Angle,
        #[doc = "Radius of the arc. Not to be confused with Raiders of the Lost Ark."]
        radius: f64,
    },
    #[serde(rename = "plane_intersect_and_project")]
    PlaneIntersectAndProject {
        #[doc = "The plane you're intersecting against."]
        plane_id: uuid::Uuid,
        #[doc = "Window coordinates where the ray cast should be aimed."]
        window: Point2D,
    },
    #[serde(rename = "curve_get_end_points")]
    CurveGetEndPoints {
        #[doc = "ID of the curve being queried."]
        curve_id: uuid::Uuid,
    },
    #[serde(rename = "reconfigure_stream")]
    ReconfigureStream {
        #[doc = "Frames per second."]
        fps: u32,
        #[doc = "Height of the stream."]
        height: u32,
        #[doc = "Width of the stream."]
        width: u32,
    },
    #[serde(rename = "import_files")]
    ImportFiles {
        #[doc = "Files to import"]
        files: Vec<ImportFile>,
    },
    #[serde(rename = "mass")]
    Mass {
        #[doc = "IDs of the entities to get the mass of. If this is empty, then the default scene \
                 is included in the mass."]
        entity_ids: Vec<uuid::Uuid>,
        #[doc = "The material density."]
        material_density: f64,
        #[doc = "The material density unit."]
        material_density_unit: UnitDensity,
        #[doc = "The output unit for the mass."]
        output_unit: UnitMass,
    },
    #[serde(rename = "density")]
    Density {
        #[doc = "IDs of the entities to get the density of. If this is empty, then the default \
                 scene is included in the density."]
        entity_ids: Vec<uuid::Uuid>,
        #[doc = "The material mass."]
        material_mass: f64,
        #[doc = "The material mass unit."]
        material_mass_unit: UnitMass,
        #[doc = "The output unit for the density."]
        output_unit: UnitDensity,
    },
    #[serde(rename = "volume")]
    Volume {
        #[doc = "IDs of the entities to get the volume of. If this is empty, then the default \
                 scene is included in the volume."]
        entity_ids: Vec<uuid::Uuid>,
        #[doc = "The output unit for the volume."]
        output_unit: UnitVolume,
    },
    #[serde(rename = "center_of_mass")]
    CenterOfMass {
        #[doc = "IDs of the entities to get the center of mass of. If this is empty, then the \
                 default scene is included in the center of mass."]
        entity_ids: Vec<uuid::Uuid>,
        #[doc = "The output unit for the center of mass."]
        output_unit: UnitLength,
    },
    #[serde(rename = "surface_area")]
    SurfaceArea {
        #[doc = "IDs of the entities to get the surface area of. If this is empty, then the \
                 default scene is included in the surface area."]
        entity_ids: Vec<uuid::Uuid>,
        #[doc = "The output unit for the surface area."]
        output_unit: UnitArea,
    },
}

#[doc = "The response from the `MouseClick` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MouseClick {
    #[doc = "Entities that are modified."]
    pub entities_modified: Vec<uuid::Uuid>,
    #[doc = "Entities that are selected."]
    pub entities_selected: Vec<uuid::Uuid>,
}

impl std::fmt::Display for MouseClick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MouseClick {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.entities_modified).into(),
            format!("{:?}", self.entities_selected).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entities_modified".into(), "entities_selected".into()]
    }
}

#[doc = "The struct that is used to create a new record. This is automatically generated and has \
         all the same fields as the main struct only it is missing the `id`."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct NewAddress {
    #[doc = "The city component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The country component. This is a two-letter ISO country code."]
    pub country: String,
    #[doc = "The state component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The first street component."]
    #[serde(rename = "street1", default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[doc = "The second street component."]
    #[serde(rename = "street2", default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[doc = "The user ID that this address belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The zip component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl std::fmt::Display for NewAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for NewAddress {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            self.country.clone().into(),
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "city".into(),
            "country".into(),
            "state".into(),
            "street_1".into(),
            "street_2".into(),
            "user_id".into(),
            "zip".into(),
        ]
    }
}

#[doc = "Information about an OAuth 2.0 client."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Oauth2ClientInfo {
    #[doc = "Value used for [CSRF](https://tools.ietf.org/html/rfc6749#section-10.12) protection \
             via the `state` parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    #[doc = "Code Verifier used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection via \
             the `code_verifier` parameter. The value must have a minimum length of 43 characters \
             and a maximum length of 128 characters.  Each character must be ASCII alphanumeric \
             or one of the characters \"-\" / \".\" / \"_\" / \"~\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pkce_code_verifier: Option<String>,
    #[doc = "The URL for consent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl std::fmt::Display for Oauth2ClientInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Oauth2ClientInfo {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(csrf_token) = &self.csrf_token {
                format!("{:?}", csrf_token).into()
            } else {
                String::new().into()
            },
            if let Some(pkce_code_verifier) = &self.pkce_code_verifier {
                format!("{:?}", pkce_code_verifier).into()
            } else {
                String::new().into()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "csrf_token".into(),
            "pkce_code_verifier".into(),
            "url".into(),
        ]
    }
}

#[doc = "An OAuth 2.0 Grant Type. These are documented here: <https://oauth.net/2/grant-types/>."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[derive(Default)]
pub enum Oauth2GrantType {
    #[doc = "An OAuth 2.0 Device Authorization Grant."]
    #[serde(rename = "urn:ietf:params:oauth:grant-type:device_code")]
    #[display("urn:ietf:params:oauth:grant-type:device_code")]
    #[default]
    UrnIetfParamsOauthGrantTypeDeviceCode,
}



#[doc = "A successful response from a modeling command. This can be one of several types of \
         responses, depending on the command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type")]
pub enum OkModelingCmdResponse {
    #[serde(rename = "empty")]
    Empty {},
    #[serde(rename = "export")]
    Export {
        #[doc = "The response from the `Export` endpoint."]
        data: Export,
    },
    #[serde(rename = "select_with_point")]
    SelectWithPoint {
        #[doc = "The response from the `SelectWithPoint` command."]
        data: SelectWithPoint,
    },
    #[serde(rename = "highlight_set_entity")]
    HighlightSetEntity {
        #[doc = "The response from the `HighlightSetEntity` command."]
        data: HighlightSetEntity,
    },
    #[serde(rename = "entity_get_child_uuid")]
    EntityGetChildUuid {
        #[doc = "The response from the `EntityGetChildUuid` command."]
        data: EntityGetChildUuid,
    },
    #[serde(rename = "entity_get_num_children")]
    EntityGetNumChildren {
        #[doc = "The response from the `EntityGetNumChildren` command."]
        data: EntityGetNumChildren,
    },
    #[serde(rename = "entity_get_parent_id")]
    EntityGetParentId {
        #[doc = "The response from the `EntityGetParentId` command."]
        data: EntityGetParentId,
    },
    #[serde(rename = "entity_get_all_child_uuids")]
    EntityGetAllChildUuids {
        #[doc = "The response from the `EntityGetAllChildUuids` command."]
        data: EntityGetAllChildUuids,
    },
    #[serde(rename = "select_get")]
    SelectGet {
        #[doc = "The response from the `SelectGet` command."]
        data: SelectGet,
    },
    #[serde(rename = "get_entity_type")]
    GetEntityType {
        #[doc = "The response from the `GetEntityType` command."]
        data: GetEntityType,
    },
    #[serde(rename = "solid3d_get_all_edge_faces")]
    Solid3DGetAllEdgeFaces {
        #[doc = "The response from the `Solid3dGetAllEdgeFaces` command."]
        data: Solid3DGetAllEdgeFaces,
    },
    #[serde(rename = "solid3d_get_all_opposite_edges")]
    Solid3DGetAllOppositeEdges {
        #[doc = "The response from the `Solid3dGetAllOppositeEdges` command."]
        data: Solid3DGetAllOppositeEdges,
    },
    #[serde(rename = "solid3d_get_opposite_edge")]
    Solid3DGetOppositeEdge {
        #[doc = "The response from the `Solid3dGetOppositeEdge` command."]
        data: Solid3DGetOppositeEdge,
    },
    #[serde(rename = "solid3d_get_prev_adjacent_edge")]
    Solid3DGetPrevAdjacentEdge {
        #[doc = "The response from the `Solid3dGetPrevAdjacentEdge` command."]
        data: Solid3DGetPrevAdjacentEdge,
    },
    #[serde(rename = "solid3d_get_next_adjacent_edge")]
    Solid3DGetNextAdjacentEdge {
        #[doc = "The response from the `Solid3dGetNextAdjacentEdge` command."]
        data: Solid3DGetNextAdjacentEdge,
    },
    #[serde(rename = "mouse_click")]
    MouseClick {
        #[doc = "The response from the `MouseClick` command."]
        data: MouseClick,
    },
    #[serde(rename = "curve_get_type")]
    CurveGetType {
        #[doc = "The response from the `CurveGetType` command."]
        data: CurveGetType,
    },
    #[serde(rename = "curve_get_control_points")]
    CurveGetControlPoints {
        #[doc = "The response from the `CurveGetControlPoints` command."]
        data: CurveGetControlPoints,
    },
    #[serde(rename = "take_snapshot")]
    TakeSnapshot {
        #[doc = "The response from the `TakeSnapshot` command."]
        data: TakeSnapshot,
    },
    #[serde(rename = "path_get_info")]
    PathGetInfo {
        #[doc = "The response from the `PathGetInfo` command."]
        data: PathGetInfo,
    },
    #[serde(rename = "path_get_curve_uuids_for_vertices")]
    PathGetCurveUuidsForVertices {
        #[doc = "The response from the `PathGetCurveUuidsForVertices` command."]
        data: PathGetCurveUuidsForVertices,
    },
    #[serde(rename = "plane_intersect_and_project")]
    PlaneIntersectAndProject {
        #[doc = "Corresponding coordinates of given window coordinates, intersected on given \
                 plane."]
        data: PlaneIntersectAndProject,
    },
    #[serde(rename = "curve_get_end_points")]
    CurveGetEndPoints {
        #[doc = "Endpoints of a curve"]
        data: CurveGetEndPoints,
    },
    #[serde(rename = "import_files")]
    ImportFiles {
        #[doc = "Data from importing the files"]
        data: ImportFiles,
    },
    #[serde(rename = "mass")]
    Mass {
        #[doc = "The mass response."]
        data: Mass,
    },
    #[serde(rename = "volume")]
    Volume {
        #[doc = "The volume response."]
        data: Volume,
    },
    #[serde(rename = "density")]
    Density {
        #[doc = "The density response."]
        data: Density,
    },
    #[serde(rename = "surface_area")]
    SurfaceArea {
        #[doc = "The surface area response."]
        data: SurfaceArea,
    },
    #[serde(rename = "center_of_mass")]
    CenterOfMass {
        #[doc = "The center of mass response."]
        data: CenterOfMass,
    },
}

#[doc = "The websocket messages this server sends."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type", content = "data")]
pub enum OkWebSocketResponseData {
    #[serde(rename = "ice_server_info")]
    IceServerInfo {
        #[doc = "Information about the ICE servers."]
        ice_servers: Vec<IceServer>,
    },
    #[serde(rename = "trickle_ice")]
    TrickleIce {
        #[doc = "Information about the ICE candidate."]
        candidate: RtcIceCandidateInit,
    },
    #[serde(rename = "sdp_answer")]
    SdpAnswer {
        #[doc = "The session description."]
        answer: RtcSessionDescription,
    },
    #[serde(rename = "modeling")]
    Modeling {
        #[doc = "The result of the command."]
        modeling_response: OkModelingCmdResponse,
    },
    #[serde(rename = "export")]
    Export {
        #[doc = "The exported files"]
        files: Vec<RawFile>,
    },
    #[serde(rename = "metrics_request")]
    MetricsRequest {},
}

#[doc = "Onboarding details"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Onboarding {
    #[doc = "When the user first called an endpoint from their machine (i.e. not a litterbox \
             execution)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_call_from_their_machine_date: Option<String>,
    #[doc = "When the user first used the litterbox"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_litterbox_execute_date: Option<String>,
    #[doc = "When the user created their first token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_token_date: Option<String>,
}

impl std::fmt::Display for Onboarding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Onboarding {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(first_call_from_their_machine_date) =
                &self.first_call_from_their_machine_date
            {
                format!("{:?}", first_call_from_their_machine_date).into()
            } else {
                String::new().into()
            },
            if let Some(first_litterbox_execute_date) = &self.first_litterbox_execute_date {
                format!("{:?}", first_litterbox_execute_date).into()
            } else {
                String::new().into()
            },
            if let Some(first_token_date) = &self.first_token_date {
                format!("{:?}", first_token_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_call_from_their_machine_date".into(),
            "first_litterbox_execute_date".into(),
            "first_token_date".into(),
        ]
    }
}

#[doc = "Output file contents."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct OutputFile {
    #[doc = "The contents of the file. This is base64 encoded so we can ensure it is UTF-8 for \
             JSON."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[doc = "The name of the file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for OutputFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for OutputFile {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(contents) = &self.contents {
                format!("{:?}", contents).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["contents".into(), "name".into()]
    }
}

#[doc = "Output format specifier."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type")]
pub enum OutputFormat {
    #[serde(rename = "fbx")]
    Fbx {
        #[doc = "Specifies which kind of FBX will be exported."]
        storage: FbxStorage,
    },
    #[serde(rename = "gltf")]
    Gltf {
        #[doc = "Specifies how the JSON will be presented."]
        presentation: GltfPresentation,
        #[doc = "Specifies which kind of glTF 2.0 will be exported."]
        storage: GltfStorage,
    },
    #[serde(rename = "obj")]
    Obj {
        #[doc = "Co-ordinate system of output data.\n\nDefaults to the [KittyCAD co-ordinate \
                 system].\n\n[KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html"]
        coords: System,
        #[doc = "Export length unit.\n\nDefaults to meters."]
        units: UnitLength,
    },
    #[serde(rename = "ply")]
    Ply {
        #[doc = "Co-ordinate system of output data.\n\nDefaults to the [KittyCAD co-ordinate \
                 system].\n\n[KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html"]
        coords: System,
        #[doc = "The storage for the output PLY file."]
        storage: PlyStorage,
    },
    #[serde(rename = "step")]
    Step {
        #[doc = "Co-ordinate system of output data.\n\nDefaults to the [KittyCAD co-ordinate \
                 system].\n\n[KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html"]
        coords: System,
    },
    #[serde(rename = "stl")]
    Stl {
        #[doc = "Co-ordinate system of output data.\n\nDefaults to the [KittyCAD co-ordinate \
                 system].\n\n[KittyCAD co-ordinate system]: ../coord/constant.KITTYCAD.html"]
        coords: System,
        #[doc = "Export storage."]
        storage: StlStorage,
        #[doc = "Export length unit.\n\nDefaults to meters."]
        units: UnitLength,
    },
}

#[doc = "The path component command type (within a Path)"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum PathCommand {
    #[serde(rename = "move_to")]
    #[display("move_to")]
    MoveTo,
    #[serde(rename = "line_to")]
    #[display("line_to")]
    LineTo,
    #[serde(rename = "bez_curve_to")]
    #[display("bez_curve_to")]
    BezCurveTo,
    #[serde(rename = "nurbs_curve_to")]
    #[display("nurbs_curve_to")]
    NurbsCurveTo,
    #[serde(rename = "add_arc")]
    #[display("add_arc")]
    AddArc,
}

#[doc = "The response from the `PathGetCurveUuidsForVertices` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PathGetCurveUuidsForVertices {
    #[doc = "The UUIDs of the curve entities."]
    pub curve_ids: Vec<uuid::Uuid>,
}

impl std::fmt::Display for PathGetCurveUuidsForVertices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PathGetCurveUuidsForVertices {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.curve_ids).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["curve_ids".into()]
    }
}

#[doc = "The response from the `PathGetInfo` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PathGetInfo {
    #[doc = "All segments in the path, in the order they were added."]
    pub segments: Vec<PathSegmentInfo>,
}

impl std::fmt::Display for PathGetInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PathGetInfo {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.segments).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["segments".into()]
    }
}

#[doc = "A segment of a path. Paths are composed of many segments."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type")]
pub enum PathSegment {
    #[serde(rename = "line")]
    Line {
        #[doc = "End point of the line."]
        end: Point3D,
        #[doc = "Whether or not this line is a relative offset"]
        relative: bool,
    },
    #[serde(rename = "arc")]
    Arc {
        #[doc = "Start of the arc along circle's perimeter."]
        angle_end: f64,
        #[doc = "Start of the arc along circle's perimeter."]
        angle_start: f64,
        #[doc = "Center of the circle"]
        center: Point2D,
        #[doc = "Radius of the circle"]
        radius: f64,
        #[doc = "Whether or not this arc is a relative offset"]
        relative: bool,
    },
    #[serde(rename = "bezier")]
    Bezier {
        #[doc = "First control point."]
        control1: Point3D,
        #[doc = "Second control point."]
        control2: Point3D,
        #[doc = "Final control point."]
        end: Point3D,
        #[doc = "Whether or not this bezier is a relative offset"]
        relative: bool,
    },
}

#[doc = "Info about a path segment"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PathSegmentInfo {
    #[doc = "What is the path segment?"]
    pub command: PathCommand,
    #[doc = "Which command created this path? This field is absent if the path command is not \
             actually creating a path segment, e.g. moving the pen doesn't create a path segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<uuid::Uuid>,
    #[doc = "Whether or not this segment is a relative offset"]
    pub relative: bool,
}

impl std::fmt::Display for PathSegmentInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PathSegmentInfo {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.command).into(),
            if let Some(command_id) = &self.command_id {
                format!("{:?}", command_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.relative).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["command".into(), "command_id".into(), "relative".into()]
    }
}

#[doc = "A payment intent response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaymentIntent {
    #[doc = "The client secret is used for client-side retrieval using a publishable key. The \
             client secret can be used to complete payment setup from your frontend. It should \
             not be stored, logged, or exposed to anyone other than the customer. Make sure that \
             you have TLS enabled on any page that includes the client secret."]
    pub client_secret: String,
}

impl std::fmt::Display for PaymentIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaymentIntent {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.client_secret.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["client_secret".into()]
    }
}

#[doc = "A payment method."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaymentMethod {
    #[doc = "The billing info for the payment method."]
    pub billing_info: BillingInfo,
    #[doc = "The card, if it is one. For our purposes, this is the only type of payment method \
             that we support."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,
    #[doc = "Time at which the object was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "Unique identifier for the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Set of key-value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The type of payment method."]
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaymentMethod {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.billing_info).into(),
            if let Some(card) = &self.card {
                format!("{:?}", card).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.type_).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "billing_info".into(),
            "card".into(),
            "created_at".into(),
            "id".into(),
            "metadata".into(),
            "type_".into(),
        ]
    }
}

#[doc = "Card checks."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaymentMethodCardChecks {
    #[doc = "If a address line1 was provided, results of the check, one of `pass`, `fail`, \
             `unavailable`, or `unchecked`."]
    #[serde(
        rename = "address_line1_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_line_1_check: Option<String>,
    #[doc = "If a address postal code was provided, results of the check, one of `pass`, `fail`, \
             `unavailable`, or `unchecked`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check: Option<String>,
    #[doc = "If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, \
             or `unchecked`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
}

impl std::fmt::Display for PaymentMethodCardChecks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaymentMethodCardChecks {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(address_line_1_check) = &self.address_line_1_check {
                format!("{:?}", address_line_1_check).into()
            } else {
                String::new().into()
            },
            if let Some(address_postal_code_check) = &self.address_postal_code_check {
                format!("{:?}", address_postal_code_check).into()
            } else {
                String::new().into()
            },
            if let Some(cvc_check) = &self.cvc_check {
                format!("{:?}", cvc_check).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "address_line_1_check".into(),
            "address_postal_code_check".into(),
            "cvc_check".into(),
        ]
    }
}

#[doc = "An enum representing the possible values of an `PaymentMethod`'s `type` field."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[derive(Default)]
pub enum PaymentMethodType {
    #[doc = "A card payment method."]
    #[serde(rename = "card")]
    #[display("card")]
    #[default]
    Card,
}



#[doc = "Corresponding coordinates of given window coordinates, intersected on given plane."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PlaneIntersectAndProject {
    #[doc = "Corresponding coordinates of given window coordinates, intersected on given plane."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plane_coordinates: Option<Point2D>,
}

impl std::fmt::Display for PlaneIntersectAndProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PlaneIntersectAndProject {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(plane_coordinates) = &self.plane_coordinates {
            format!("{:?}", plane_coordinates).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["plane_coordinates".into()]
    }
}

#[doc = "The storage for the output PLY file."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum PlyStorage {
    #[doc = "Write numbers in their ascii representation (e.g. -13, 6.28, etc.). Properties are \
             separated by spaces and elements are separated by line breaks."]
    #[serde(rename = "ascii")]
    #[display("ascii")]
    Ascii,
    #[doc = "Encode payload as binary using little endian."]
    #[serde(rename = "binary_little_endian")]
    #[display("binary_little_endian")]
    BinaryLittleEndian,
    #[doc = "Encode payload as binary using big endian."]
    #[serde(rename = "binary_big_endian")]
    #[display("binary_big_endian")]
    BinaryBigEndian,
}

#[doc = "A point in 2D space"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Point2D {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.x).into(),
            format!("{:?}", self.y).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["x".into(), "y".into()]
    }
}

#[doc = "A point in 3D space"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Point3D {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.x).into(),
            format!("{:?}", self.y).into(),
            format!("{:?}", self.z).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["x".into(), "y".into(), "z".into()]
    }
}

#[doc = "Metadata about our point-e instance.\n\nThis is mostly used for internal purposes and \
         debugging."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PointEMetadata {
    #[doc = "If the point-e service returned an ok response from ping."]
    pub ok: bool,
}

impl std::fmt::Display for PointEMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PointEMetadata {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.ok).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["ok".into()]
    }
}

#[doc = "The response from the `/ping` endpoint."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Pong {
    #[doc = "The pong response."]
    pub message: String,
}

impl std::fmt::Display for Pong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Pong {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.message.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["message".into()]
    }
}

#[doc = "A raw file with unencoded contents to be passed over binary websockets."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RawFile {
    #[doc = "The contents of the file."]
    #[serde(
        serialize_with = "serde_bytes::serialize",
        deserialize_with = "serde_bytes::deserialize"
    )]
    pub contents: Vec<u8>,
    #[doc = "The name of the file."]
    pub name: String,
}

impl std::fmt::Display for RawFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RawFile {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.contents).into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["contents".into(), "name".into()]
    }
}

#[doc = "ICECandidateInit is used to serialize ice candidates"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RtcIceCandidateInit {
    #[doc = "The candidate string associated with the object."]
    pub candidate: String,
    #[doc = "The index (starting at zero) of the m-line in the SDP this candidate is associated \
             with."]
    #[serde(
        rename = "sdpMLineIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sdp_m_line_index: Option<u16>,
    #[doc = "The identifier of the \"media stream identification\" as defined in [RFC 8841](https://tools.ietf.org/html/rfc8841)."]
    #[serde(rename = "sdpMid", default, skip_serializing_if = "Option::is_none")]
    pub sdp_mid: Option<String>,
    #[doc = "The username fragment (as defined in [RFC 8445](https://tools.ietf.org/html/rfc8445#section-5.2.1)) associated with the object."]
    #[serde(
        rename = "usernameFragment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub username_fragment: Option<String>,
}

impl std::fmt::Display for RtcIceCandidateInit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RtcIceCandidateInit {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.candidate.clone().into(),
            if let Some(sdp_m_line_index) = &self.sdp_m_line_index {
                format!("{:?}", sdp_m_line_index).into()
            } else {
                String::new().into()
            },
            if let Some(sdp_mid) = &self.sdp_mid {
                format!("{:?}", sdp_mid).into()
            } else {
                String::new().into()
            },
            if let Some(username_fragment) = &self.username_fragment {
                format!("{:?}", username_fragment).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "candidate".into(),
            "sdp_m_line_index".into(),
            "sdp_mid".into(),
            "username_fragment".into(),
        ]
    }
}

#[doc = "SDPType describes the type of an SessionDescription."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum RtcSdpType {
    #[doc = "Unspecified indicates that the type is unspecified."]
    #[serde(rename = "unspecified")]
    #[display("unspecified")]
    Unspecified,
    #[doc = "indicates that a description MUST be treated as an SDP offer."]
    #[serde(rename = "offer")]
    #[display("offer")]
    Offer,
    #[doc = "indicates that a description MUST be treated as an SDP answer, but not a final \
             answer. A description used as an SDP pranswer may be applied as a response to an SDP \
             offer, or an update to a previously sent SDP pranswer."]
    #[serde(rename = "pranswer")]
    #[display("pranswer")]
    Pranswer,
    #[doc = "indicates that a description MUST be treated as an SDP final answer, and the \
             offer-answer exchange MUST be considered complete. A description used as an SDP \
             answer may be applied as a response to an SDP offer or as an update to a previously \
             sent SDP pranswer."]
    #[serde(rename = "answer")]
    #[display("answer")]
    Answer,
    #[doc = "indicates that a description MUST be treated as canceling the current SDP \
             negotiation and moving the SDP offer and answer back to what it was in the previous \
             stable state. Note the local or remote SDP descriptions in the previous stable state \
             could be null if there has not yet been a successful offer-answer negotiation."]
    #[serde(rename = "rollback")]
    #[display("rollback")]
    Rollback,
}

#[doc = "SessionDescription is used to expose local and remote session descriptions."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RtcSessionDescription {
    #[doc = "SDP string."]
    pub sdp: String,
    #[doc = "SDP type."]
    #[serde(rename = "type")]
    pub type_: RtcSdpType,
}

impl std::fmt::Display for RtcSessionDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RtcSessionDescription {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.sdp.clone().into(), format!("{:?}", self.type_).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["sdp".into(), "type_".into()]
    }
}

#[doc = "The type of scene selection change"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum SceneSelectionType {
    #[doc = "Replaces the selection"]
    #[serde(rename = "replace")]
    #[display("replace")]
    Replace,
    #[doc = "Adds to the selection"]
    #[serde(rename = "add")]
    #[display("add")]
    Add,
    #[doc = "Removes from the selection"]
    #[serde(rename = "remove")]
    #[display("remove")]
    Remove,
}

#[doc = "The type of scene's active tool"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum SceneToolType {
    #[serde(rename = "camera_revolve")]
    #[display("camera_revolve")]
    CameraRevolve,
    #[serde(rename = "select")]
    #[display("select")]
    Select,
    #[serde(rename = "move")]
    #[display("move")]
    Move,
    #[serde(rename = "sketch_line")]
    #[display("sketch_line")]
    SketchLine,
    #[serde(rename = "sketch_curve")]
    #[display("sketch_curve")]
    SketchCurve,
    #[serde(rename = "sketch_curve_mod")]
    #[display("sketch_curve_mod")]
    SketchCurveMod,
}

#[doc = "The response from the `SelectGet` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SelectGet {
    #[doc = "The UUIDs of the selected entities."]
    pub entity_ids: Vec<uuid::Uuid>,
}

impl std::fmt::Display for SelectGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SelectGet {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.entity_ids).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_ids".into()]
    }
}

#[doc = "The response from the `SelectWithPoint` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SelectWithPoint {
    #[doc = "The UUID of the entity that was selected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for SelectWithPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SelectWithPoint {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(entity_id) = &self.entity_id {
            format!("{:?}", entity_id).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_id".into()]
    }
}

#[doc = "An authentication session.\n\nFor our UIs, these are automatically created by Next.js."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Session {
    #[doc = "The date and time the session was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The date and time the session expires."]
    pub expires: chrono::DateTime<chrono::Utc>,
    #[doc = "The unique identifier for the session."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The session token."]
    pub session_token: uuid::Uuid,
    #[doc = "The date and time the session was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user that the session belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Session {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.created_at).into(),
            format!("{:?}", self.expires).into(),
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.session_token).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "expires".into(),
            "id".into(),
            "session_token".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The response from the `Solid3dGetAllEdgeFaces` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Solid3DGetAllEdgeFaces {
    #[doc = "The UUIDs of the faces."]
    pub faces: Vec<uuid::Uuid>,
}

impl std::fmt::Display for Solid3DGetAllEdgeFaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Solid3DGetAllEdgeFaces {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.faces).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["faces".into()]
    }
}

#[doc = "The response from the `Solid3dGetAllOppositeEdges` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Solid3DGetAllOppositeEdges {
    #[doc = "The UUIDs of the edges."]
    pub edges: Vec<uuid::Uuid>,
}

impl std::fmt::Display for Solid3DGetAllOppositeEdges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Solid3DGetAllOppositeEdges {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.edges).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["edges".into()]
    }
}

#[doc = "The response from the `Solid3dGetNextAdjacentEdge` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Solid3DGetNextAdjacentEdge {
    #[doc = "The UUID of the edge."]
    pub edge: uuid::Uuid,
}

impl std::fmt::Display for Solid3DGetNextAdjacentEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Solid3DGetNextAdjacentEdge {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.edge).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["edge".into()]
    }
}

#[doc = "The response from the `Solid3dGetOppositeEdge` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Solid3DGetOppositeEdge {
    #[doc = "The UUID of the edge."]
    pub edge: uuid::Uuid,
}

impl std::fmt::Display for Solid3DGetOppositeEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Solid3DGetOppositeEdge {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.edge).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["edge".into()]
    }
}

#[doc = "The response from the `Solid3dGetPrevAdjacentEdge` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Solid3DGetPrevAdjacentEdge {
    #[doc = "The UUID of the edge."]
    pub edge: uuid::Uuid,
}

impl std::fmt::Display for Solid3DGetPrevAdjacentEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Solid3DGetPrevAdjacentEdge {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.edge).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["edge".into()]
    }
}

#[doc = "Export storage."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum StlStorage {
    #[doc = "Plaintext encoding."]
    #[serde(rename = "ascii")]
    #[display("ascii")]
    Ascii,
    #[doc = "Binary STL encoding.\n\nThis is the default setting."]
    #[serde(rename = "binary")]
    #[display("binary")]
    Binary,
}

#[doc = "Successful Websocket response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SuccessWebSocketResponse {
    #[doc = "Which request this is a response to. If the request was a modeling command, this is \
             the modeling command ID. If no request ID was sent, this will be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<uuid::Uuid>,
    #[doc = "The data sent with a successful response. This will be flattened into a 'type' and \
             'data' field."]
    pub resp: OkWebSocketResponseData,
    #[doc = "Always true"]
    pub success: bool,
}

impl std::fmt::Display for SuccessWebSocketResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SuccessWebSocketResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(request_id) = &self.request_id {
                format!("{:?}", request_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.resp).into(),
            format!("{:?}", self.success).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["request_id".into(), "resp".into(), "success".into()]
    }
}

#[doc = "The surface area response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SurfaceArea {
    #[doc = "The output unit for the surface area."]
    pub output_unit: UnitArea,
    #[doc = "The surface area."]
    pub surface_area: f64,
}

impl std::fmt::Display for SurfaceArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SurfaceArea {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.output_unit).into(),
            format!("{:?}", self.surface_area).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["output_unit".into(), "surface_area".into()]
    }
}

#[doc = "Co-ordinate system definition.\n\nThe `up` axis must be orthogonal to the `forward` axis.\n\nSee [cglearn.eu] for background reading.\n\n[cglearn.eu](https://cglearn.eu/pub/computer-graphics/introduction-to-geometry#material-coordinate-systems-1)"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct System {
    #[doc = "Axis the front face of a model looks along."]
    pub forward: AxisDirectionPair,
    #[doc = "Axis pointing up and away from a model."]
    pub up: AxisDirectionPair,
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for System {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.forward).into(),
            format!("{:?}", self.up).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["forward".into(), "up".into()]
    }
}

#[doc = "The response from the `TakeSnapshot` command."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TakeSnapshot {
    #[doc = "Contents of the image."]
    pub contents: base64::Base64Data,
}

impl std::fmt::Display for TakeSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TakeSnapshot {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.contents).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["contents".into()]
    }
}

#[doc = "The valid types of angle formats."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitAngle {
    #[doc = "Degrees <https://en.wikipedia.org/wiki/Degree_(angle)>"]
    #[serde(rename = "degrees")]
    #[display("degrees")]
    Degrees,
    #[doc = "Radians <https://en.wikipedia.org/wiki/Radian>"]
    #[serde(rename = "radians")]
    #[display("radians")]
    Radians,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitAngleConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitAngle,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitAngle,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitAngleConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitAngleConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of area units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitArea {
    #[doc = "Square centimetres <https://en.wikipedia.org/wiki/Square_centimetre>"]
    #[serde(rename = "cm2")]
    #[display("cm2")]
    Cm2,
    #[doc = "Square decimetres <https://en.wikipedia.org/wiki/Square_decimetre>"]
    #[serde(rename = "dm2")]
    #[display("dm2")]
    Dm2,
    #[doc = "Square feet <https://en.wikipedia.org/wiki/Square_foot>"]
    #[serde(rename = "ft2")]
    #[display("ft2")]
    Ft2,
    #[doc = "Square inches <https://en.wikipedia.org/wiki/Square_inch>"]
    #[serde(rename = "in2")]
    #[display("in2")]
    In2,
    #[doc = "Square kilometres <https://en.wikipedia.org/wiki/Square_kilometre>"]
    #[serde(rename = "km2")]
    #[display("km2")]
    Km2,
    #[doc = "Square metres <https://en.wikipedia.org/wiki/Square_metre>"]
    #[serde(rename = "m2")]
    #[display("m2")]
    M2,
    #[doc = "Square millimetres <https://en.wikipedia.org/wiki/Square_millimetre>"]
    #[serde(rename = "mm2")]
    #[display("mm2")]
    Mm2,
    #[doc = "Square yards <https://en.wikipedia.org/wiki/Square_mile>"]
    #[serde(rename = "yd2")]
    #[display("yd2")]
    Yd2,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitAreaConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitArea,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitArea,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitAreaConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitAreaConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of current units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitCurrent {
    #[doc = "Amperes <https://en.wikipedia.org/wiki/Ampere>"]
    #[serde(rename = "amperes")]
    #[display("amperes")]
    Amperes,
    #[doc = "Microamperes <https://en.wikipedia.org/wiki/Microampere>"]
    #[serde(rename = "microamperes")]
    #[display("microamperes")]
    Microamperes,
    #[doc = "Milliamperes <https://en.wikipedia.org/wiki/Milliampere>"]
    #[serde(rename = "milliamperes")]
    #[display("milliamperes")]
    Milliamperes,
    #[doc = "Nanoamperes <https://en.wikipedia.org/wiki/Nanoampere>"]
    #[serde(rename = "nanoamperes")]
    #[display("nanoamperes")]
    Nanoamperes,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitCurrentConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitCurrent,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitCurrent,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitCurrentConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitCurrentConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types for density units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitDensity {
    #[doc = "Pounds per cubic feet."]
    #[serde(rename = "lb:ft3")]
    #[display("lb:ft3")]
    LbFt3,
    #[doc = "Kilograms per cubic meter."]
    #[serde(rename = "kg:m3")]
    #[display("kg:m3")]
    KgM3,
}

#[doc = "The valid types of energy units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitEnergy {
    #[doc = "British Thermal Unit (BTU) <https://en.wikipedia.org/wiki/British_thermal_unit>"]
    #[serde(rename = "btu")]
    #[display("btu")]
    Btu,
    #[doc = "Electron Volts (eV) <https://en.wikipedia.org/wiki/Electronvolt>"]
    #[serde(rename = "electronvolts")]
    #[display("electronvolts")]
    Electronvolts,
    #[doc = "Joules (or watt-seconds) <https://en.wikipedia.org/wiki/Joule>"]
    #[serde(rename = "joules")]
    #[display("joules")]
    Joules,
    #[doc = "Kilocalories (often just called calories) <https://en.wikipedia.org/wiki/Kilocalorie>"]
    #[serde(rename = "kilocalories")]
    #[display("kilocalories")]
    Kilocalories,
    #[doc = "Kilowatt hours (kWh) <https://en.wikipedia.org/wiki/Kilowatt-hour>"]
    #[serde(rename = "kilowatt_hours")]
    #[display("kilowatt_hours")]
    KilowattHours,
    #[doc = "Watt hours (Wh) <https://en.wikipedia.org/wiki/Kilowatt-hour>"]
    #[serde(rename = "watt_hours")]
    #[display("watt_hours")]
    WattHours,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitEnergyConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitEnergy,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitEnergy,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitEnergyConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitEnergyConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of force units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitForce {
    #[doc = "Dynes <https://en.wikipedia.org/wiki/Dyne>"]
    #[serde(rename = "dynes")]
    #[display("dynes")]
    Dynes,
    #[doc = "Kiloponds <https://en.wikipedia.org/wiki/Kilopond>"]
    #[serde(rename = "kiloponds")]
    #[display("kiloponds")]
    Kiloponds,
    #[doc = "Micronewtons <https://en.wikipedia.org/wiki/Newton_(unit)>"]
    #[serde(rename = "micronewtons")]
    #[display("micronewtons")]
    Micronewtons,
    #[doc = "Millinewtons <https://en.wikipedia.org/wiki/Newton_(unit)>"]
    #[serde(rename = "millinewtons")]
    #[display("millinewtons")]
    Millinewtons,
    #[doc = "Newtons <https://en.wikipedia.org/wiki/Newton_(unit)>"]
    #[serde(rename = "newtons")]
    #[display("newtons")]
    Newtons,
    #[doc = "Poundals <https://en.wikipedia.org/wiki/Poundal>"]
    #[serde(rename = "poundals")]
    #[display("poundals")]
    Poundals,
    #[doc = "Pounds <https://en.wikipedia.org/wiki/Pound_(force)>"]
    #[serde(rename = "pounds")]
    #[display("pounds")]
    Pounds,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitForceConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitForce,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitForce,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitForceConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitForceConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of frequency units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitFrequency {
    #[doc = "Gigahertz <https://en.wikipedia.org/wiki/Hertz>"]
    #[serde(rename = "gigahertz")]
    #[display("gigahertz")]
    Gigahertz,
    #[doc = "Hertz <https://en.wikipedia.org/wiki/Hertz>"]
    #[serde(rename = "hertz")]
    #[display("hertz")]
    Hertz,
    #[doc = "Kilohertz <https://en.wikipedia.org/wiki/Hertz>"]
    #[serde(rename = "kilohertz")]
    #[display("kilohertz")]
    Kilohertz,
    #[doc = "Megahertz <https://en.wikipedia.org/wiki/Hertz>"]
    #[serde(rename = "megahertz")]
    #[display("megahertz")]
    Megahertz,
    #[doc = "Microhertz <https://en.wikipedia.org/wiki/Hertz>"]
    #[serde(rename = "microhertz")]
    #[display("microhertz")]
    Microhertz,
    #[doc = "Millihertz <https://en.wikipedia.org/wiki/Hertz>"]
    #[serde(rename = "millihertz")]
    #[display("millihertz")]
    Millihertz,
    #[doc = "Nanohertz <https://en.wikipedia.org/wiki/Hertz>"]
    #[serde(rename = "nanohertz")]
    #[display("nanohertz")]
    Nanohertz,
    #[doc = "Terahertz <https://en.wikipedia.org/wiki/Hertz>"]
    #[serde(rename = "terahertz")]
    #[display("terahertz")]
    Terahertz,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitFrequencyConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitFrequency,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitFrequency,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitFrequencyConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitFrequencyConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of length units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitLength {
    #[doc = "Centimetres <https://en.wikipedia.org/wiki/Centimetre>"]
    #[serde(rename = "cm")]
    #[display("cm")]
    Cm,
    #[doc = "Feet <https://en.wikipedia.org/wiki/Foot_(unit)>"]
    #[serde(rename = "ft")]
    #[display("ft")]
    Ft,
    #[doc = "Inches <https://en.wikipedia.org/wiki/Inch>"]
    #[serde(rename = "in")]
    #[display("in")]
    In,
    #[doc = "Metres <https://en.wikipedia.org/wiki/Metre>"]
    #[serde(rename = "m")]
    #[display("m")]
    M,
    #[doc = "Millimetres <https://en.wikipedia.org/wiki/Millimetre>"]
    #[serde(rename = "mm")]
    #[display("mm")]
    Mm,
    #[doc = "Yards <https://en.wikipedia.org/wiki/Yard>"]
    #[serde(rename = "yd")]
    #[display("yd")]
    Yd,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitLengthConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitLength,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitLength,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitLengthConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitLengthConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of mass units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitMass {
    #[doc = "Grams <https://en.wikipedia.org/wiki/Gram>"]
    #[serde(rename = "g")]
    #[display("g")]
    G,
    #[doc = "Kilograms <https://en.wikipedia.org/wiki/Kilogram>"]
    #[serde(rename = "kg")]
    #[display("kg")]
    Kg,
    #[doc = "Pounds <https://en.wikipedia.org/wiki/Pound_(mass)>"]
    #[serde(rename = "lb")]
    #[display("lb")]
    Lb,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitMassConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitMass,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitMass,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitMassConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitMassConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of power units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitPower {
    #[doc = "British thermal units (BTU) per minute <https://en.wikipedia.org/wiki/British_thermal_unit>"]
    #[serde(rename = "btu_per_minute")]
    #[display("btu_per_minute")]
    BtuPerMinute,
    #[doc = "Horsepower (hp) <https://en.wikipedia.org/wiki/Horsepower>"]
    #[serde(rename = "horsepower")]
    #[display("horsepower")]
    Horsepower,
    #[doc = "Kilowatts <https://en.wikipedia.org/wiki/Kilowatt>"]
    #[serde(rename = "kilowatts")]
    #[display("kilowatts")]
    Kilowatts,
    #[doc = "Metric horsepower (PS) <https://en.wikipedia.org/wiki/Horsepower#Metric_horsepower>"]
    #[serde(rename = "metric_horsepower")]
    #[display("metric_horsepower")]
    MetricHorsepower,
    #[doc = "Microwatts <https://en.wikipedia.org/wiki/Microwatt>"]
    #[serde(rename = "microwatts")]
    #[display("microwatts")]
    Microwatts,
    #[doc = "Millwatts <https://en.wikipedia.org/wiki/Milliwatt>"]
    #[serde(rename = "milliwatts")]
    #[display("milliwatts")]
    Milliwatts,
    #[doc = "Watts <https://en.wikipedia.org/wiki/Watt>"]
    #[serde(rename = "watts")]
    #[display("watts")]
    Watts,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitPowerConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitPower,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitPower,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitPowerConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitPowerConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of pressure units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitPressure {
    #[doc = "Atmospheres <https://en.wikipedia.org/wiki/Standard_atmosphere_(unit)>"]
    #[serde(rename = "atmospheres")]
    #[display("atmospheres")]
    Atmospheres,
    #[doc = "Bars <https://en.wikipedia.org/wiki/Bar_(unit)>"]
    #[serde(rename = "bars")]
    #[display("bars")]
    Bars,
    #[doc = "Hectopascals <https://en.wikipedia.org/wiki/Hectopascal>"]
    #[serde(rename = "hectopascals")]
    #[display("hectopascals")]
    Hectopascals,
    #[doc = "Kilopascals <https://en.wikipedia.org/wiki/Kilopascal>"]
    #[serde(rename = "kilopascals")]
    #[display("kilopascals")]
    Kilopascals,
    #[doc = "Millibars <https://en.wikipedia.org/wiki/Bar_(unit)>"]
    #[serde(rename = "millibars")]
    #[display("millibars")]
    Millibars,
    #[doc = "Pascals <https://en.wikipedia.org/wiki/Pascal_(unit)>"]
    #[serde(rename = "pascals")]
    #[display("pascals")]
    Pascals,
    #[doc = "Pounds per square inch (PSI) - <https://en.wikipedia.org/wiki/Pound_per_square_inch>"]
    #[serde(rename = "psi")]
    #[display("psi")]
    Psi,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitPressureConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitPressure,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitPressure,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitPressureConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitPressureConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of temperature units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitTemperature {
    #[doc = "Celsius <https://en.wikipedia.org/wiki/Celsius>"]
    #[serde(rename = "celsius")]
    #[display("celsius")]
    Celsius,
    #[doc = "Fahrenheit <https://en.wikipedia.org/wiki/Fahrenheit>"]
    #[serde(rename = "fahrenheit")]
    #[display("fahrenheit")]
    Fahrenheit,
    #[doc = "Kelvin <https://en.wikipedia.org/wiki/Kelvin>"]
    #[serde(rename = "kelvin")]
    #[display("kelvin")]
    Kelvin,
    #[doc = "Rankine <https://en.wikipedia.org/wiki/Rankine_scale>"]
    #[serde(rename = "rankine")]
    #[display("rankine")]
    Rankine,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitTemperatureConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitTemperature,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitTemperature,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitTemperatureConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitTemperatureConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of torque units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitTorque {
    #[doc = "Newton metres <https://en.wikipedia.org/wiki/Newton_metre>"]
    #[serde(rename = "newton_metres")]
    #[display("newton_metres")]
    NewtonMetres,
    #[doc = "Pound foot <https://en.wikipedia.org/wiki/Pound-foot_(torque)>"]
    #[serde(rename = "pound_foot")]
    #[display("pound_foot")]
    PoundFoot,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitTorqueConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitTorque,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitTorque,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitTorqueConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitTorqueConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The valid types of volume units."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnitVolume {
    #[doc = "Cubic centimetres (cc or cm³) <https://en.wikipedia.org/wiki/Cubic_centimetre>"]
    #[serde(rename = "cm3")]
    #[display("cm3")]
    Cm3,
    #[doc = "Cubic feet (ft³) <https://en.wikipedia.org/wiki/Cubic_foot>"]
    #[serde(rename = "ft3")]
    #[display("ft3")]
    Ft3,
    #[doc = "Cubic inches (cu in or in³) <https://en.wikipedia.org/wiki/Cubic_inch>"]
    #[serde(rename = "in3")]
    #[display("in3")]
    In3,
    #[doc = "Cubic metres (m³) <https://en.wikipedia.org/wiki/Cubic_metre>"]
    #[serde(rename = "m3")]
    #[display("m3")]
    M3,
    #[doc = "Cubic yards (yd³) <https://en.wikipedia.org/wiki/Cubic_yard>"]
    #[serde(rename = "yd3")]
    #[display("yd3")]
    Yd3,
    #[doc = "US Fluid Ounces (fl oz) <https://en.wikipedia.org/wiki/Fluid_ounce>"]
    #[serde(rename = "usfloz")]
    #[display("usfloz")]
    Usfloz,
    #[doc = "US Gallons (gal US) <https://en.wikipedia.org/wiki/Gallon>"]
    #[serde(rename = "usgal")]
    #[display("usgal")]
    Usgal,
    #[doc = "Liters (l) <https://en.wikipedia.org/wiki/Litre>"]
    #[serde(rename = "l")]
    #[display("l")]
    L,
    #[doc = "Milliliters (ml) <https://en.wikipedia.org/wiki/Litre>"]
    #[serde(rename = "ml")]
    #[display("ml")]
    Ml,
}

#[doc = "Result of converting between units."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitVolumeConversion {
    #[doc = "The time and date the API call was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the API call was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the API call.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The source format of the unit conversion."]
    pub input_unit: UnitVolume,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_unit: UnitVolume,
    #[doc = "The time and date the API call was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the API call."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the API call was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the API call."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitVolumeConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnitVolumeConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(input) = &self.input {
                format!("{:?}", input).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_unit).into(),
            if let Some(output) = &self.output {
                format!("{:?}", output).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.output_unit).into(),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.updated_at).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "created_at".into(),
            "error".into(),
            "id".into(),
            "input".into(),
            "input_unit".into(),
            "output".into(),
            "output_unit".into(),
            "started_at".into(),
            "status".into(),
            "updated_at".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "The user-modifiable parts of a User."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateUser {
    #[doc = "The user's company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "The user's Discord handle."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[doc = "The user's first name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The user's GitHub handle."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[doc = "The user's last name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The user's phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: phone_number::PhoneNumber,
}

impl std::fmt::Display for UpdateUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateUser {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(company) = &self.company {
                format!("{:?}", company).into()
            } else {
                String::new().into()
            },
            if let Some(discord) = &self.discord {
                format!("{:?}", discord).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(github) = &self.github {
                format!("{:?}", github).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.phone).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company".into(),
            "discord".into(),
            "first_name".into(),
            "github".into(),
            "last_name".into(),
            "phone".into(),
        ]
    }
}

#[doc = "A user."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct User {
    #[doc = "The user's company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "The date and time the user was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user's Discord handle."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[doc = "The email address of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The date and time the email address was verified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The user's first name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The user's GitHub handle."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[doc = "The unique identifier for the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The image avatar for the user. This is a URL."]
    pub image: String,
    #[doc = "The user's last name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The name of the user. This is auto populated at first from the authentication \
             provider (if there was a name). It can be updated by the user by updating their \
             `first_name` and `last_name` fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The user's phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: phone_number::PhoneNumber,
    #[doc = "The date and time the user was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for User {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(company) = &self.company {
                format!("{:?}", company).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(discord) = &self.discord {
                format!("{:?}", discord).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(email_verified) = &self.email_verified {
                format!("{:?}", email_verified).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(github) = &self.github {
                format!("{:?}", github).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            self.image.clone().into(),
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.phone).into(),
            format!("{:?}", self.updated_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company".into(),
            "created_at".into(),
            "discord".into(),
            "email".into(),
            "email_verified".into(),
            "first_name".into(),
            "github".into(),
            "id".into(),
            "image".into(),
            "last_name".into(),
            "name".into(),
            "phone".into(),
            "updated_at".into(),
        ]
    }
}

#[doc = "A single page of results"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserResultsPage {
    #[doc = "list of items on this page of results"]
    pub items: Vec<User>,
    #[doc = "token used to fetch the next page of results (if any)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

impl std::fmt::Display for UserResultsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for UserResultsPage {
    type Item = User;
    fn has_more_pages(&self) -> bool {
        self.next_page.is_some()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        req.url_mut()
            .query_pairs_mut()
            .append_pair("next_page", self.next_page.as_deref().unwrap_or(""));
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.items.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.items).into(),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["items".into(), "next_page".into()]
    }
}

#[doc = "A verification token for a user.\n\nThis is typically used to verify a user's email \
         address."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct VerificationToken {
    #[doc = "The date and time the verification token was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The date and time the verification token expires."]
    pub expires: chrono::DateTime<chrono::Utc>,
    #[doc = "The token used for verification. This is used as the id for the table since it is \
             unique per record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The identifier for the user. This is typically the user's email address since that \
             is what we are verifying."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[doc = "The date and time the verification token was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for VerificationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for VerificationToken {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.created_at).into(),
            format!("{:?}", self.expires).into(),
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(identifier) = &self.identifier {
                format!("{:?}", identifier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.updated_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "expires".into(),
            "id".into(),
            "identifier".into(),
            "updated_at".into(),
        ]
    }
}

#[doc = "The volume response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Volume {
    #[doc = "The output unit for the volume."]
    pub output_unit: UnitVolume,
    #[doc = "The volume."]
    pub volume: f64,
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Volume {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.output_unit).into(),
            format!("{:?}", self.volume).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["output_unit".into(), "volume".into()]
    }
}

#[doc = "The websocket messages the server receives."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type")]
pub enum WebSocketRequest {
    #[serde(rename = "trickle_ice")]
    TrickleIce {
        #[doc = "Information about the ICE candidate."]
        candidate: RtcIceCandidateInit,
    },
    #[serde(rename = "sdp_offer")]
    SdpOffer {
        #[doc = "The session description."]
        offer: RtcSessionDescription,
    },
    #[serde(rename = "modeling_cmd_req")]
    ModelingCmdReq {
        #[doc = "Which command to submit to the Kittycad engine."]
        cmd: ModelingCmd,
        #[doc = "ID of command being submitted."]
        cmd_id: uuid::Uuid,
    },
    #[serde(rename = "ping")]
    Ping {},
    #[serde(rename = "metrics_response")]
    MetricsResponse {
        #[doc = "Collected metrics from the Client's end of the engine connection."]
        metrics: ClientMetrics,
    },
}

#[doc = "Websocket responses can either be successful or unsuccessful. Slightly different schemas \
         in either case."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WebSocketResponse {
    #[doc = "Which request this is a response to. If the request was a modeling command, this is \
             the modeling command ID. If no request ID was sent, this will be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<uuid::Uuid>,
    #[doc = "The data sent with a successful response. This will be flattened into a 'type' and \
             'data' field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resp: Option<OkWebSocketResponseData>,
    #[doc = "Always false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[doc = "The errors that occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ApiError>>,
}

impl std::fmt::Display for WebSocketResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WebSocketResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(request_id) = &self.request_id {
                format!("{:?}", request_id).into()
            } else {
                String::new().into()
            },
            if let Some(resp) = &self.resp {
                format!("{:?}", resp).into()
            } else {
                String::new().into()
            },
            if let Some(success) = &self.success {
                format!("{:?}", success).into()
            } else {
                String::new().into()
            },
            if let Some(errors) = &self.errors {
                format!("{:?}", errors).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "request_id".into(),
            "resp".into(),
            "success".into(),
            "errors".into(),
        ]
    }
}
