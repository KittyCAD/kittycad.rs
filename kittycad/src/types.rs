#![doc = r" This module contains the generated types for the library."]
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

pub mod error {
    #![doc = " Error methods."]
    #[doc = " Error produced by generated client methods."]
    pub enum Error {
        #[doc = " The request did not conform to API requirements."]
        InvalidRequest(String),
        #[doc = " A server error either due to the data, or with the connection."]
        CommunicationError(reqwest::Error),
        #[doc = " An expected response whose deserialization failed."]
        SerdeError {
            #[doc = " The error."]
            error: format_serde_error::SerdeError,
            #[doc = " The response status."]
            status: reqwest::StatusCode,
        },
        #[doc = " An expected error response."]
        InvalidResponsePayload {
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
                Error::CommunicationError(e) => e.status(),
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

    impl From<reqwest::Error> for Error {
        fn from(e: reqwest::Error) -> Self {
            Self::CommunicationError(e)
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::InvalidRequest(s) => {
                    write!(f, "Invalid Request: {}", s)
                }
                Error::CommunicationError(e) => {
                    write!(f, "Communication Error: {}", e)
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
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum AccountProvider {
    #[serde(rename = "google")]
    #[display("google")]
    Google,
    #[serde(rename = "github")]
    #[display("github")]
    Github,
}

#[doc = "An address."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Address {
    #[doc = "The city component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The country component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The time and date the address was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The unique identifier of the address."]
    pub id: uuid::Uuid,
    #[doc = "The state component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The first street component."]
    #[serde(rename = "street1", default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[doc = "The second street component."]
    #[serde(rename = "street2", default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[doc = "The time and date the address was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID that this address belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The zip component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Address {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(city) = &self.city {
                format!("{:?}", city)
            } else {
                String::new()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            format!("{:?}", self.id),
            if let Some(state) = &self.state {
                format!("{:?}", state)
            } else {
                String::new()
            },
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1)
            } else {
                String::new()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2)
            } else {
                String::new()
            },
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "city".to_string(),
            "country".to_string(),
            "created_at".to_string(),
            "id".to_string(),
            "state".to_string(),
            "street_1".to_string(),
            "street_2".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
            "zip".to_string(),
        ]
    }
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

impl tabled::Tabled for ApiCallQueryGroup {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.count), self.query.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["count".to_string(), "query".to_string()]
    }
}

#[doc = "The field of an API call to group by."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum ApiCallQueryGroupBy {
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[serde(rename = "method")]
    #[display("method")]
    Method,
    #[serde(rename = "endpoint")]
    #[display("endpoint")]
    Endpoint,
    #[serde(rename = "user_id")]
    #[display("user_id")]
    UserId,
    #[serde(rename = "origin")]
    #[display("origin")]
    Origin,
    #[serde(rename = "ip_address")]
    #[display("ip_address")]
    IpAddress,
}

#[doc = "The status of an async API call."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum ApiCallStatus {
    Queued,
    Uploaded,
    #[serde(rename = "In Progress")]
    #[display("In Progress")]
    InProgress,
    Completed,
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
    pub price: Option<f64>,
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

impl tabled::Tabled for ApiCallWithPrice {
    const LENGTH: usize = 21;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(duration) = &self.duration {
                format!("{:?}", duration)
            } else {
                String::new()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email)
            } else {
                String::new()
            },
            if let Some(endpoint) = &self.endpoint {
                format!("{:?}", endpoint)
            } else {
                String::new()
            },
            format!("{:?}", self.id),
            if let Some(ip_address) = &self.ip_address {
                format!("{:?}", ip_address)
            } else {
                String::new()
            },
            format!("{:?}", self.method),
            if let Some(minutes) = &self.minutes {
                format!("{:?}", minutes)
            } else {
                String::new()
            },
            if let Some(origin) = &self.origin {
                format!("{:?}", origin)
            } else {
                String::new()
            },
            if let Some(price) = &self.price {
                format!("{:?}", price)
            } else {
                String::new()
            },
            if let Some(request_body) = &self.request_body {
                format!("{:?}", request_body)
            } else {
                String::new()
            },
            if let Some(request_query_params) = &self.request_query_params {
                format!("{:?}", request_query_params)
            } else {
                String::new()
            },
            if let Some(response_body) = &self.response_body {
                format!("{:?}", response_body)
            } else {
                String::new()
            },
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at)
            } else {
                String::new()
            },
            if let Some(status_code) = &self.status_code {
                format!("{:?}", status_code)
            } else {
                String::new()
            },
            if let Some(stripe_invoice_item_id) = &self.stripe_invoice_item_id {
                format!("{:?}", stripe_invoice_item_id)
            } else {
                String::new()
            },
            format!("{:?}", self.token),
            format!("{:?}", self.updated_at),
            self.user_agent.clone(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "completed_at".to_string(),
            "created_at".to_string(),
            "duration".to_string(),
            "email".to_string(),
            "endpoint".to_string(),
            "id".to_string(),
            "ip_address".to_string(),
            "method".to_string(),
            "minutes".to_string(),
            "origin".to_string(),
            "price".to_string(),
            "request_body".to_string(),
            "request_query_params".to_string(),
            "response_body".to_string(),
            "started_at".to_string(),
            "status_code".to_string(),
            "stripe_invoice_item_id".to_string(),
            "token".to_string(),
            "updated_at".to_string(),
            "user_agent".to_string(),
            "user_id".to_string(),
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

impl tabled::Tabled for ApiCallWithPriceResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.items),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["items".to_string(), "next_page".to_string()]
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

impl tabled::Tabled for ApiToken {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.created_at),
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            format!("{:?}", self.is_valid),
            format!("{:?}", self.token),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "created_at".to_string(),
            "id".to_string(),
            "is_valid".to_string(),
            "token".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
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

impl tabled::Tabled for ApiTokenResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.items),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["items".to_string(), "next_page".to_string()]
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

impl tabled::Tabled for AppClientInfo {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(url) = &self.url {
            format!("{:?}", url)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["url".to_string()]
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

impl tabled::Tabled for AsyncApiCall {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(error) = &self.error {
                format!("{:?}", error)
            } else {
                String::new()
            },
            format!("{:?}", self.id),
            if let Some(input) = &self.input {
                format!("{:?}", input)
            } else {
                String::new()
            },
            if let Some(output) = &self.output {
                format!("{:?}", output)
            } else {
                String::new()
            },
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at)
            } else {
                String::new()
            },
            format!("{:?}", self.status),
            format!("{:?}", self.type_),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
            if let Some(worker) = &self.worker {
                format!("{:?}", worker)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "completed_at".to_string(),
            "created_at".to_string(),
            "error".to_string(),
            "id".to_string(),
            "input".to_string(),
            "output".to_string(),
            "started_at".to_string(),
            "status".to_string(),
            "type_".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
            "worker".to_string(),
        ]
    }
}

#[doc = "The output from the async API call."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
)]
#[serde(tag = "type")]
pub enum AsyncApiCallOutput {
    FileConversion(FileConversion),
    FileMass(FileMass),
    FileVolume(FileVolume),
    FileDensity(FileDensity),
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

impl tabled::Tabled for AsyncApiCallResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.items),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["items".to_string(), "next_page".to_string()]
    }
}

#[doc = "The type of async API call."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum AsyncApiCallType {
    FileConversion,
    FileVolume,
    FileMass,
    FileDensity,
}

#[doc = "The billing information for payments."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BillingInfo {
    #[doc = "The address of the customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
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

impl tabled::Tabled for BillingInfo {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(address) = &self.address {
                format!("{:?}", address)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.phone),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "address".to_string(),
            "name".to_string(),
            "phone".to_string(),
        ]
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

impl tabled::Tabled for CacheMetadata {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.ok)]
    }

    fn headers() -> Vec<String> {
        vec!["ok".to_string()]
    }
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

impl tabled::Tabled for CardDetails {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(brand) = &self.brand {
                format!("{:?}", brand)
            } else {
                String::new()
            },
            if let Some(checks) = &self.checks {
                format!("{:?}", checks)
            } else {
                String::new()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country)
            } else {
                String::new()
            },
            if let Some(exp_month) = &self.exp_month {
                format!("{:?}", exp_month)
            } else {
                String::new()
            },
            if let Some(exp_year) = &self.exp_year {
                format!("{:?}", exp_year)
            } else {
                String::new()
            },
            if let Some(fingerprint) = &self.fingerprint {
                format!("{:?}", fingerprint)
            } else {
                String::new()
            },
            if let Some(funding) = &self.funding {
                format!("{:?}", funding)
            } else {
                String::new()
            },
            if let Some(last_4) = &self.last_4 {
                format!("{:?}", last_4)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "brand".to_string(),
            "checks".to_string(),
            "country".to_string(),
            "exp_month".to_string(),
            "exp_year".to_string(),
            "fingerprint".to_string(),
            "funding".to_string(),
            "last_4".to_string(),
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
    pub addr: Option<std::net::IpAddr>,
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

impl tabled::Tabled for Cluster {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(addr) = &self.addr {
                format!("{:?}", addr)
            } else {
                String::new()
            },
            if let Some(auth_timeout) = &self.auth_timeout {
                format!("{:?}", auth_timeout)
            } else {
                String::new()
            },
            if let Some(cluster_port) = &self.cluster_port {
                format!("{:?}", cluster_port)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(tls_timeout) = &self.tls_timeout {
                format!("{:?}", tls_timeout)
            } else {
                String::new()
            },
            if let Some(urls) = &self.urls {
                format!("{:?}", urls)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "addr".to_string(),
            "auth_timeout".to_string(),
            "cluster_port".to_string(),
            "name".to_string(),
            "tls_timeout".to_string(),
            "urls".to_string(),
        ]
    }
}

#[doc = "The language code is written in."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum CodeLanguage {
    #[serde(rename = "go")]
    #[display("go")]
    Go,
    #[serde(rename = "python")]
    #[display("python")]
    Python,
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

impl tabled::Tabled for CodeOutput {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(output_files) = &self.output_files {
                format!("{:?}", output_files)
            } else {
                String::new()
            },
            if let Some(stderr) = &self.stderr {
                format!("{:?}", stderr)
            } else {
                String::new()
            },
            if let Some(stdout) = &self.stdout {
                format!("{:?}", stdout)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "output_files".to_string(),
            "stderr".to_string(),
            "stdout".to_string(),
        ]
    }
}

#[doc = "Commit holds the Git-commit (SHA1) that a binary was built from, as reported in the \
         version-string of external tools, such as `containerd`, or `runC`."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Commit {
    #[doc = "Commit ID of external tool expected by dockerd as set at build time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
    #[doc = "Actual commit ID of external tool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl std::fmt::Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Commit {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(expected) = &self.expected {
                format!("{:?}", expected)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["expected".to_string(), "id".to_string()]
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

impl tabled::Tabled for Connection {
    const LENGTH: usize = 46;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(auth_timeout) = &self.auth_timeout {
                format!("{:?}", auth_timeout)
            } else {
                String::new()
            },
            if let Some(cluster) = &self.cluster {
                format!("{:?}", cluster)
            } else {
                String::new()
            },
            format!("{:?}", self.config_load_time),
            if let Some(connections) = &self.connections {
                format!("{:?}", connections)
            } else {
                String::new()
            },
            if let Some(cores) = &self.cores {
                format!("{:?}", cores)
            } else {
                String::new()
            },
            if let Some(cpu) = &self.cpu {
                format!("{:?}", cpu)
            } else {
                String::new()
            },
            if let Some(gateway) = &self.gateway {
                format!("{:?}", gateway)
            } else {
                String::new()
            },
            if let Some(git_commit) = &self.git_commit {
                format!("{:?}", git_commit)
            } else {
                String::new()
            },
            if let Some(go) = &self.go {
                format!("{:?}", go)
            } else {
                String::new()
            },
            if let Some(gomaxprocs) = &self.gomaxprocs {
                format!("{:?}", gomaxprocs)
            } else {
                String::new()
            },
            format!("{:?}", self.host),
            if let Some(http_base_path) = &self.http_base_path {
                format!("{:?}", http_base_path)
            } else {
                String::new()
            },
            if let Some(http_host) = &self.http_host {
                format!("{:?}", http_host)
            } else {
                String::new()
            },
            if let Some(http_port) = &self.http_port {
                format!("{:?}", http_port)
            } else {
                String::new()
            },
            format!("{:?}", self.http_req_stats),
            if let Some(https_port) = &self.https_port {
                format!("{:?}", https_port)
            } else {
                String::new()
            },
            if let Some(in_bytes) = &self.in_bytes {
                format!("{:?}", in_bytes)
            } else {
                String::new()
            },
            if let Some(in_msgs) = &self.in_msgs {
                format!("{:?}", in_msgs)
            } else {
                String::new()
            },
            if let Some(jetstream) = &self.jetstream {
                format!("{:?}", jetstream)
            } else {
                String::new()
            },
            if let Some(leaf) = &self.leaf {
                format!("{:?}", leaf)
            } else {
                String::new()
            },
            if let Some(leafnodes) = &self.leafnodes {
                format!("{:?}", leafnodes)
            } else {
                String::new()
            },
            if let Some(max_connections) = &self.max_connections {
                format!("{:?}", max_connections)
            } else {
                String::new()
            },
            if let Some(max_control_line) = &self.max_control_line {
                format!("{:?}", max_control_line)
            } else {
                String::new()
            },
            if let Some(max_payload) = &self.max_payload {
                format!("{:?}", max_payload)
            } else {
                String::new()
            },
            if let Some(max_pending) = &self.max_pending {
                format!("{:?}", max_pending)
            } else {
                String::new()
            },
            if let Some(mem) = &self.mem {
                format!("{:?}", mem)
            } else {
                String::new()
            },
            format!("{:?}", self.now),
            if let Some(out_bytes) = &self.out_bytes {
                format!("{:?}", out_bytes)
            } else {
                String::new()
            },
            if let Some(out_msgs) = &self.out_msgs {
                format!("{:?}", out_msgs)
            } else {
                String::new()
            },
            if let Some(ping_interval) = &self.ping_interval {
                format!("{:?}", ping_interval)
            } else {
                String::new()
            },
            if let Some(ping_max) = &self.ping_max {
                format!("{:?}", ping_max)
            } else {
                String::new()
            },
            if let Some(port) = &self.port {
                format!("{:?}", port)
            } else {
                String::new()
            },
            if let Some(proto) = &self.proto {
                format!("{:?}", proto)
            } else {
                String::new()
            },
            if let Some(remotes) = &self.remotes {
                format!("{:?}", remotes)
            } else {
                String::new()
            },
            if let Some(routes) = &self.routes {
                format!("{:?}", routes)
            } else {
                String::new()
            },
            if let Some(server_id) = &self.server_id {
                format!("{:?}", server_id)
            } else {
                String::new()
            },
            if let Some(server_name) = &self.server_name {
                format!("{:?}", server_name)
            } else {
                String::new()
            },
            if let Some(slow_consumers) = &self.slow_consumers {
                format!("{:?}", slow_consumers)
            } else {
                String::new()
            },
            format!("{:?}", self.start),
            if let Some(subscriptions) = &self.subscriptions {
                format!("{:?}", subscriptions)
            } else {
                String::new()
            },
            if let Some(system_account) = &self.system_account {
                format!("{:?}", system_account)
            } else {
                String::new()
            },
            if let Some(tls_timeout) = &self.tls_timeout {
                format!("{:?}", tls_timeout)
            } else {
                String::new()
            },
            if let Some(total_connections) = &self.total_connections {
                format!("{:?}", total_connections)
            } else {
                String::new()
            },
            if let Some(uptime) = &self.uptime {
                format!("{:?}", uptime)
            } else {
                String::new()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version)
            } else {
                String::new()
            },
            if let Some(write_deadline) = &self.write_deadline {
                format!("{:?}", write_deadline)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "auth_timeout".to_string(),
            "cluster".to_string(),
            "config_load_time".to_string(),
            "connections".to_string(),
            "cores".to_string(),
            "cpu".to_string(),
            "gateway".to_string(),
            "git_commit".to_string(),
            "go".to_string(),
            "gomaxprocs".to_string(),
            "host".to_string(),
            "http_base_path".to_string(),
            "http_host".to_string(),
            "http_port".to_string(),
            "http_req_stats".to_string(),
            "https_port".to_string(),
            "in_bytes".to_string(),
            "in_msgs".to_string(),
            "jetstream".to_string(),
            "leaf".to_string(),
            "leafnodes".to_string(),
            "max_connections".to_string(),
            "max_control_line".to_string(),
            "max_payload".to_string(),
            "max_pending".to_string(),
            "mem".to_string(),
            "now".to_string(),
            "out_bytes".to_string(),
            "out_msgs".to_string(),
            "ping_interval".to_string(),
            "ping_max".to_string(),
            "port".to_string(),
            "proto".to_string(),
            "remotes".to_string(),
            "routes".to_string(),
            "server_id".to_string(),
            "server_name".to_string(),
            "slow_consumers".to_string(),
            "start".to_string(),
            "subscriptions".to_string(),
            "system_account".to_string(),
            "tls_timeout".to_string(),
            "total_connections".to_string(),
            "uptime".to_string(),
            "version".to_string(),
            "write_deadline".to_string(),
        ]
    }
}

#[doc = "Supported set of sort modes for scanning by created_at only.\n\nCurrently, we only \
         support scanning in ascending order."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum CreatedAtSortMode {
    #[serde(rename = "created-at-ascending")]
    #[display("created-at-ascending")]
    CreatedAtAscending,
    #[serde(rename = "created-at-descending")]
    #[display("created-at-descending")]
    CreatedAtDescending,
}

#[doc = "Currency is the list of supported currencies.\n\nFor more details see <https://support.stripe.com/questions/which-currencies-does-stripe-support>."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Currency {
    #[serde(rename = "aed")]
    #[display("aed")]
    Aed,
    #[serde(rename = "afn")]
    #[display("afn")]
    Afn,
    #[serde(rename = "all")]
    #[display("all")]
    All,
    #[serde(rename = "amd")]
    #[display("amd")]
    Amd,
    #[serde(rename = "ang")]
    #[display("ang")]
    Ang,
    #[serde(rename = "aoa")]
    #[display("aoa")]
    Aoa,
    #[serde(rename = "ars")]
    #[display("ars")]
    Ars,
    #[serde(rename = "aud")]
    #[display("aud")]
    Aud,
    #[serde(rename = "awg")]
    #[display("awg")]
    Awg,
    #[serde(rename = "azn")]
    #[display("azn")]
    Azn,
    #[serde(rename = "bam")]
    #[display("bam")]
    Bam,
    #[serde(rename = "bbd")]
    #[display("bbd")]
    Bbd,
    #[serde(rename = "bdt")]
    #[display("bdt")]
    Bdt,
    #[serde(rename = "bgn")]
    #[display("bgn")]
    Bgn,
    #[serde(rename = "bif")]
    #[display("bif")]
    Bif,
    #[serde(rename = "bmd")]
    #[display("bmd")]
    Bmd,
    #[serde(rename = "bnd")]
    #[display("bnd")]
    Bnd,
    #[serde(rename = "bob")]
    #[display("bob")]
    Bob,
    #[serde(rename = "brl")]
    #[display("brl")]
    Brl,
    #[serde(rename = "bsd")]
    #[display("bsd")]
    Bsd,
    #[serde(rename = "bwp")]
    #[display("bwp")]
    Bwp,
    #[serde(rename = "bzd")]
    #[display("bzd")]
    Bzd,
    #[serde(rename = "cad")]
    #[display("cad")]
    Cad,
    #[serde(rename = "cdf")]
    #[display("cdf")]
    Cdf,
    #[serde(rename = "chf")]
    #[display("chf")]
    Chf,
    #[serde(rename = "clp")]
    #[display("clp")]
    Clp,
    #[serde(rename = "cny")]
    #[display("cny")]
    Cny,
    #[serde(rename = "cop")]
    #[display("cop")]
    Cop,
    #[serde(rename = "crc")]
    #[display("crc")]
    Crc,
    #[serde(rename = "cve")]
    #[display("cve")]
    Cve,
    #[serde(rename = "czk")]
    #[display("czk")]
    Czk,
    #[serde(rename = "djf")]
    #[display("djf")]
    Djf,
    #[serde(rename = "dkk")]
    #[display("dkk")]
    Dkk,
    #[serde(rename = "dop")]
    #[display("dop")]
    Dop,
    #[serde(rename = "dzd")]
    #[display("dzd")]
    Dzd,
    #[serde(rename = "eek")]
    #[display("eek")]
    Eek,
    #[serde(rename = "egp")]
    #[display("egp")]
    Egp,
    #[serde(rename = "etb")]
    #[display("etb")]
    Etb,
    #[serde(rename = "eur")]
    #[display("eur")]
    Eur,
    #[serde(rename = "fjd")]
    #[display("fjd")]
    Fjd,
    #[serde(rename = "fkp")]
    #[display("fkp")]
    Fkp,
    #[serde(rename = "gbp")]
    #[display("gbp")]
    Gbp,
    #[serde(rename = "gel")]
    #[display("gel")]
    Gel,
    #[serde(rename = "gip")]
    #[display("gip")]
    Gip,
    #[serde(rename = "gmd")]
    #[display("gmd")]
    Gmd,
    #[serde(rename = "gnf")]
    #[display("gnf")]
    Gnf,
    #[serde(rename = "gtq")]
    #[display("gtq")]
    Gtq,
    #[serde(rename = "gyd")]
    #[display("gyd")]
    Gyd,
    #[serde(rename = "hkd")]
    #[display("hkd")]
    Hkd,
    #[serde(rename = "hnl")]
    #[display("hnl")]
    Hnl,
    #[serde(rename = "hrk")]
    #[display("hrk")]
    Hrk,
    #[serde(rename = "htg")]
    #[display("htg")]
    Htg,
    #[serde(rename = "huf")]
    #[display("huf")]
    Huf,
    #[serde(rename = "idr")]
    #[display("idr")]
    Idr,
    #[serde(rename = "ils")]
    #[display("ils")]
    Ils,
    #[serde(rename = "inr")]
    #[display("inr")]
    Inr,
    #[serde(rename = "isk")]
    #[display("isk")]
    Isk,
    #[serde(rename = "jmd")]
    #[display("jmd")]
    Jmd,
    #[serde(rename = "jpy")]
    #[display("jpy")]
    Jpy,
    #[serde(rename = "kes")]
    #[display("kes")]
    Kes,
    #[serde(rename = "kgs")]
    #[display("kgs")]
    Kgs,
    #[serde(rename = "khr")]
    #[display("khr")]
    Khr,
    #[serde(rename = "kmf")]
    #[display("kmf")]
    Kmf,
    #[serde(rename = "krw")]
    #[display("krw")]
    Krw,
    #[serde(rename = "kyd")]
    #[display("kyd")]
    Kyd,
    #[serde(rename = "kzt")]
    #[display("kzt")]
    Kzt,
    #[serde(rename = "lak")]
    #[display("lak")]
    Lak,
    #[serde(rename = "lbp")]
    #[display("lbp")]
    Lbp,
    #[serde(rename = "lkr")]
    #[display("lkr")]
    Lkr,
    #[serde(rename = "lrd")]
    #[display("lrd")]
    Lrd,
    #[serde(rename = "lsl")]
    #[display("lsl")]
    Lsl,
    #[serde(rename = "ltl")]
    #[display("ltl")]
    Ltl,
    #[serde(rename = "lvl")]
    #[display("lvl")]
    Lvl,
    #[serde(rename = "mad")]
    #[display("mad")]
    Mad,
    #[serde(rename = "mdl")]
    #[display("mdl")]
    Mdl,
    #[serde(rename = "mga")]
    #[display("mga")]
    Mga,
    #[serde(rename = "mkd")]
    #[display("mkd")]
    Mkd,
    #[serde(rename = "mnt")]
    #[display("mnt")]
    Mnt,
    #[serde(rename = "mop")]
    #[display("mop")]
    Mop,
    #[serde(rename = "mro")]
    #[display("mro")]
    Mro,
    #[serde(rename = "mur")]
    #[display("mur")]
    Mur,
    #[serde(rename = "mvr")]
    #[display("mvr")]
    Mvr,
    #[serde(rename = "mwk")]
    #[display("mwk")]
    Mwk,
    #[serde(rename = "mxn")]
    #[display("mxn")]
    Mxn,
    #[serde(rename = "myr")]
    #[display("myr")]
    Myr,
    #[serde(rename = "mzn")]
    #[display("mzn")]
    Mzn,
    #[serde(rename = "nad")]
    #[display("nad")]
    Nad,
    #[serde(rename = "ngn")]
    #[display("ngn")]
    Ngn,
    #[serde(rename = "nio")]
    #[display("nio")]
    Nio,
    #[serde(rename = "nok")]
    #[display("nok")]
    Nok,
    #[serde(rename = "npr")]
    #[display("npr")]
    Npr,
    #[serde(rename = "nzd")]
    #[display("nzd")]
    Nzd,
    #[serde(rename = "pab")]
    #[display("pab")]
    Pab,
    #[serde(rename = "pen")]
    #[display("pen")]
    Pen,
    #[serde(rename = "pgk")]
    #[display("pgk")]
    Pgk,
    #[serde(rename = "php")]
    #[display("php")]
    Php,
    #[serde(rename = "pkr")]
    #[display("pkr")]
    Pkr,
    #[serde(rename = "pln")]
    #[display("pln")]
    Pln,
    #[serde(rename = "pyg")]
    #[display("pyg")]
    Pyg,
    #[serde(rename = "qar")]
    #[display("qar")]
    Qar,
    #[serde(rename = "ron")]
    #[display("ron")]
    Ron,
    #[serde(rename = "rsd")]
    #[display("rsd")]
    Rsd,
    #[serde(rename = "rub")]
    #[display("rub")]
    Rub,
    #[serde(rename = "rwf")]
    #[display("rwf")]
    Rwf,
    #[serde(rename = "sar")]
    #[display("sar")]
    Sar,
    #[serde(rename = "sbd")]
    #[display("sbd")]
    Sbd,
    #[serde(rename = "scr")]
    #[display("scr")]
    Scr,
    #[serde(rename = "sek")]
    #[display("sek")]
    Sek,
    #[serde(rename = "sgd")]
    #[display("sgd")]
    Sgd,
    #[serde(rename = "shp")]
    #[display("shp")]
    Shp,
    #[serde(rename = "sll")]
    #[display("sll")]
    Sll,
    #[serde(rename = "sos")]
    #[display("sos")]
    Sos,
    #[serde(rename = "srd")]
    #[display("srd")]
    Srd,
    #[serde(rename = "std")]
    #[display("std")]
    Std,
    #[serde(rename = "svc")]
    #[display("svc")]
    Svc,
    #[serde(rename = "szl")]
    #[display("szl")]
    Szl,
    #[serde(rename = "thb")]
    #[display("thb")]
    Thb,
    #[serde(rename = "tjs")]
    #[display("tjs")]
    Tjs,
    #[serde(rename = "top")]
    #[display("top")]
    Top,
    #[serde(rename = "try")]
    #[display("try")]
    Try,
    #[serde(rename = "ttd")]
    #[display("ttd")]
    Ttd,
    #[serde(rename = "twd")]
    #[display("twd")]
    Twd,
    #[serde(rename = "tzs")]
    #[display("tzs")]
    Tzs,
    #[serde(rename = "uah")]
    #[display("uah")]
    Uah,
    #[serde(rename = "ugx")]
    #[display("ugx")]
    Ugx,
    #[serde(rename = "usd")]
    #[display("usd")]
    Usd,
    #[serde(rename = "uyu")]
    #[display("uyu")]
    Uyu,
    #[serde(rename = "uzs")]
    #[display("uzs")]
    Uzs,
    #[serde(rename = "vef")]
    #[display("vef")]
    Vef,
    #[serde(rename = "vnd")]
    #[display("vnd")]
    Vnd,
    #[serde(rename = "vuv")]
    #[display("vuv")]
    Vuv,
    #[serde(rename = "wst")]
    #[display("wst")]
    Wst,
    #[serde(rename = "xaf")]
    #[display("xaf")]
    Xaf,
    #[serde(rename = "xcd")]
    #[display("xcd")]
    Xcd,
    #[serde(rename = "xof")]
    #[display("xof")]
    Xof,
    #[serde(rename = "xpf")]
    #[display("xpf")]
    Xpf,
    #[serde(rename = "yer")]
    #[display("yer")]
    Yer,
    #[serde(rename = "zar")]
    #[display("zar")]
    Zar,
    #[serde(rename = "zmw")]
    #[display("zmw")]
    Zmw,
}

#[doc = "The resource representing a payment \"Customer\"."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Customer {
    #[doc = "The customer's address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[doc = "Current balance, if any, being stored on the customer in the payments service.\n\nIf \
             negative, the customer has credit to apply to their next invoice. If positive, the \
             customer has an amount owed that will be added to their next invoice. The balance \
             does not refer to any unpaid invoices; it solely takes into account amounts that \
             have yet to be successfully applied to any invoice. This balance is only taken into \
             account as invoices are finalized."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    #[doc = "Time at which the object was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "Three-letter ISO code for the currency the customer can be charged in for recurring \
             billing purposes."]
    pub currency: Currency,
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

impl tabled::Tabled for Customer {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(address) = &self.address {
                format!("{:?}", address)
            } else {
                String::new()
            },
            if let Some(balance) = &self.balance {
                format!("{:?}", balance)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            format!("{:?}", self.currency),
            if let Some(delinquent) = &self.delinquent {
                format!("{:?}", delinquent)
            } else {
                String::new()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.phone),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "address".to_string(),
            "balance".to_string(),
            "created_at".to_string(),
            "currency".to_string(),
            "delinquent".to_string(),
            "email".to_string(),
            "id".to_string(),
            "metadata".to_string(),
            "name".to_string(),
            "phone".to_string(),
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
    pub monthly_credits_remaining: f64,
    #[doc = "The amount of pre-pay cash remaining in the balance. This number goes down as the \
             user uses their pre-paid credits. The reason we track this amount is if a user ever \
             wants to withdraw their pre-pay cash, we can use this amount to determine how much \
             to give them. Say a user has $100 in pre-paid cash, their bill is worth, $50 after \
             subtracting any other credits (like monthly etc.) Their bill is $50, their pre-pay \
             cash remaining will be subtracted by 50 to pay the bill and their \
             `pre_pay_credits_remaining` will be subtracted by 50 to pay the bill. This way if \
             they want to withdraw money after, they can only withdraw $50 since that is the \
             amount of cash they have remaining."]
    pub pre_pay_cash_remaining: f64,
    #[doc = "The amount of credits remaining in the balance. This is typically the amount of cash \
             * some multiplier they get for pre-paying their account. This number lowers every \
             time a bill is paid with the balance. This number increases every time a user adds \
             funds to their balance. This may be through a subscription or a one off payment."]
    pub pre_pay_credits_remaining: f64,
    #[doc = "This includes any outstanding, draft, or open invoices and any pending invoice \
             items. This does not include any credits the user has on their account."]
    pub total_due: f64,
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

impl tabled::Tabled for CustomerBalance {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.created_at),
            format!("{:?}", self.id),
            format!("{:?}", self.monthly_credits_remaining),
            format!("{:?}", self.pre_pay_cash_remaining),
            format!("{:?}", self.pre_pay_credits_remaining),
            format!("{:?}", self.total_due),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "created_at".to_string(),
            "id".to_string(),
            "monthly_credits_remaining".to_string(),
            "pre_pay_cash_remaining".to_string(),
            "pre_pay_credits_remaining".to_string(),
            "total_due".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
        ]
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

impl tabled::Tabled for DeviceAccessTokenRequestForm {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.client_id),
            format!("{:?}", self.device_code),
            format!("{:?}", self.grant_type),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "client_id".to_string(),
            "device_code".to_string(),
            "grant_type".to_string(),
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

impl tabled::Tabled for DeviceAuthRequestForm {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.client_id)]
    }

    fn headers() -> Vec<String> {
        vec!["client_id".to_string()]
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

impl tabled::Tabled for DeviceAuthVerifyParams {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![self.user_code.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["user_code".to_string()]
    }
}

#[doc = "Docker system info."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DockerSystemInfo {
    #[doc = "Hardware architecture of the host, as returned by the Go runtime (`GOARCH`).  A full list of possible values can be found in the [Go documentation](https://golang.org/doc/install/source#environment)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[doc = "Indicates if `bridge-nf-call-ip6tables` is available on the host."]
    #[serde(
        rename = "bridge_nf_ip6tables",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bridge_nf_ip_6tables: Option<bool>,
    #[doc = "Indicates if `bridge-nf-call-iptables` is available on the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge_nf_iptables: Option<bool>,
    #[doc = "The driver to use for managing cgroups."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cgroup_driver: Option<SystemInfoCgroupDriverEnum>,
    #[doc = "The version of the cgroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cgroup_version: Option<SystemInfoCgroupVersionEnum>,
    #[doc = "The network endpoint that the Engine advertises for the purpose of node discovery. \
             ClusterAdvertise is a `host:port` combination on which the daemon is reachable by \
             other hosts.\n\n**Deprecated**: This field is only propagated when using standalone \
             Swarm mode, and overlay networking using an external k/v store. Overlay networks \
             with Swarm mode enabled use the built-in raft store, and this field will be empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_advertise: Option<String>,
    #[doc = "URL of the distributed storage backend.   The storage backend is used for multihost \
             networking (to store network and endpoint information) and by the node discovery \
             mechanism.\n\n**Deprecated**: This field is only propagated when using standalone \
             Swarm mode, and overlay networking using an external k/v store. Overlay networks \
             with Swarm mode enabled use the built-in raft store, and this field will be empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_store: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containerd_commit: Option<Commit>,
    #[doc = "Total number of containers on the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<i64>,
    #[doc = "Number of containers with status `\\\"paused\\\"`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers_paused: Option<i64>,
    #[doc = "Number of containers with status `\\\"running\\\"`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers_running: Option<i64>,
    #[doc = "Number of containers with status `\\\"stopped\\\"`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers_stopped: Option<i64>,
    #[doc = "Indicates if CPU CFS(Completely Fair Scheduler) period is supported by the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_period: Option<bool>,
    #[doc = "Indicates if CPU CFS(Completely Fair Scheduler) quota is supported by the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_quota: Option<bool>,
    #[doc = "Indicates if CPUsets (cpuset.cpus, cpuset.mems) are supported by the host.  See [cpuset(7)](https://www.kernel.org/doc/Documentation/cgroup-v1/cpusets.txt)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_set: Option<bool>,
    #[doc = "Indicates if CPU Shares limiting is supported by the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<bool>,
    #[doc = "Indicates if the daemon is running in debug-mode / with debug-level logging enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[doc = "List of custom default address pools for local networks, which can be specified in \
             the daemon.json file or dockerd option.  Example: a Base \\\"10.10.0.0/16\\\" with \
             Size 24 will define the set of 256 10.10.[0-255].0/24 address pools."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_address_pools: Option<Vec<SystemInfoDefaultAddressPools>>,
    #[doc = "Name of the default OCI runtime that is used when starting containers.  The default \
             can be overridden per-container at create time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_runtime: Option<String>,
    #[doc = "Root directory of persistent Docker state.  Defaults to `/var/lib/docker` on Linux, \
             and `C:\\\\ProgramData\\\\docker` on Windows."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker_root_dir: Option<String>,
    #[doc = "Name of the storage driver in use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[doc = "Information specific to the storage driver, provided as \\\"label\\\" / \
             \\\"value\\\" pairs.  This information is provided by the storage driver, and \
             formatted in a way consistent with the output of `docker info` on the command \
             line.\n\n**Note**: The information returned in this field, including the formatting \
             of values and labels, should not be considered stable, and may change without notice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver_status: Option<Vec<Vec<String>>>,
    #[doc = "Indicates if experimental features are enabled on the daemon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental_build: Option<bool>,
    #[doc = "HTTP-proxy configured for the daemon. This value is obtained from the [`HTTP_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<String>,
    #[doc = "HTTPS-proxy configured for the daemon. This value is obtained from the [`HTTPS_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_proxy: Option<String>,
    #[doc = "Unique identifier of the daemon.\n\n**Note**: The format of the ID itself is not \
             part of the API, and should not be considered stable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Total number of images on the host. Both _tagged_ and _untagged_ (dangling) images \
             are counted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<i64>,
    #[doc = "Address / URL of the index server that is used for image search, and as a default \
             for user authentication for Docker Hub and Docker Cloud."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_server_address: Option<String>,
    #[doc = "Name and, optional, path of the `docker-init` binary.  If the path is omitted, the \
             daemon searches the host's `$PATH` for the binary and uses the first result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init_binary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init_commit: Option<Commit>,
    #[doc = "Indicates IPv4 forwarding is enabled."]
    #[serde(
        rename = "ipv4_forwarding",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ipv_4_forwarding: Option<bool>,
    #[doc = "Represents the isolation technology to use as a default for containers. The \
             supported values are platform-specific.  If no isolation value is specified on \
             daemon start, on Windows client, the default is `hyperv`, and on Windows server, the \
             default is `process`.  This option is currently not used on other platforms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isolation: Option<SystemInfoIsolationEnum>,
    #[doc = "Indicates if the host has kernel memory limit support enabled.\n\n**Deprecated**: \
             This field is deprecated as the kernel 5.4 deprecated `kmem.limit_in_bytes`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<bool>,
    #[doc = "Indicates if the host has kernel memory TCP limit support enabled.  Kernel memory \
             TCP limits are not supported when using cgroups v2, which does not support the \
             corresponding `memory.kmem.tcp.limit_in_bytes` cgroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<bool>,
    #[doc = "Kernel version of the host.  On Linux, this information obtained from `uname`. On \
             Windows this information is queried from the \
             <kbd>HKEY_LOCAL_MACHINE\\\\\\\\SOFTWARE\\\\\\\\Microsoft\\\\\\\\Windows \
             NT\\\\\\\\CurrentVersion\\\\\\\\</kbd> registry value, for example _\\\"10.0 14393 \
             (14393.1198.amd64fre.rs1_release_sec.170427-1353)\\\"_."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    #[doc = "User-defined labels (key/value metadata) as set on the daemon.\n\n**Note**: When \
             part of a Swarm, nodes can both have _daemon_ labels, set through the daemon \
             configuration, and _node_ labels, set from a manager node in the Swarm. Node labels \
             are not included in this field. Node labels can be retrieved using the `/nodes/(id)` \
             endpoint on a manager node in the Swarm."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[doc = "Indicates if live restore is enabled.  If enabled, containers are kept running when \
             the daemon is shutdown or upon daemon start if running containers are detected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_restore_enabled: Option<bool>,
    #[doc = "The logging driver to use as a default for new containers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging_driver: Option<String>,
    #[doc = "Total amount of physical memory available on the host, in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem_total: Option<i64>,
    #[doc = "Indicates if the host has memory limit support enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<bool>,
    #[doc = "Number of event listeners subscribed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_events_listener: Option<i64>,
    #[doc = "The total number of file Descriptors in use by the daemon process.  This information \
             is only returned if debug-mode is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n_fd: Option<i64>,
    #[doc = "Hostname of the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The number of logical CPUs usable by the daemon.  The number of available CPUs is \
             checked by querying the operating system when the daemon starts. Changes to \
             operating system CPU allocation after the daemon is started are not reflected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ncpu: Option<i64>,
    #[doc = "Comma-separated list of domain extensions for which no proxy should be used. This value is obtained from the [`NO_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable.  Containers do not automatically inherit this configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_proxy: Option<String>,
    #[doc = "Indicates if OOM killer disable is supported on the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    #[doc = "Name of the host's operating system, for example: \\\"Ubuntu 16.04.2 LTS\\\" or \
             \\\"Windows Server 2016 Datacenter\\\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[doc = "Generic type of the operating system of the host, as returned by the Go runtime (`GOOS`).  Currently returned values are \\\"linux\\\" and \\\"windows\\\". A full list of possible values can be found in the [Go documentation](https://golang.org/doc/install/source#environment)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Version of the host's operating system\n\n**Note**: The information returned in this \
             field, including its very existence, and the formatting of values, should not be \
             considered stable, and may change without notice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Indicates if the host kernel has PID limit support enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<PluginsInfo>,
    #[doc = "Reports a summary of the product license on the daemon.  If a commercial license has \
             been applied to the daemon, information such as number of nodes, and expiration are \
             included."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry_config: Option<RegistryServiceConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runc_commit: Option<Commit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<std::collections::HashMap<String, Runtime>>,
    #[doc = "List of security features that are enabled on the daemon, such as apparmor, seccomp, \
             SELinux, user-namespaces (userns), and rootless.  Additional configuration options \
             for each security feature may be present, and are included as a comma-separated list \
             of key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_options: Option<Vec<String>>,
    #[doc = "Version string of the daemon. **Note**: the [standalone Swarm API](https://docs.docker.com/swarm/swarm-api/) returns the Swarm version instead of the daemon  version, for example `swarm/1.2.8`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    #[doc = "Indicates if the host has memory swap limit support enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap_limit: Option<bool>,
    #[doc = "The  number of goroutines that currently exist.  This information is only returned \
             if debug-mode is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_time: Option<String>,
    #[doc = "List of warnings / informational messages about missing features, or issues related \
             to the daemon configuration.  These messages can be printed by the client as \
             information to the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

impl std::fmt::Display for DockerSystemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for DockerSystemInfo {
    const LENGTH: usize = 60;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(architecture) = &self.architecture {
                format!("{:?}", architecture)
            } else {
                String::new()
            },
            if let Some(bridge_nf_ip_6tables) = &self.bridge_nf_ip_6tables {
                format!("{:?}", bridge_nf_ip_6tables)
            } else {
                String::new()
            },
            if let Some(bridge_nf_iptables) = &self.bridge_nf_iptables {
                format!("{:?}", bridge_nf_iptables)
            } else {
                String::new()
            },
            if let Some(cgroup_driver) = &self.cgroup_driver {
                format!("{:?}", cgroup_driver)
            } else {
                String::new()
            },
            if let Some(cgroup_version) = &self.cgroup_version {
                format!("{:?}", cgroup_version)
            } else {
                String::new()
            },
            if let Some(cluster_advertise) = &self.cluster_advertise {
                format!("{:?}", cluster_advertise)
            } else {
                String::new()
            },
            if let Some(cluster_store) = &self.cluster_store {
                format!("{:?}", cluster_store)
            } else {
                String::new()
            },
            if let Some(containerd_commit) = &self.containerd_commit {
                format!("{:?}", containerd_commit)
            } else {
                String::new()
            },
            if let Some(containers) = &self.containers {
                format!("{:?}", containers)
            } else {
                String::new()
            },
            if let Some(containers_paused) = &self.containers_paused {
                format!("{:?}", containers_paused)
            } else {
                String::new()
            },
            if let Some(containers_running) = &self.containers_running {
                format!("{:?}", containers_running)
            } else {
                String::new()
            },
            if let Some(containers_stopped) = &self.containers_stopped {
                format!("{:?}", containers_stopped)
            } else {
                String::new()
            },
            if let Some(cpu_cfs_period) = &self.cpu_cfs_period {
                format!("{:?}", cpu_cfs_period)
            } else {
                String::new()
            },
            if let Some(cpu_cfs_quota) = &self.cpu_cfs_quota {
                format!("{:?}", cpu_cfs_quota)
            } else {
                String::new()
            },
            if let Some(cpu_set) = &self.cpu_set {
                format!("{:?}", cpu_set)
            } else {
                String::new()
            },
            if let Some(cpu_shares) = &self.cpu_shares {
                format!("{:?}", cpu_shares)
            } else {
                String::new()
            },
            if let Some(debug) = &self.debug {
                format!("{:?}", debug)
            } else {
                String::new()
            },
            if let Some(default_address_pools) = &self.default_address_pools {
                format!("{:?}", default_address_pools)
            } else {
                String::new()
            },
            if let Some(default_runtime) = &self.default_runtime {
                format!("{:?}", default_runtime)
            } else {
                String::new()
            },
            if let Some(docker_root_dir) = &self.docker_root_dir {
                format!("{:?}", docker_root_dir)
            } else {
                String::new()
            },
            if let Some(driver) = &self.driver {
                format!("{:?}", driver)
            } else {
                String::new()
            },
            if let Some(driver_status) = &self.driver_status {
                format!("{:?}", driver_status)
            } else {
                String::new()
            },
            if let Some(experimental_build) = &self.experimental_build {
                format!("{:?}", experimental_build)
            } else {
                String::new()
            },
            if let Some(http_proxy) = &self.http_proxy {
                format!("{:?}", http_proxy)
            } else {
                String::new()
            },
            if let Some(https_proxy) = &self.https_proxy {
                format!("{:?}", https_proxy)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(images) = &self.images {
                format!("{:?}", images)
            } else {
                String::new()
            },
            if let Some(index_server_address) = &self.index_server_address {
                format!("{:?}", index_server_address)
            } else {
                String::new()
            },
            if let Some(init_binary) = &self.init_binary {
                format!("{:?}", init_binary)
            } else {
                String::new()
            },
            if let Some(init_commit) = &self.init_commit {
                format!("{:?}", init_commit)
            } else {
                String::new()
            },
            if let Some(ipv_4_forwarding) = &self.ipv_4_forwarding {
                format!("{:?}", ipv_4_forwarding)
            } else {
                String::new()
            },
            if let Some(isolation) = &self.isolation {
                format!("{:?}", isolation)
            } else {
                String::new()
            },
            if let Some(kernel_memory) = &self.kernel_memory {
                format!("{:?}", kernel_memory)
            } else {
                String::new()
            },
            if let Some(kernel_memory_tcp) = &self.kernel_memory_tcp {
                format!("{:?}", kernel_memory_tcp)
            } else {
                String::new()
            },
            if let Some(kernel_version) = &self.kernel_version {
                format!("{:?}", kernel_version)
            } else {
                String::new()
            },
            if let Some(labels) = &self.labels {
                format!("{:?}", labels)
            } else {
                String::new()
            },
            if let Some(live_restore_enabled) = &self.live_restore_enabled {
                format!("{:?}", live_restore_enabled)
            } else {
                String::new()
            },
            if let Some(logging_driver) = &self.logging_driver {
                format!("{:?}", logging_driver)
            } else {
                String::new()
            },
            if let Some(mem_total) = &self.mem_total {
                format!("{:?}", mem_total)
            } else {
                String::new()
            },
            if let Some(memory_limit) = &self.memory_limit {
                format!("{:?}", memory_limit)
            } else {
                String::new()
            },
            if let Some(n_events_listener) = &self.n_events_listener {
                format!("{:?}", n_events_listener)
            } else {
                String::new()
            },
            if let Some(n_fd) = &self.n_fd {
                format!("{:?}", n_fd)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(ncpu) = &self.ncpu {
                format!("{:?}", ncpu)
            } else {
                String::new()
            },
            if let Some(no_proxy) = &self.no_proxy {
                format!("{:?}", no_proxy)
            } else {
                String::new()
            },
            if let Some(oom_kill_disable) = &self.oom_kill_disable {
                format!("{:?}", oom_kill_disable)
            } else {
                String::new()
            },
            if let Some(operating_system) = &self.operating_system {
                format!("{:?}", operating_system)
            } else {
                String::new()
            },
            if let Some(os_type) = &self.os_type {
                format!("{:?}", os_type)
            } else {
                String::new()
            },
            if let Some(os_version) = &self.os_version {
                format!("{:?}", os_version)
            } else {
                String::new()
            },
            if let Some(pids_limit) = &self.pids_limit {
                format!("{:?}", pids_limit)
            } else {
                String::new()
            },
            if let Some(plugins) = &self.plugins {
                format!("{:?}", plugins)
            } else {
                String::new()
            },
            if let Some(product_license) = &self.product_license {
                format!("{:?}", product_license)
            } else {
                String::new()
            },
            if let Some(registry_config) = &self.registry_config {
                format!("{:?}", registry_config)
            } else {
                String::new()
            },
            if let Some(runc_commit) = &self.runc_commit {
                format!("{:?}", runc_commit)
            } else {
                String::new()
            },
            if let Some(runtimes) = &self.runtimes {
                format!("{:?}", runtimes)
            } else {
                String::new()
            },
            if let Some(security_options) = &self.security_options {
                format!("{:?}", security_options)
            } else {
                String::new()
            },
            if let Some(server_version) = &self.server_version {
                format!("{:?}", server_version)
            } else {
                String::new()
            },
            if let Some(swap_limit) = &self.swap_limit {
                format!("{:?}", swap_limit)
            } else {
                String::new()
            },
            if let Some(system_time) = &self.system_time {
                format!("{:?}", system_time)
            } else {
                String::new()
            },
            if let Some(warnings) = &self.warnings {
                format!("{:?}", warnings)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "architecture".to_string(),
            "bridge_nf_ip_6tables".to_string(),
            "bridge_nf_iptables".to_string(),
            "cgroup_driver".to_string(),
            "cgroup_version".to_string(),
            "cluster_advertise".to_string(),
            "cluster_store".to_string(),
            "containerd_commit".to_string(),
            "containers".to_string(),
            "containers_paused".to_string(),
            "containers_running".to_string(),
            "containers_stopped".to_string(),
            "cpu_cfs_period".to_string(),
            "cpu_cfs_quota".to_string(),
            "cpu_set".to_string(),
            "cpu_shares".to_string(),
            "debug".to_string(),
            "default_address_pools".to_string(),
            "default_runtime".to_string(),
            "docker_root_dir".to_string(),
            "driver".to_string(),
            "driver_status".to_string(),
            "experimental_build".to_string(),
            "http_proxy".to_string(),
            "https_proxy".to_string(),
            "id".to_string(),
            "images".to_string(),
            "index_server_address".to_string(),
            "init_binary".to_string(),
            "init_commit".to_string(),
            "ipv_4_forwarding".to_string(),
            "isolation".to_string(),
            "kernel_memory".to_string(),
            "kernel_memory_tcp".to_string(),
            "kernel_version".to_string(),
            "labels".to_string(),
            "live_restore_enabled".to_string(),
            "logging_driver".to_string(),
            "mem_total".to_string(),
            "memory_limit".to_string(),
            "n_events_listener".to_string(),
            "n_fd".to_string(),
            "name".to_string(),
            "ncpu".to_string(),
            "no_proxy".to_string(),
            "oom_kill_disable".to_string(),
            "operating_system".to_string(),
            "os_type".to_string(),
            "os_version".to_string(),
            "pids_limit".to_string(),
            "plugins".to_string(),
            "product_license".to_string(),
            "registry_config".to_string(),
            "runc_commit".to_string(),
            "runtimes".to_string(),
            "security_options".to_string(),
            "server_version".to_string(),
            "swap_limit".to_string(),
            "system_time".to_string(),
            "warnings".to_string(),
        ]
    }
}

#[doc = "The body of the form for email authentication."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmailAuthenticationForm {
    #[doc = "The URL to redirect back to after we have authenticated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<url::Url>,
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

impl tabled::Tabled for EmailAuthenticationForm {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(callback_url) = &self.callback_url {
                format!("{:?}", callback_url)
            } else {
                String::new()
            },
            self.email.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["callback_url".to_string(), "email".to_string()]
    }
}

#[doc = "Metadata about our currently running server.\n\nThis is mostly used for internal purposes \
         and debugging."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EngineMetadata {
    #[doc = "If any async job is currently running."]
    pub async_jobs_running: bool,
    #[doc = "Metadata about our cache."]
    pub cache: CacheMetadata,
    #[doc = "The environment we are running in."]
    pub environment: Environment,
    #[doc = "Metadata about our file system."]
    pub fs: FileSystemMetadata,
    #[doc = "The git hash of the server."]
    pub git_hash: String,
    #[doc = "Metadata about our pub-sub connection."]
    pub pubsub: Connection,
}

impl std::fmt::Display for EngineMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for EngineMetadata {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.async_jobs_running),
            format!("{:?}", self.cache),
            format!("{:?}", self.environment),
            format!("{:?}", self.fs),
            self.git_hash.clone(),
            format!("{:?}", self.pubsub),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "async_jobs_running".to_string(),
            "cache".to_string(),
            "environment".to_string(),
            "fs".to_string(),
            "git_hash".to_string(),
            "pubsub".to_string(),
        ]
    }
}

#[doc = "The environment the server is running in."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Environment {
    #[serde(rename = "DEVELOPMENT")]
    #[display("DEVELOPMENT")]
    Development,
    #[serde(rename = "PREVIEW")]
    #[display("PREVIEW")]
    Preview,
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

impl tabled::Tabled for Error {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(error_code) = &self.error_code {
                format!("{:?}", error_code)
            } else {
                String::new()
            },
            self.message.clone(),
            self.request_id.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "error_code".to_string(),
            "message".to_string(),
            "request_id".to_string(),
        ]
    }
}

#[doc = "Metadata about our currently running server.\n\nThis is mostly used for internal purposes \
         and debugging."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ExecutorMetadata {
    #[doc = "Information about the docker daemon."]
    pub docker_info: DockerSystemInfo,
    #[doc = "The environment we are running in."]
    pub environment: Environment,
    #[doc = "The git hash of the server."]
    pub git_hash: String,
}

impl std::fmt::Display for ExecutorMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ExecutorMetadata {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.docker_info),
            format!("{:?}", self.environment),
            self.git_hash.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "docker_info".to_string(),
            "environment".to_string(),
            "git_hash".to_string(),
        ]
    }
}

#[doc = "Extended user information.\n\nThis is mostly used for internal purposes. It returns a \
         mapping of the user's information, including that of our third party services we use for \
         users: MailChimp, Stripe, and Zendesk."]
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
    #[doc = "The user's GitHub handle."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[doc = "The unique identifier for the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The image avatar for the user. This is a URL."]
    pub image: url::Url,
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
    #[doc = "The user's Zendesk ID. This is mostly used for internal mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zendesk_id: Option<String>,
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

impl tabled::Tabled for ExtendedUser {
    const LENGTH: usize = 16;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(company) = &self.company {
                format!("{:?}", company)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(discord) = &self.discord {
                format!("{:?}", discord)
            } else {
                String::new()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email)
            } else {
                String::new()
            },
            if let Some(email_verified) = &self.email_verified {
                format!("{:?}", email_verified)
            } else {
                String::new()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name)
            } else {
                String::new()
            },
            if let Some(github) = &self.github {
                format!("{:?}", github)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            format!("{:?}", self.image),
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name)
            } else {
                String::new()
            },
            if let Some(mailchimp_id) = &self.mailchimp_id {
                format!("{:?}", mailchimp_id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.phone),
            if let Some(stripe_id) = &self.stripe_id {
                format!("{:?}", stripe_id)
            } else {
                String::new()
            },
            format!("{:?}", self.updated_at),
            if let Some(zendesk_id) = &self.zendesk_id {
                format!("{:?}", zendesk_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "company".to_string(),
            "created_at".to_string(),
            "discord".to_string(),
            "email".to_string(),
            "email_verified".to_string(),
            "first_name".to_string(),
            "github".to_string(),
            "id".to_string(),
            "image".to_string(),
            "last_name".to_string(),
            "mailchimp_id".to_string(),
            "name".to_string(),
            "phone".to_string(),
            "stripe_id".to_string(),
            "updated_at".to_string(),
            "zendesk_id".to_string(),
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

impl tabled::Tabled for ExtendedUserResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.items),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["items".to_string(), "next_page".to_string()]
    }
}

#[doc = "A file conversion."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileConversion {
    #[doc = "The time and date the file conversion was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the file conversion was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the file conversion.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The converted file, if completed, base64 encoded."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<base64::Base64Data>,
    #[doc = "The output format of the file conversion."]
    pub output_format: FileOutputFormat,
    #[doc = "The source format of the file conversion."]
    pub src_format: FileSourceFormat,
    #[doc = "The time and date the file conversion was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the file conversion."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the file conversion was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the file conversion."]
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

impl tabled::Tabled for FileConversion {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(error) = &self.error {
                format!("{:?}", error)
            } else {
                String::new()
            },
            format!("{:?}", self.id),
            if let Some(output) = &self.output {
                format!("{:?}", output)
            } else {
                String::new()
            },
            format!("{:?}", self.output_format),
            format!("{:?}", self.src_format),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at)
            } else {
                String::new()
            },
            format!("{:?}", self.status),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "completed_at".to_string(),
            "created_at".to_string(),
            "error".to_string(),
            "id".to_string(),
            "output".to_string(),
            "output_format".to_string(),
            "src_format".to_string(),
            "started_at".to_string(),
            "status".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
        ]
    }
}

#[doc = "A file density result."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileDensity {
    #[doc = "The time and date the density was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the density was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The resulting density."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the density request.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The material mass as denoted by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_mass: Option<f64>,
    #[doc = "The source format of the file."]
    pub src_format: FileSourceFormat,
    #[doc = "The time and date the density was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the density."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the density was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the density."]
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

impl tabled::Tabled for FileDensity {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(density) = &self.density {
                format!("{:?}", density)
            } else {
                String::new()
            },
            if let Some(error) = &self.error {
                format!("{:?}", error)
            } else {
                String::new()
            },
            format!("{:?}", self.id),
            if let Some(material_mass) = &self.material_mass {
                format!("{:?}", material_mass)
            } else {
                String::new()
            },
            format!("{:?}", self.src_format),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at)
            } else {
                String::new()
            },
            format!("{:?}", self.status),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "completed_at".to_string(),
            "created_at".to_string(),
            "density".to_string(),
            "error".to_string(),
            "id".to_string(),
            "material_mass".to_string(),
            "src_format".to_string(),
            "started_at".to_string(),
            "status".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
        ]
    }
}

#[doc = "A file mass result."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileMass {
    #[doc = "The time and date the mass was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the mass was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the mass request.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The resulting mass."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mass: Option<f64>,
    #[doc = "The material density as denoted by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_density: Option<f64>,
    #[doc = "The source format of the file."]
    pub src_format: FileSourceFormat,
    #[doc = "The time and date the mass was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the mass."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the mass was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the mass."]
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

impl tabled::Tabled for FileMass {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(error) = &self.error {
                format!("{:?}", error)
            } else {
                String::new()
            },
            format!("{:?}", self.id),
            if let Some(mass) = &self.mass {
                format!("{:?}", mass)
            } else {
                String::new()
            },
            if let Some(material_density) = &self.material_density {
                format!("{:?}", material_density)
            } else {
                String::new()
            },
            format!("{:?}", self.src_format),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at)
            } else {
                String::new()
            },
            format!("{:?}", self.status),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "completed_at".to_string(),
            "created_at".to_string(),
            "error".to_string(),
            "id".to_string(),
            "mass".to_string(),
            "material_density".to_string(),
            "src_format".to_string(),
            "started_at".to_string(),
            "status".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
        ]
    }
}

#[doc = "The valid types of output file formats."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum FileOutputFormat {
    #[serde(rename = "stl")]
    #[display("stl")]
    Stl,
    #[serde(rename = "obj")]
    #[display("obj")]
    Obj,
    #[serde(rename = "dae")]
    #[display("dae")]
    Dae,
    #[serde(rename = "step")]
    #[display("step")]
    Step,
    #[serde(rename = "fbx")]
    #[display("fbx")]
    Fbx,
    #[serde(rename = "fbxb")]
    #[display("fbxb")]
    Fbxb,
}

#[doc = "The valid types of source file formats."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum FileSourceFormat {
    #[serde(rename = "stl")]
    #[display("stl")]
    Stl,
    #[serde(rename = "obj")]
    #[display("obj")]
    Obj,
    #[serde(rename = "dae")]
    #[display("dae")]
    Dae,
    #[serde(rename = "step")]
    #[display("step")]
    Step,
    #[serde(rename = "fbx")]
    #[display("fbx")]
    Fbx,
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

impl tabled::Tabled for FileSystemMetadata {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.ok)]
    }

    fn headers() -> Vec<String> {
        vec!["ok".to_string()]
    }
}

#[doc = "A file volume result."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileVolume {
    #[doc = "The time and date the volume was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the volume was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the volume request.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The source format of the file."]
    pub src_format: FileSourceFormat,
    #[doc = "The time and date the volume was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the volume."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the volume was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the volume."]
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

impl tabled::Tabled for FileVolume {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(error) = &self.error {
                format!("{:?}", error)
            } else {
                String::new()
            },
            format!("{:?}", self.id),
            format!("{:?}", self.src_format),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at)
            } else {
                String::new()
            },
            format!("{:?}", self.status),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
            if let Some(volume) = &self.volume {
                format!("{:?}", volume)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "completed_at".to_string(),
            "created_at".to_string(),
            "error".to_string(),
            "id".to_string(),
            "src_format".to_string(),
            "started_at".to_string(),
            "status".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
            "volume".to_string(),
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

impl tabled::Tabled for Gateway {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(auth_timeout) = &self.auth_timeout {
                format!("{:?}", auth_timeout)
            } else {
                String::new()
            },
            if let Some(host) = &self.host {
                format!("{:?}", host)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(port) = &self.port {
                format!("{:?}", port)
            } else {
                String::new()
            },
            if let Some(tls_timeout) = &self.tls_timeout {
                format!("{:?}", tls_timeout)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "auth_timeout".to_string(),
            "host".to_string(),
            "name".to_string(),
            "port".to_string(),
            "tls_timeout".to_string(),
        ]
    }
}

#[doc = "IndexInfo contains information about a registry."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct IndexInfo {
    #[doc = "List of mirrors, expressed as URIs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirrors: Option<Vec<String>>,
    #[doc = "Name of the registry, such as \\\"docker.io\\\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether this is an official registry (i.e., Docker Hub / docker.io)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub official: Option<bool>,
    #[doc = "Indicates if the registry is part of the list of insecure registries.  If `false`, \
             the registry is insecure. Insecure registries accept un-encrypted (HTTP) and/or \
             untrusted (HTTPS with certificates from unknown CAs) communication.\n\n**Warning**: \
             Insecure registries can be useful when running a local registry. However, because \
             its use creates security vulnerabilities it should ONLY be enabled for testing \
             purposes. For increased security, users should add their CA to their system's list \
             of trusted CAs instead of enabling this option."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
}

impl std::fmt::Display for IndexInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for IndexInfo {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(mirrors) = &self.mirrors {
                format!("{:?}", mirrors)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(official) = &self.official {
                format!("{:?}", official)
            } else {
                String::new()
            },
            if let Some(secure) = &self.secure {
                format!("{:?}", secure)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "mirrors".to_string(),
            "name".to_string(),
            "official".to_string(),
            "secure".to_string(),
        ]
    }
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
    pub amount_due: Option<f64>,
    #[doc = "The amount, in USD, that was paid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<f64>,
    #[doc = "The amount remaining, in USD, that is due."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_remaining: Option<f64>,
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
    pub currency: Currency,
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
    pub pdf: Option<url::Url>,
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
    pub subtotal: Option<f64>,
    #[doc = "The amount of tax on this invoice.\n\nThis is the sum of all the tax amounts on this \
             invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[doc = "Total after discounts and taxes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[doc = "The URL for the hosted invoice page, which allows customers to view and pay an \
             invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<url::Url>,
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

impl tabled::Tabled for Invoice {
    const LENGTH: usize = 24;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(amount_due) = &self.amount_due {
                format!("{:?}", amount_due)
            } else {
                String::new()
            },
            if let Some(amount_paid) = &self.amount_paid {
                format!("{:?}", amount_paid)
            } else {
                String::new()
            },
            if let Some(amount_remaining) = &self.amount_remaining {
                format!("{:?}", amount_remaining)
            } else {
                String::new()
            },
            if let Some(attempt_count) = &self.attempt_count {
                format!("{:?}", attempt_count)
            } else {
                String::new()
            },
            if let Some(attempted) = &self.attempted {
                format!("{:?}", attempted)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            format!("{:?}", self.currency),
            if let Some(customer_email) = &self.customer_email {
                format!("{:?}", customer_email)
            } else {
                String::new()
            },
            if let Some(customer_id) = &self.customer_id {
                format!("{:?}", customer_id)
            } else {
                String::new()
            },
            if let Some(default_payment_method) = &self.default_payment_method {
                format!("{:?}", default_payment_method)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(lines) = &self.lines {
                format!("{:?}", lines)
            } else {
                String::new()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata)
            } else {
                String::new()
            },
            if let Some(number) = &self.number {
                format!("{:?}", number)
            } else {
                String::new()
            },
            if let Some(paid) = &self.paid {
                format!("{:?}", paid)
            } else {
                String::new()
            },
            if let Some(pdf) = &self.pdf {
                format!("{:?}", pdf)
            } else {
                String::new()
            },
            if let Some(receipt_number) = &self.receipt_number {
                format!("{:?}", receipt_number)
            } else {
                String::new()
            },
            if let Some(statement_descriptor) = &self.statement_descriptor {
                format!("{:?}", statement_descriptor)
            } else {
                String::new()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(subtotal) = &self.subtotal {
                format!("{:?}", subtotal)
            } else {
                String::new()
            },
            if let Some(tax) = &self.tax {
                format!("{:?}", tax)
            } else {
                String::new()
            },
            if let Some(total) = &self.total {
                format!("{:?}", total)
            } else {
                String::new()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "amount_due".to_string(),
            "amount_paid".to_string(),
            "amount_remaining".to_string(),
            "attempt_count".to_string(),
            "attempted".to_string(),
            "created_at".to_string(),
            "currency".to_string(),
            "customer_email".to_string(),
            "customer_id".to_string(),
            "default_payment_method".to_string(),
            "description".to_string(),
            "id".to_string(),
            "lines".to_string(),
            "metadata".to_string(),
            "number".to_string(),
            "paid".to_string(),
            "pdf".to_string(),
            "receipt_number".to_string(),
            "statement_descriptor".to_string(),
            "status".to_string(),
            "subtotal".to_string(),
            "tax".to_string(),
            "total".to_string(),
            "url".to_string(),
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
    pub amount: Option<f64>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), \
             in lowercase."]
    pub currency: Currency,
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

impl tabled::Tabled for InvoiceLineItem {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(amount) = &self.amount {
                format!("{:?}", amount)
            } else {
                String::new()
            },
            format!("{:?}", self.currency),
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(invoice_item) = &self.invoice_item {
                format!("{:?}", invoice_item)
            } else {
                String::new()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "amount".to_string(),
            "currency".to_string(),
            "description".to_string(),
            "id".to_string(),
            "invoice_item".to_string(),
            "metadata".to_string(),
        ]
    }
}

#[doc = "An enum representing the possible values of an `Invoice`'s `status` field."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum InvoiceStatus {
    #[serde(rename = "deleted")]
    #[display("deleted")]
    Deleted,
    #[serde(rename = "draft")]
    #[display("draft")]
    Draft,
    #[serde(rename = "open")]
    #[display("open")]
    Open,
    #[serde(rename = "paid")]
    #[display("paid")]
    Paid,
    #[serde(rename = "uncollectible")]
    #[display("uncollectible")]
    Uncollectible,
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

impl tabled::Tabled for Jetstream {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(config) = &self.config {
                format!("{:?}", config)
            } else {
                String::new()
            },
            if let Some(meta) = &self.meta {
                format!("{:?}", meta)
            } else {
                String::new()
            },
            if let Some(stats) = &self.stats {
                format!("{:?}", stats)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "config".to_string(),
            "meta".to_string(),
            "stats".to_string(),
        ]
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

impl tabled::Tabled for JetstreamApiStats {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(errors) = &self.errors {
                format!("{:?}", errors)
            } else {
                String::new()
            },
            if let Some(inflight) = &self.inflight {
                format!("{:?}", inflight)
            } else {
                String::new()
            },
            if let Some(total) = &self.total {
                format!("{:?}", total)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "errors".to_string(),
            "inflight".to_string(),
            "total".to_string(),
        ]
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

impl tabled::Tabled for JetstreamConfig {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(domain) = &self.domain {
                format!("{:?}", domain)
            } else {
                String::new()
            },
            if let Some(max_memory) = &self.max_memory {
                format!("{:?}", max_memory)
            } else {
                String::new()
            },
            if let Some(max_storage) = &self.max_storage {
                format!("{:?}", max_storage)
            } else {
                String::new()
            },
            if let Some(store_dir) = &self.store_dir {
                format!("{:?}", store_dir)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "domain".to_string(),
            "max_memory".to_string(),
            "max_storage".to_string(),
            "store_dir".to_string(),
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

impl tabled::Tabled for JetstreamStats {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(accounts) = &self.accounts {
                format!("{:?}", accounts)
            } else {
                String::new()
            },
            if let Some(api) = &self.api {
                format!("{:?}", api)
            } else {
                String::new()
            },
            if let Some(ha_assets) = &self.ha_assets {
                format!("{:?}", ha_assets)
            } else {
                String::new()
            },
            if let Some(memory) = &self.memory {
                format!("{:?}", memory)
            } else {
                String::new()
            },
            if let Some(reserved_memory) = &self.reserved_memory {
                format!("{:?}", reserved_memory)
            } else {
                String::new()
            },
            if let Some(reserved_store) = &self.reserved_store {
                format!("{:?}", reserved_store)
            } else {
                String::new()
            },
            if let Some(store) = &self.store {
                format!("{:?}", store)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "accounts".to_string(),
            "api".to_string(),
            "ha_assets".to_string(),
            "memory".to_string(),
            "reserved_memory".to_string(),
            "reserved_store".to_string(),
            "store".to_string(),
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

impl tabled::Tabled for LeafNode {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(auth_timeout) = &self.auth_timeout {
                format!("{:?}", auth_timeout)
            } else {
                String::new()
            },
            if let Some(host) = &self.host {
                format!("{:?}", host)
            } else {
                String::new()
            },
            if let Some(port) = &self.port {
                format!("{:?}", port)
            } else {
                String::new()
            },
            if let Some(tls_timeout) = &self.tls_timeout {
                format!("{:?}", tls_timeout)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "auth_timeout".to_string(),
            "host".to_string(),
            "port".to_string(),
            "tls_timeout".to_string(),
        ]
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

impl tabled::Tabled for MetaClusterInfo {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(cluster_size) = &self.cluster_size {
                format!("{:?}", cluster_size)
            } else {
                String::new()
            },
            if let Some(leader) = &self.leader {
                format!("{:?}", leader)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "cluster_size".to_string(),
            "leader".to_string(),
            "name".to_string(),
        ]
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
    #[doc = "Metadata about our engine API connection."]
    pub engine: EngineMetadata,
    #[doc = "The environment we are running in."]
    pub environment: Environment,
    #[doc = "Metadata about our executor API connection."]
    pub executor: ExecutorMetadata,
    #[doc = "Metadata about our file system."]
    pub fs: FileSystemMetadata,
    #[doc = "The git hash of the server."]
    pub git_hash: String,
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

impl tabled::Tabled for Metadata {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.cache),
            format!("{:?}", self.engine),
            format!("{:?}", self.environment),
            format!("{:?}", self.executor),
            format!("{:?}", self.fs),
            self.git_hash.clone(),
            format!("{:?}", self.pubsub),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "cache".to_string(),
            "engine".to_string(),
            "environment".to_string(),
            "executor".to_string(),
            "fs".to_string(),
            "git_hash".to_string(),
            "pubsub".to_string(),
        ]
    }
}

#[doc = "The Request Method (VERB)\n\nThis type also contains constants for a number of common HTTP methods such as GET, POST, etc.\n\nCurrently includes 8 variants representing the 8 methods defined in [RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH, and an Extension variant for all extensions."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Method {
    #[serde(rename = "OPTIONS")]
    #[display("OPTIONS")]
    Options,
    #[serde(rename = "GET")]
    #[display("GET")]
    Get,
    #[serde(rename = "POST")]
    #[display("POST")]
    Post,
    #[serde(rename = "PUT")]
    #[display("PUT")]
    Put,
    #[serde(rename = "DELETE")]
    #[display("DELETE")]
    Delete,
    #[serde(rename = "HEAD")]
    #[display("HEAD")]
    Head,
    #[serde(rename = "TRACE")]
    #[display("TRACE")]
    Trace,
    #[serde(rename = "CONNECT")]
    #[display("CONNECT")]
    Connect,
    #[serde(rename = "PATCH")]
    #[display("PATCH")]
    Patch,
    #[serde(rename = "EXTENSION")]
    #[display("EXTENSION")]
    Extension,
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

impl tabled::Tabled for Oauth2ClientInfo {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(csrf_token) = &self.csrf_token {
                format!("{:?}", csrf_token)
            } else {
                String::new()
            },
            if let Some(pkce_code_verifier) = &self.pkce_code_verifier {
                format!("{:?}", pkce_code_verifier)
            } else {
                String::new()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "csrf_token".to_string(),
            "pkce_code_verifier".to_string(),
            "url".to_string(),
        ]
    }
}

#[doc = "An OAuth 2.0 Grant Type. These are documented here: <https://oauth.net/2/grant-types/>."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Oauth2GrantType {
    #[serde(rename = "urn:ietf:params:oauth:grant-type:device_code")]
    #[display("urn:ietf:params:oauth:grant-type:device_code")]
    UrnIetfParamsOauthGrantTypeDeviceCode,
}

impl std::default::Default for Oauth2GrantType {
    fn default() -> Self {
        Oauth2GrantType::UrnIetfParamsOauthGrantTypeDeviceCode
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

impl tabled::Tabled for OutputFile {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(contents) = &self.contents {
                format!("{:?}", contents)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["contents".to_string(), "name".to_string()]
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

impl tabled::Tabled for PaymentIntent {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![self.client_secret.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["client_secret".to_string()]
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

impl tabled::Tabled for PaymentMethod {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.billing_info),
            if let Some(card) = &self.card {
                format!("{:?}", card)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata)
            } else {
                String::new()
            },
            format!("{:?}", self.type_),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "billing_info".to_string(),
            "card".to_string(),
            "created_at".to_string(),
            "id".to_string(),
            "metadata".to_string(),
            "type_".to_string(),
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

impl tabled::Tabled for PaymentMethodCardChecks {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(address_line_1_check) = &self.address_line_1_check {
                format!("{:?}", address_line_1_check)
            } else {
                String::new()
            },
            if let Some(address_postal_code_check) = &self.address_postal_code_check {
                format!("{:?}", address_postal_code_check)
            } else {
                String::new()
            },
            if let Some(cvc_check) = &self.cvc_check {
                format!("{:?}", cvc_check)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "address_line_1_check".to_string(),
            "address_postal_code_check".to_string(),
            "cvc_check".to_string(),
        ]
    }
}

#[doc = "An enum representing the possible values of an `PaymentMethod`'s `type` field."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum PaymentMethodType {
    #[serde(rename = "card")]
    #[display("card")]
    Card,
}

impl std::default::Default for PaymentMethodType {
    fn default() -> Self {
        PaymentMethodType::Card
    }
}

#[doc = "Available plugins per type.\n\n**Note**: Only unmanaged (V1) plugins are included in this \
         list. V1 plugins are \\\"lazily\\\" loaded, and are not returned in this list if there is \
         no resource using the plugin."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PluginsInfo {
    #[doc = "Names of available authorization plugins."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Vec<String>>,
    #[doc = "Names of available logging-drivers, and logging-driver plugins."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<String>>,
    #[doc = "Names of available network-drivers, and network-driver plugins."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<String>>,
    #[doc = "Names of available volume-drivers, and network-driver plugins."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<String>>,
}

impl std::fmt::Display for PluginsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for PluginsInfo {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(authorization) = &self.authorization {
                format!("{:?}", authorization)
            } else {
                String::new()
            },
            if let Some(log) = &self.log {
                format!("{:?}", log)
            } else {
                String::new()
            },
            if let Some(network) = &self.network {
                format!("{:?}", network)
            } else {
                String::new()
            },
            if let Some(volume) = &self.volume {
                format!("{:?}", volume)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "authorization".to_string(),
            "log".to_string(),
            "network".to_string(),
            "volume".to_string(),
        ]
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

impl tabled::Tabled for Pong {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![self.message.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["message".to_string()]
    }
}

#[doc = "RegistryServiceConfig stores daemon registry services configuration."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RegistryServiceConfig {
    #[doc = "List of IP ranges to which nondistributable artifacts can be pushed, using the CIDR syntax [RFC 4632](https://tools.ietf.org/html/4632).  Some images (for example, Windows base images) contain artifacts whose distribution is restricted by license. When these images are pushed to a registry, restricted artifacts are not included.  This configuration override this behavior, and enables the daemon to push nondistributable artifacts to all registries whose resolved IP address is within the subnet described by the CIDR syntax.  This option is useful when pushing images containing nondistributable artifacts to a registry on an air-gapped network so hosts on that network can pull the images without connecting to another server.\n\n**Warning**: Nondistributable artifacts typically have restrictions on how and where they can be distributed and shared. Only use this feature to push artifacts to private registries and ensure that you are in compliance with any terms that cover redistributing nondistributable artifacts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_nondistributable_artifacts_cid_rs: Option<Vec<String>>,
    #[doc = "List of registry hostnames to which nondistributable artifacts can be pushed, using \
             the format `<hostname>[:<port>]` or `<IP address>[:<port>]`.  Some images (for \
             example, Windows base images) contain artifacts whose distribution is restricted by \
             license. When these images are pushed to a registry, restricted artifacts are not \
             included.  This configuration override this behavior for the specified registries.  \
             This option is useful when pushing images containing nondistributable artifacts to a \
             registry on an air-gapped network so hosts on that network can pull the images \
             without connecting to another server.\n\n**Warning**: Nondistributable artifacts \
             typically have restrictions on how and where they can be distributed and shared. \
             Only use this feature to push artifacts to private registries and ensure that you \
             are in compliance with any terms that cover redistributing nondistributable \
             artifacts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_nondistributable_artifacts_hostnames: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_configs: Option<std::collections::HashMap<String, IndexInfo>>,
    #[doc = "List of IP ranges of insecure registries, using the CIDR syntax ([RFC 4632](https://tools.ietf.org/html/4632)). Insecure registries accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from unknown CAs) communication.  By default, local registries (`127.0.0.0/8`) are configured as insecure. All other registries are secure. Communicating with an insecure registry is not possible if the daemon assumes that registry is secure.  This configuration override this behavior, insecure communication with registries whose resolved IP address is within the subnet described by the CIDR syntax.  Registries can also be marked insecure by hostname. Those registries are listed under `IndexConfigs` and have their `Secure` field set to `false`.\n\n**Warning**: Using this option can be useful when running a local  registry, but introduces security vulnerabilities. This option should therefore ONLY be used for testing purposes. For increased security, users should add their CA to their system's list of trusted CAs instead of enabling this option."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_registry_cid_rs: Option<Vec<String>>,
    #[doc = "List of registry URLs that act as a mirror for the official (`docker.io`) registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirrors: Option<Vec<String>>,
}

impl std::fmt::Display for RegistryServiceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RegistryServiceConfig {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(allow_nondistributable_artifacts_cid_rs) =
                &self.allow_nondistributable_artifacts_cid_rs
            {
                format!("{:?}", allow_nondistributable_artifacts_cid_rs)
            } else {
                String::new()
            },
            if let Some(allow_nondistributable_artifacts_hostnames) =
                &self.allow_nondistributable_artifacts_hostnames
            {
                format!("{:?}", allow_nondistributable_artifacts_hostnames)
            } else {
                String::new()
            },
            if let Some(index_configs) = &self.index_configs {
                format!("{:?}", index_configs)
            } else {
                String::new()
            },
            if let Some(insecure_registry_cid_rs) = &self.insecure_registry_cid_rs {
                format!("{:?}", insecure_registry_cid_rs)
            } else {
                String::new()
            },
            if let Some(mirrors) = &self.mirrors {
                format!("{:?}", mirrors)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "allow_nondistributable_artifacts_cid_rs".to_string(),
            "allow_nondistributable_artifacts_hostnames".to_string(),
            "index_configs".to_string(),
            "insecure_registry_cid_rs".to_string(),
            "mirrors".to_string(),
        ]
    }
}

#[doc = "Runtime describes an [OCI compliant](https://github.com/opencontainers/runtime-spec) \
         runtime.  The runtime is invoked by the daemon via the `containerd` daemon. OCI runtimes \
         act as an interface to the Linux kernel namespaces, cgroups, and SELinux."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Runtime {
    #[doc = "Name and, optional, path, of the OCI executable binary.  If the path is omitted, the \
             daemon searches the host's `$PATH` for the binary and uses the first result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "List of command-line arguments to pass to the runtime when invoked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_args: Option<Vec<String>>,
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Runtime {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(path) = &self.path {
                format!("{:?}", path)
            } else {
                String::new()
            },
            if let Some(runtime_args) = &self.runtime_args {
                format!("{:?}", runtime_args)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["path".to_string(), "runtime_args".to_string()]
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

impl tabled::Tabled for Session {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.created_at),
            format!("{:?}", self.expires),
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            format!("{:?}", self.session_token),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "created_at".to_string(),
            "expires".to_string(),
            "id".to_string(),
            "session_token".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum SystemInfoCgroupDriverEnum {
    #[serde(rename = "")]
    #[display("")]
    Empty,
    #[serde(rename = "cgroupfs")]
    #[display("cgroupfs")]
    Cgroupfs,
    #[serde(rename = "systemd")]
    #[display("systemd")]
    Systemd,
    #[serde(rename = "none")]
    #[display("none")]
    None,
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum SystemInfoCgroupVersionEnum {
    #[serde(rename = "")]
    #[display("")]
    Empty,
    #[serde(rename = "1")]
    #[display("1")]
    One,
    #[serde(rename = "2")]
    #[display("2")]
    Two,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SystemInfoDefaultAddressPools {
    #[doc = "The network address in CIDR format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[doc = "The network pool size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl std::fmt::Display for SystemInfoDefaultAddressPools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SystemInfoDefaultAddressPools {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(base) = &self.base {
                format!("{:?}", base)
            } else {
                String::new()
            },
            if let Some(size) = &self.size {
                format!("{:?}", size)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["base".to_string(), "size".to_string()]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum SystemInfoIsolationEnum {
    #[serde(rename = "")]
    #[display("")]
    Empty,
    #[serde(rename = "default")]
    #[display("default")]
    Default,
    #[serde(rename = "hyperv")]
    #[display("hyperv")]
    Hyperv,
    #[serde(rename = "process")]
    #[display("process")]
    Process,
}

#[doc = "A unit conversion."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnitConversion {
    #[doc = "The time and date the unit conversion was completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The time and date the unit conversion was created."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The error the function returned, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The unique identifier of the unit conversion.\n\nThis is the same as the API call ID."]
    pub id: uuid::Uuid,
    #[doc = "The input value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    #[doc = "The resulting value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    #[doc = "The output format of the unit conversion."]
    pub output_format: UnitMetricFormat,
    #[doc = "The source format of the unit conversion."]
    pub src_format: UnitMetricFormat,
    #[doc = "The time and date the unit conversion was started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The status of the unit conversion."]
    pub status: ApiCallStatus,
    #[doc = "The time and date the unit conversion was last updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The user ID of the user who created the unit conversion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl std::fmt::Display for UnitConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UnitConversion {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(completed_at) = &self.completed_at {
                format!("{:?}", completed_at)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(error) = &self.error {
                format!("{:?}", error)
            } else {
                String::new()
            },
            format!("{:?}", self.id),
            if let Some(input) = &self.input {
                format!("{:?}", input)
            } else {
                String::new()
            },
            if let Some(output) = &self.output {
                format!("{:?}", output)
            } else {
                String::new()
            },
            format!("{:?}", self.output_format),
            format!("{:?}", self.src_format),
            if let Some(started_at) = &self.started_at {
                format!("{:?}", started_at)
            } else {
                String::new()
            },
            format!("{:?}", self.status),
            format!("{:?}", self.updated_at),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "completed_at".to_string(),
            "created_at".to_string(),
            "error".to_string(),
            "id".to_string(),
            "input".to_string(),
            "output".to_string(),
            "output_format".to_string(),
            "src_format".to_string(),
            "started_at".to_string(),
            "status".to_string(),
            "updated_at".to_string(),
            "user_id".to_string(),
        ]
    }
}

#[doc = "The valid types of metric unit formats."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum UnitMetricFormat {
    #[serde(rename = "atto")]
    #[display("atto")]
    Atto,
    #[serde(rename = "femto")]
    #[display("femto")]
    Femto,
    #[serde(rename = "pico")]
    #[display("pico")]
    Pico,
    #[serde(rename = "nano")]
    #[display("nano")]
    Nano,
    #[serde(rename = "micro")]
    #[display("micro")]
    Micro,
    #[serde(rename = "milli")]
    #[display("milli")]
    Milli,
    #[serde(rename = "centi")]
    #[display("centi")]
    Centi,
    #[serde(rename = "deci")]
    #[display("deci")]
    Deci,
    #[serde(rename = "metric_unit")]
    #[display("metric_unit")]
    MetricUnit,
    #[serde(rename = "deca")]
    #[display("deca")]
    Deca,
    #[serde(rename = "hecto")]
    #[display("hecto")]
    Hecto,
    #[serde(rename = "kilo")]
    #[display("kilo")]
    Kilo,
    #[serde(rename = "mega")]
    #[display("mega")]
    Mega,
    #[serde(rename = "giga")]
    #[display("giga")]
    Giga,
    #[serde(rename = "tera")]
    #[display("tera")]
    Tera,
    #[serde(rename = "peta")]
    #[display("peta")]
    Peta,
    #[serde(rename = "exa")]
    #[display("exa")]
    Exa,
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

impl tabled::Tabled for UpdateUser {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(company) = &self.company {
                format!("{:?}", company)
            } else {
                String::new()
            },
            if let Some(discord) = &self.discord {
                format!("{:?}", discord)
            } else {
                String::new()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name)
            } else {
                String::new()
            },
            if let Some(github) = &self.github {
                format!("{:?}", github)
            } else {
                String::new()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name)
            } else {
                String::new()
            },
            format!("{:?}", self.phone),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "company".to_string(),
            "discord".to_string(),
            "first_name".to_string(),
            "github".to_string(),
            "last_name".to_string(),
            "phone".to_string(),
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
    pub image: url::Url,
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

impl tabled::Tabled for User {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(company) = &self.company {
                format!("{:?}", company)
            } else {
                String::new()
            },
            format!("{:?}", self.created_at),
            if let Some(discord) = &self.discord {
                format!("{:?}", discord)
            } else {
                String::new()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email)
            } else {
                String::new()
            },
            if let Some(email_verified) = &self.email_verified {
                format!("{:?}", email_verified)
            } else {
                String::new()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name)
            } else {
                String::new()
            },
            if let Some(github) = &self.github {
                format!("{:?}", github)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            format!("{:?}", self.image),
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.phone),
            format!("{:?}", self.updated_at),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "company".to_string(),
            "created_at".to_string(),
            "discord".to_string(),
            "email".to_string(),
            "email_verified".to_string(),
            "first_name".to_string(),
            "github".to_string(),
            "id".to_string(),
            "image".to_string(),
            "last_name".to_string(),
            "name".to_string(),
            "phone".to_string(),
            "updated_at".to_string(),
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

impl tabled::Tabled for UserResultsPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.items),
            if let Some(next_page) = &self.next_page {
                format!("{:?}", next_page)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["items".to_string(), "next_page".to_string()]
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

impl tabled::Tabled for VerificationToken {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.created_at),
            format!("{:?}", self.expires),
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(identifier) = &self.identifier {
                format!("{:?}", identifier)
            } else {
                String::new()
            },
            format!("{:?}", self.updated_at),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "created_at".to_string(),
            "expires".to_string(),
            "id".to_string(),
            "identifier".to_string(),
            "updated_at".to_string(),
        ]
    }
}
