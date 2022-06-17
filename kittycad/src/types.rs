//! The data types sent to and returned from the API client.
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use tabled::Tabled;

/**
* The status of an async API call.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum ApiCallStatus {
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "In Progress")]
    InProgress,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "Uploaded")]
    Uploaded,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ApiCallStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ApiCallStatus::Completed => "Completed",
            ApiCallStatus::Failed => "Failed",
            ApiCallStatus::InProgress => "In Progress",
            ApiCallStatus::Queued => "Queued",
            ApiCallStatus::Uploaded => "Uploaded",
            ApiCallStatus::Noop => "",
            ApiCallStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ApiCallStatus {
    fn default() -> ApiCallStatus {
        ApiCallStatus::Completed
    }
}
impl std::str::FromStr for ApiCallStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Completed" {
            return Ok(ApiCallStatus::Completed);
        }
        if s == "Failed" {
            return Ok(ApiCallStatus::Failed);
        }
        if s == "In Progress" {
            return Ok(ApiCallStatus::InProgress);
        }
        if s == "Queued" {
            return Ok(ApiCallStatus::Queued);
        }
        if s == "Uploaded" {
            return Ok(ApiCallStatus::Uploaded);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl ApiCallStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, ApiCallStatus::Noop)
    }
}

/// An address.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Address {
    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * An address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub city: String,

    /**
    * An address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,

    /**
    * The time and date the address was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * An address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,

    /**
    * An address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "street1"
    )]
    pub street_1: String,

    /**
    * An address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "street2"
    )]
    pub street_2: String,

    /**
    * The time and date the address was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * An address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,

    /**
    * An address.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zip: String,
}

/// A response for a query on the API call table that is grouped by something.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ApiCallQueryGroup {
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub count: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub query: String,
}

/**
* The field of an API call to group by.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum ApiCallQueryGroupBy {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "endpoint")]
    Endpoint,
    #[serde(rename = "ip_address")]
    IpAddress,
    #[serde(rename = "method")]
    Method,
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for ApiCallQueryGroupBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            ApiCallQueryGroupBy::Email => "email",
            ApiCallQueryGroupBy::Endpoint => "endpoint",
            ApiCallQueryGroupBy::IpAddress => "ip_address",
            ApiCallQueryGroupBy::Method => "method",
            ApiCallQueryGroupBy::Origin => "origin",
            ApiCallQueryGroupBy::UserId => "user_id",
            ApiCallQueryGroupBy::Noop => "",
            ApiCallQueryGroupBy::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for ApiCallQueryGroupBy {
    fn default() -> ApiCallQueryGroupBy {
        ApiCallQueryGroupBy::Email
    }
}
impl std::str::FromStr for ApiCallQueryGroupBy {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "email" {
            return Ok(ApiCallQueryGroupBy::Email);
        }
        if s == "endpoint" {
            return Ok(ApiCallQueryGroupBy::Endpoint);
        }
        if s == "ip_address" {
            return Ok(ApiCallQueryGroupBy::IpAddress);
        }
        if s == "method" {
            return Ok(ApiCallQueryGroupBy::Method);
        }
        if s == "origin" {
            return Ok(ApiCallQueryGroupBy::Origin);
        }
        if s == "user_id" {
            return Ok(ApiCallQueryGroupBy::UserId);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl ApiCallQueryGroupBy {
    pub fn is_noop(&self) -> bool {
        matches!(self, ApiCallQueryGroupBy::Noop)
    }
}

/**
* The Request Method (VERB)
*   
*   This type also contains constants for a number of common HTTP methods such as GET, POST, etc.
*   
*   Currently includes 8 variants representing the 8 methods defined in [RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH, and an Extension variant for all extensions.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum Method {
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "EXTENSION")]
    Extension,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Method::Connect => "CONNECT",
            Method::Delete => "DELETE",
            Method::Extension => "EXTENSION",
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
            Method::Patch => "PATCH",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Trace => "TRACE",
            Method::Noop => "",
            Method::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Method {
    fn default() -> Method {
        Method::Connect
    }
}
impl std::str::FromStr for Method {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "CONNECT" {
            return Ok(Method::Connect);
        }
        if s == "DELETE" {
            return Ok(Method::Delete);
        }
        if s == "EXTENSION" {
            return Ok(Method::Extension);
        }
        if s == "GET" {
            return Ok(Method::Get);
        }
        if s == "HEAD" {
            return Ok(Method::Head);
        }
        if s == "OPTIONS" {
            return Ok(Method::Options);
        }
        if s == "PATCH" {
            return Ok(Method::Patch);
        }
        if s == "POST" {
            return Ok(Method::Post);
        }
        if s == "PUT" {
            return Ok(Method::Put);
        }
        if s == "TRACE" {
            return Ok(Method::Trace);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl Method {
    pub fn is_noop(&self) -> bool {
        matches!(self, Method::Noop)
    }
}

/// An API call with the price.
///
/// This is a join of the `APICall` and `APICallPrice` tables.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ApiCallWithPrice {
    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The date and time the API call completed billing.
    */
    #[serde()]
    pub completed_at: crate::utils::DisplayOptionDateTime,

    /**
    * The date and time the API call was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * The duration of the API call.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub duration: i64,

    /**
    * An API call with the price.
    *  
    *  This is a join of the `APICall` and `APICallPrice` tables.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,

    /**
    * An API call with the price.
    *  
    *  This is a join of the `APICall` and `APICallPrice` tables.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub endpoint: String,

    /**
    * An API call with the price.
    *  
    *  This is a join of the `APICall` and `APICallPrice` tables.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip_address: String,

    /**
    * The Request Method (VERB)
    *  
    *  This type also contains constants for a number of common HTTP methods such as GET, POST, etc.
    *  
    *  Currently includes 8 variants representing the 8 methods defined in [RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH, and an Extension variant for all extensions.
    */
    #[serde(default, skip_serializing_if = "Method::is_noop")]
    pub method: Method,

    /**
    * The number of minutes the API call was billed for.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i32",
        deserialize_with = "crate::utils::deserialize_null_i32::deserialize"
    )]
    pub minutes: i32,

    /**
    * An API call with the price.
    *  
    *  This is a join of the `APICall` and `APICallPrice` tables.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub origin: String,

    /**
    * The price of the API call.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub price: f64,

    /**
    * The request body sent by the API call.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub request_body: String,

    /**
    * An API call with the price.
    *  
    *  This is a join of the `APICall` and `APICallPrice` tables.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub request_query_params: String,

    /**
    * The response body returned by the API call. We do not store this information if it is above a certain size.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub response_body: String,

    /**
    * The date and time the API call started billing.
    */
    #[serde()]
    pub started_at: crate::utils::DisplayOptionDateTime,

    /**
    * The status code returned by the API call.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i32",
        deserialize_with = "crate::utils::deserialize_null_i32::deserialize"
    )]
    pub status_code: i32,

    /**
    * An API call with the price.
    *  
    *  This is a join of the `APICall` and `APICallPrice` tables.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stripe_invoice_item_id: String,

    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,

    /**
    * The date and time the API call was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * The user agent of the request.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_agent: String,

    /**
    * An API call with the price.
    *  
    *  This is a join of the `APICall` and `APICallPrice` tables.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ApiCallWithPriceResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[tabled(skip)]
    pub items: Vec<ApiCallWithPrice>,

    /**
    * token used to fetch the next page of results (if any)
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// An API token.
///
/// These are used to authenticate users with Bearer authentication.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ApiToken {
    /**
    * An API token.
    *  
    *  These are used to authenticate users with Bearer authentication.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The date and time the API token was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * If the token is valid. We never delete API tokens, but we can mark them as invalid. We save them for ever to preserve the history of the API token.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub is_valid: bool,

    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,

    /**
    * The date and time the API token was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * An API token.
    *  
    *  These are used to authenticate users with Bearer authentication.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ApiTokenResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[tabled(skip)]
    pub items: Vec<ApiToken>,

    /**
    * token used to fetch the next page of results (if any)
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/**
* The type of async API call.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum AsyncApiCallType {
    #[serde(rename = "FileConversion")]
    FileConversion,
    #[serde(rename = "FileDensity")]
    FileDensity,
    #[serde(rename = "FileMass")]
    FileMass,
    #[serde(rename = "FileVolume")]
    FileVolume,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for AsyncApiCallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AsyncApiCallType::FileConversion => "FileConversion",
            AsyncApiCallType::FileDensity => "FileDensity",
            AsyncApiCallType::FileMass => "FileMass",
            AsyncApiCallType::FileVolume => "FileVolume",
            AsyncApiCallType::Noop => "",
            AsyncApiCallType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for AsyncApiCallType {
    fn default() -> AsyncApiCallType {
        AsyncApiCallType::FileConversion
    }
}
impl std::str::FromStr for AsyncApiCallType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "FileConversion" {
            return Ok(AsyncApiCallType::FileConversion);
        }
        if s == "FileDensity" {
            return Ok(AsyncApiCallType::FileDensity);
        }
        if s == "FileMass" {
            return Ok(AsyncApiCallType::FileMass);
        }
        if s == "FileVolume" {
            return Ok(AsyncApiCallType::FileVolume);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl AsyncApiCallType {
    pub fn is_noop(&self) -> bool {
        matches!(self, AsyncApiCallType::Noop)
    }
}

/// An async API call.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct AsyncApiCall {
    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The time and date the async API call was completed.
    */
    #[serde()]
    pub completed_at: crate::utils::DisplayOptionDateTime,

    /**
    * The time and date the async API call was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * The error the function returned, if any.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,

    /**
    * An async API call.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[tabled(skip)]
    pub input: Option<serde_json::Value>,

    /**
    * The JSON output for the API call. These are determined by the endpoint that is run.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[tabled(skip)]
    pub output: Option<serde_json::Value>,

    /**
    * The time and date the async API call was started.
    */
    #[serde()]
    pub started_at: crate::utils::DisplayOptionDateTime,

    /**
    * The status of an async API call.
    */
    #[serde(default, skip_serializing_if = "ApiCallStatus::is_noop")]
    pub status: ApiCallStatus,

    /**
    * The type of async API call.
    */
    #[serde(
        default,
        skip_serializing_if = "AsyncApiCallType::is_noop",
        rename = "type"
    )]
    pub type_: AsyncApiCallType,

    /**
    * The time and date the async API call was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * An async API call.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,

    /**
    * An async API call.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub worker: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
#[serde(tag = "type")]
pub enum AsyncApiCallOutput {
    FileConversion {
        completed_at: crate::utils::DisplayOptionDateTime,
        created_at: crate::utils::DisplayOptionDateTime,
        error: String,
        id: String,
        output: String,
        output_format: FileOutputFormat,
        src_format: FileSourceFormat,
        started_at: crate::utils::DisplayOptionDateTime,
        status: ApiCallStatus,
        updated_at: crate::utils::DisplayOptionDateTime,
        user_id: String,
    },
    FileMass {
        completed_at: crate::utils::DisplayOptionDateTime,
        created_at: crate::utils::DisplayOptionDateTime,
        error: String,
        id: String,
        mass: f64,
        material_density: f64,
        src_format: FileSourceFormat,
        started_at: crate::utils::DisplayOptionDateTime,
        status: ApiCallStatus,
        updated_at: crate::utils::DisplayOptionDateTime,
        user_id: String,
    },
    FileVolume {
        completed_at: crate::utils::DisplayOptionDateTime,
        created_at: crate::utils::DisplayOptionDateTime,
        error: String,
        id: String,
        src_format: FileSourceFormat,
        started_at: crate::utils::DisplayOptionDateTime,
        status: ApiCallStatus,
        updated_at: crate::utils::DisplayOptionDateTime,
        user_id: String,
        volume: f64,
    },
    FileDensity {
        completed_at: crate::utils::DisplayOptionDateTime,
        created_at: crate::utils::DisplayOptionDateTime,
        density: f64,
        error: String,
        id: String,
        material_mass: f64,
        src_format: FileSourceFormat,
        started_at: crate::utils::DisplayOptionDateTime,
        status: ApiCallStatus,
        updated_at: crate::utils::DisplayOptionDateTime,
        user_id: String,
    },
}

impl fmt::Display for AsyncApiCallOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

impl std::str::FromStr for AsyncApiCallOutput {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}
impl AsyncApiCallOutput {
    pub fn variants() -> Vec<String> {
        vec![
            "FileConversion".to_string(),
            "FileDensity".to_string(),
            "FileMass".to_string(),
            "FileVolume".to_string(),
        ]
    }
}
/**
* The types for AsyncApiCallOutput.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum AsyncApiCallOutputType {
    #[serde(rename = "FileConversion")]
    FileConversion,
    #[serde(rename = "FileDensity")]
    FileDensity,
    #[serde(rename = "FileMass")]
    FileMass,
    #[serde(rename = "FileVolume")]
    FileVolume,
}

impl std::fmt::Display for AsyncApiCallOutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AsyncApiCallOutputType::FileConversion => "FileConversion",
            AsyncApiCallOutputType::FileDensity => "FileDensity",
            AsyncApiCallOutputType::FileMass => "FileMass",
            AsyncApiCallOutputType::FileVolume => "FileVolume",
        }
        .fmt(f)
    }
}

impl Default for AsyncApiCallOutputType {
    fn default() -> AsyncApiCallOutputType {
        AsyncApiCallOutputType::FileConversion
    }
}
impl std::str::FromStr for AsyncApiCallOutputType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "FileConversion" {
            return Ok(AsyncApiCallOutputType::FileConversion);
        }
        if s == "FileDensity" {
            return Ok(AsyncApiCallOutputType::FileDensity);
        }
        if s == "FileMass" {
            return Ok(AsyncApiCallOutputType::FileMass);
        }
        if s == "FileVolume" {
            return Ok(AsyncApiCallOutputType::FileVolume);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct AsyncApiCallResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[tabled(skip)]
    pub items: Vec<AsyncApiCall>,

    /**
    * token used to fetch the next page of results (if any)
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/// The billing information for payments.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct BillingInfo {
    /**
    * The billing information for payments.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
    * The address of the customer.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /**
    * The billing information for payments.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
}

/// Metadata about our cache.
///
/// This is mostly used for internal purposes and debugging.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct CacheMetadata {
    /**
    * If the cache returned an ok response from ping.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub ok: bool,
}

/// Card checks.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct PaymentMethodCardChecks {
    /**
    * Card checks.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "address_line1_check"
    )]
    pub address_line_1_check: String,

    /**
    * Card checks.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address_postal_code_check: String,

    /**
    * Card checks.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cvc_check: String,
}

/// The card details of a payment method.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct CardDetails {
    /**
    * The card details of a payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub brand: String,

    /**
    * The card details of a payment method.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<PaymentMethodCardChecks>,

    /**
    * The card details of a payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country: String,

    /**
    * The card details of a payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub exp_month: i64,

    /**
    * The card details of a payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub exp_year: i64,

    /**
    * The card details of a payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,

    /**
    * The card details of a payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub funding: String,

    /**
    * The card details of a payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "last4"
    )]
    pub last_4: String,
}

/// Cluster information.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Cluster {
    /**
    * Cluster information.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
    * The IP address of the cluster.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub addr: String,

    /**
    * Cluster information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub auth_timeout: i64,

    /**
    * Cluster information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub cluster_port: i64,

    /**
    * Cluster information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tls_timeout: i64,

    /**
    * Cluster information.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[tabled(skip)]
    pub urls: Vec<String>,
}

/**
* The language code is written in.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum CodeLanguage {
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CodeLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CodeLanguage::Go => "go",
            CodeLanguage::Node => "node",
            CodeLanguage::Python => "python",
            CodeLanguage::Rust => "rust",
            CodeLanguage::Noop => "",
            CodeLanguage::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CodeLanguage {
    fn default() -> CodeLanguage {
        CodeLanguage::Go
    }
}
impl std::str::FromStr for CodeLanguage {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "go" {
            return Ok(CodeLanguage::Go);
        }
        if s == "node" {
            return Ok(CodeLanguage::Node);
        }
        if s == "python" {
            return Ok(CodeLanguage::Python);
        }
        if s == "rust" {
            return Ok(CodeLanguage::Rust);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl CodeLanguage {
    pub fn is_noop(&self) -> bool {
        matches!(self, CodeLanguage::Noop)
    }
}

/// Output file contents.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct OutputFile {
    /**
    * Output file contents.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
    * Output file contents.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents: String,
}

/// Output of the code being executed.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct CodeOutput {
    /**
    * Output of the code being executed.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[tabled(skip)]
    pub output_files: Vec<OutputFile>,

    /**
    * Output of the code being executed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stderr: String,

    /**
    * Output of the code being executed.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stdout: String,
}

/// Gateway information.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Gateway {
    /**
    * Gateway information.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
    * Gateway information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub auth_timeout: i64,

    /**
    * Gateway information.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,

    /**
    * Gateway information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub port: i64,

    /**
    * Gateway information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tls_timeout: i64,
}

/// Jetstream information.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct Jetstream {
    /**
    * Jetstream information.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<JetstreamConfig>,

    /**
    * Jetstream information.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<MetaClusterInfo>,

    /**
    * Jetstream information.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<JetstreamStats>,
}

/// Leaf node information.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct LeafNode {
    /**
    * Leaf node information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub auth_timeout: i64,

    /**
    * Leaf node information.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,

    /**
    * Leaf node information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub port: i64,

    /**
    * Leaf node information.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tls_timeout: i64,
}

/// Metadata about a pub-sub connection.
///
/// This is mostly used for internal purposes and debugging.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct Connection {
    /**
    * The ID as known by the most recently connected server.
    */
    #[serde(default)]
    pub id: u64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub auth_timeout: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,

    /**
    * The time the configuration was loaded.
    */
    #[serde()]
    pub config_load_time: crate::utils::DisplayOptionDateTime,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub connections: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub cores: i64,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub cpu: f64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gateway>,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commit: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub go: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub gomaxprocs: i64,

    /**
    * The host of the server.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub http_base_path: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub http_host: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub http_port: i64,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub http_req_stats: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub https_port: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub in_bytes: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub in_msgs: i64,

    /**
    * The client IP as known by the most recently connected server.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ip: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jetstream: Option<Jetstream>,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaf: Option<LeafNode>,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub leafnodes: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_connections: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_control_line: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_payload: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_pending: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub mem: i64,

    /**
    * The time now.
    */
    #[serde()]
    pub now: crate::utils::DisplayOptionDateTime,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub out_bytes: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub out_msgs: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub ping_interval: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub ping_max: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub port: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub proto: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub remotes: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub routes: i64,

    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub rtt: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub server_id: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub server_name: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub slow_consumers: i64,

    /**
    * When the server was started.
    */
    #[serde()]
    pub start: crate::utils::DisplayOptionDateTime,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subscriptions: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub system_account: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tls_timeout: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total_connections: i64,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uptime: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub write_deadline: i64,
}

/**
* Supported set of sort modes for scanning by created_at only.
*   
*   Currently, we only support scanning in ascending order.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum CreatedAtSortMode {
    #[serde(rename = "created-at-ascending")]
    CreatedAtAscending,
    #[serde(rename = "created-at-descending")]
    CreatedAtDescending,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for CreatedAtSortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CreatedAtSortMode::CreatedAtAscending => "created-at-ascending",
            CreatedAtSortMode::CreatedAtDescending => "created-at-descending",
            CreatedAtSortMode::Noop => "",
            CreatedAtSortMode::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for CreatedAtSortMode {
    fn default() -> CreatedAtSortMode {
        CreatedAtSortMode::CreatedAtAscending
    }
}
impl std::str::FromStr for CreatedAtSortMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "created-at-ascending" {
            return Ok(CreatedAtSortMode::CreatedAtAscending);
        }
        if s == "created-at-descending" {
            return Ok(CreatedAtSortMode::CreatedAtDescending);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl CreatedAtSortMode {
    pub fn is_noop(&self) -> bool {
        matches!(self, CreatedAtSortMode::Noop)
    }
}

/**
* Currency is the list of supported currencies.
*   
*   For more details see <https://support.stripe.com/questions/which-currencies-does-stripe-support>.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum Currency {
    #[serde(rename = "aed")]
    Aed,
    #[serde(rename = "afn")]
    Afn,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "amd")]
    Amd,
    #[serde(rename = "ang")]
    Ang,
    #[serde(rename = "aoa")]
    Aoa,
    #[serde(rename = "ars")]
    Ars,
    #[serde(rename = "aud")]
    Aud,
    #[serde(rename = "awg")]
    Awg,
    #[serde(rename = "azn")]
    Azn,
    #[serde(rename = "bam")]
    Bam,
    #[serde(rename = "bbd")]
    Bbd,
    #[serde(rename = "bdt")]
    Bdt,
    #[serde(rename = "bgn")]
    Bgn,
    #[serde(rename = "bif")]
    Bif,
    #[serde(rename = "bmd")]
    Bmd,
    #[serde(rename = "bnd")]
    Bnd,
    #[serde(rename = "bob")]
    Bob,
    #[serde(rename = "brl")]
    Brl,
    #[serde(rename = "bsd")]
    Bsd,
    #[serde(rename = "bwp")]
    Bwp,
    #[serde(rename = "bzd")]
    Bzd,
    #[serde(rename = "cad")]
    Cad,
    #[serde(rename = "cdf")]
    Cdf,
    #[serde(rename = "chf")]
    Chf,
    #[serde(rename = "clp")]
    Clp,
    #[serde(rename = "cny")]
    Cny,
    #[serde(rename = "cop")]
    Cop,
    #[serde(rename = "crc")]
    Crc,
    #[serde(rename = "cve")]
    Cve,
    #[serde(rename = "czk")]
    Czk,
    #[serde(rename = "djf")]
    Djf,
    #[serde(rename = "dkk")]
    Dkk,
    #[serde(rename = "dop")]
    Dop,
    #[serde(rename = "dzd")]
    Dzd,
    #[serde(rename = "eek")]
    Eek,
    #[serde(rename = "egp")]
    Egp,
    #[serde(rename = "etb")]
    Etb,
    #[serde(rename = "eur")]
    Eur,
    #[serde(rename = "fjd")]
    Fjd,
    #[serde(rename = "fkp")]
    Fkp,
    #[serde(rename = "gbp")]
    Gbp,
    #[serde(rename = "gel")]
    Gel,
    #[serde(rename = "gip")]
    Gip,
    #[serde(rename = "gmd")]
    Gmd,
    #[serde(rename = "gnf")]
    Gnf,
    #[serde(rename = "gtq")]
    Gtq,
    #[serde(rename = "gyd")]
    Gyd,
    #[serde(rename = "hkd")]
    Hkd,
    #[serde(rename = "hnl")]
    Hnl,
    #[serde(rename = "hrk")]
    Hrk,
    #[serde(rename = "htg")]
    Htg,
    #[serde(rename = "huf")]
    Huf,
    #[serde(rename = "idr")]
    Idr,
    #[serde(rename = "ils")]
    Ils,
    #[serde(rename = "inr")]
    Inr,
    #[serde(rename = "isk")]
    Isk,
    #[serde(rename = "jmd")]
    Jmd,
    #[serde(rename = "jpy")]
    Jpy,
    #[serde(rename = "kes")]
    Kes,
    #[serde(rename = "kgs")]
    Kgs,
    #[serde(rename = "khr")]
    Khr,
    #[serde(rename = "kmf")]
    Kmf,
    #[serde(rename = "krw")]
    Krw,
    #[serde(rename = "kyd")]
    Kyd,
    #[serde(rename = "kzt")]
    Kzt,
    #[serde(rename = "lak")]
    Lak,
    #[serde(rename = "lbp")]
    Lbp,
    #[serde(rename = "lkr")]
    Lkr,
    #[serde(rename = "lrd")]
    Lrd,
    #[serde(rename = "lsl")]
    Lsl,
    #[serde(rename = "ltl")]
    Ltl,
    #[serde(rename = "lvl")]
    Lvl,
    #[serde(rename = "mad")]
    Mad,
    #[serde(rename = "mdl")]
    Mdl,
    #[serde(rename = "mga")]
    Mga,
    #[serde(rename = "mkd")]
    Mkd,
    #[serde(rename = "mnt")]
    Mnt,
    #[serde(rename = "mop")]
    Mop,
    #[serde(rename = "mro")]
    Mro,
    #[serde(rename = "mur")]
    Mur,
    #[serde(rename = "mvr")]
    Mvr,
    #[serde(rename = "mwk")]
    Mwk,
    #[serde(rename = "mxn")]
    Mxn,
    #[serde(rename = "myr")]
    Myr,
    #[serde(rename = "mzn")]
    Mzn,
    #[serde(rename = "nad")]
    Nad,
    #[serde(rename = "ngn")]
    Ngn,
    #[serde(rename = "nio")]
    Nio,
    #[serde(rename = "nok")]
    Nok,
    #[serde(rename = "npr")]
    Npr,
    #[serde(rename = "nzd")]
    Nzd,
    #[serde(rename = "pab")]
    Pab,
    #[serde(rename = "pen")]
    Pen,
    #[serde(rename = "pgk")]
    Pgk,
    #[serde(rename = "php")]
    Php,
    #[serde(rename = "pkr")]
    Pkr,
    #[serde(rename = "pln")]
    Pln,
    #[serde(rename = "pyg")]
    Pyg,
    #[serde(rename = "qar")]
    Qar,
    #[serde(rename = "ron")]
    Ron,
    #[serde(rename = "rsd")]
    Rsd,
    #[serde(rename = "rub")]
    Rub,
    #[serde(rename = "rwf")]
    Rwf,
    #[serde(rename = "sar")]
    Sar,
    #[serde(rename = "sbd")]
    Sbd,
    #[serde(rename = "scr")]
    Scr,
    #[serde(rename = "sek")]
    Sek,
    #[serde(rename = "sgd")]
    Sgd,
    #[serde(rename = "shp")]
    Shp,
    #[serde(rename = "sll")]
    Sll,
    #[serde(rename = "sos")]
    Sos,
    #[serde(rename = "srd")]
    Srd,
    #[serde(rename = "std")]
    Std,
    #[serde(rename = "svc")]
    Svc,
    #[serde(rename = "szl")]
    Szl,
    #[serde(rename = "thb")]
    Thb,
    #[serde(rename = "tjs")]
    Tjs,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "try")]
    Try,
    #[serde(rename = "ttd")]
    Ttd,
    #[serde(rename = "twd")]
    Twd,
    #[serde(rename = "tzs")]
    Tzs,
    #[serde(rename = "uah")]
    Uah,
    #[serde(rename = "ugx")]
    Ugx,
    #[serde(rename = "usd")]
    Usd,
    #[serde(rename = "uyu")]
    Uyu,
    #[serde(rename = "uzs")]
    Uzs,
    #[serde(rename = "vef")]
    Vef,
    #[serde(rename = "vnd")]
    Vnd,
    #[serde(rename = "vuv")]
    Vuv,
    #[serde(rename = "wst")]
    Wst,
    #[serde(rename = "xaf")]
    Xaf,
    #[serde(rename = "xcd")]
    Xcd,
    #[serde(rename = "xof")]
    Xof,
    #[serde(rename = "xpf")]
    Xpf,
    #[serde(rename = "yer")]
    Yer,
    #[serde(rename = "zar")]
    Zar,
    #[serde(rename = "zmw")]
    Zmw,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Currency::Aed => "aed",
            Currency::Afn => "afn",
            Currency::All => "all",
            Currency::Amd => "amd",
            Currency::Ang => "ang",
            Currency::Aoa => "aoa",
            Currency::Ars => "ars",
            Currency::Aud => "aud",
            Currency::Awg => "awg",
            Currency::Azn => "azn",
            Currency::Bam => "bam",
            Currency::Bbd => "bbd",
            Currency::Bdt => "bdt",
            Currency::Bgn => "bgn",
            Currency::Bif => "bif",
            Currency::Bmd => "bmd",
            Currency::Bnd => "bnd",
            Currency::Bob => "bob",
            Currency::Brl => "brl",
            Currency::Bsd => "bsd",
            Currency::Bwp => "bwp",
            Currency::Bzd => "bzd",
            Currency::Cad => "cad",
            Currency::Cdf => "cdf",
            Currency::Chf => "chf",
            Currency::Clp => "clp",
            Currency::Cny => "cny",
            Currency::Cop => "cop",
            Currency::Crc => "crc",
            Currency::Cve => "cve",
            Currency::Czk => "czk",
            Currency::Djf => "djf",
            Currency::Dkk => "dkk",
            Currency::Dop => "dop",
            Currency::Dzd => "dzd",
            Currency::Eek => "eek",
            Currency::Egp => "egp",
            Currency::Etb => "etb",
            Currency::Eur => "eur",
            Currency::Fjd => "fjd",
            Currency::Fkp => "fkp",
            Currency::Gbp => "gbp",
            Currency::Gel => "gel",
            Currency::Gip => "gip",
            Currency::Gmd => "gmd",
            Currency::Gnf => "gnf",
            Currency::Gtq => "gtq",
            Currency::Gyd => "gyd",
            Currency::Hkd => "hkd",
            Currency::Hnl => "hnl",
            Currency::Hrk => "hrk",
            Currency::Htg => "htg",
            Currency::Huf => "huf",
            Currency::Idr => "idr",
            Currency::Ils => "ils",
            Currency::Inr => "inr",
            Currency::Isk => "isk",
            Currency::Jmd => "jmd",
            Currency::Jpy => "jpy",
            Currency::Kes => "kes",
            Currency::Kgs => "kgs",
            Currency::Khr => "khr",
            Currency::Kmf => "kmf",
            Currency::Krw => "krw",
            Currency::Kyd => "kyd",
            Currency::Kzt => "kzt",
            Currency::Lak => "lak",
            Currency::Lbp => "lbp",
            Currency::Lkr => "lkr",
            Currency::Lrd => "lrd",
            Currency::Lsl => "lsl",
            Currency::Ltl => "ltl",
            Currency::Lvl => "lvl",
            Currency::Mad => "mad",
            Currency::Mdl => "mdl",
            Currency::Mga => "mga",
            Currency::Mkd => "mkd",
            Currency::Mnt => "mnt",
            Currency::Mop => "mop",
            Currency::Mro => "mro",
            Currency::Mur => "mur",
            Currency::Mvr => "mvr",
            Currency::Mwk => "mwk",
            Currency::Mxn => "mxn",
            Currency::Myr => "myr",
            Currency::Mzn => "mzn",
            Currency::Nad => "nad",
            Currency::Ngn => "ngn",
            Currency::Nio => "nio",
            Currency::Nok => "nok",
            Currency::Npr => "npr",
            Currency::Nzd => "nzd",
            Currency::Pab => "pab",
            Currency::Pen => "pen",
            Currency::Pgk => "pgk",
            Currency::Php => "php",
            Currency::Pkr => "pkr",
            Currency::Pln => "pln",
            Currency::Pyg => "pyg",
            Currency::Qar => "qar",
            Currency::Ron => "ron",
            Currency::Rsd => "rsd",
            Currency::Rub => "rub",
            Currency::Rwf => "rwf",
            Currency::Sar => "sar",
            Currency::Sbd => "sbd",
            Currency::Scr => "scr",
            Currency::Sek => "sek",
            Currency::Sgd => "sgd",
            Currency::Shp => "shp",
            Currency::Sll => "sll",
            Currency::Sos => "sos",
            Currency::Srd => "srd",
            Currency::Std => "std",
            Currency::Svc => "svc",
            Currency::Szl => "szl",
            Currency::Thb => "thb",
            Currency::Tjs => "tjs",
            Currency::Top => "top",
            Currency::Try => "try",
            Currency::Ttd => "ttd",
            Currency::Twd => "twd",
            Currency::Tzs => "tzs",
            Currency::Uah => "uah",
            Currency::Ugx => "ugx",
            Currency::Usd => "usd",
            Currency::Uyu => "uyu",
            Currency::Uzs => "uzs",
            Currency::Vef => "vef",
            Currency::Vnd => "vnd",
            Currency::Vuv => "vuv",
            Currency::Wst => "wst",
            Currency::Xaf => "xaf",
            Currency::Xcd => "xcd",
            Currency::Xof => "xof",
            Currency::Xpf => "xpf",
            Currency::Yer => "yer",
            Currency::Zar => "zar",
            Currency::Zmw => "zmw",
            Currency::Noop => "",
            Currency::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Currency {
    fn default() -> Currency {
        Currency::Aed
    }
}
impl std::str::FromStr for Currency {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "aed" {
            return Ok(Currency::Aed);
        }
        if s == "afn" {
            return Ok(Currency::Afn);
        }
        if s == "all" {
            return Ok(Currency::All);
        }
        if s == "amd" {
            return Ok(Currency::Amd);
        }
        if s == "ang" {
            return Ok(Currency::Ang);
        }
        if s == "aoa" {
            return Ok(Currency::Aoa);
        }
        if s == "ars" {
            return Ok(Currency::Ars);
        }
        if s == "aud" {
            return Ok(Currency::Aud);
        }
        if s == "awg" {
            return Ok(Currency::Awg);
        }
        if s == "azn" {
            return Ok(Currency::Azn);
        }
        if s == "bam" {
            return Ok(Currency::Bam);
        }
        if s == "bbd" {
            return Ok(Currency::Bbd);
        }
        if s == "bdt" {
            return Ok(Currency::Bdt);
        }
        if s == "bgn" {
            return Ok(Currency::Bgn);
        }
        if s == "bif" {
            return Ok(Currency::Bif);
        }
        if s == "bmd" {
            return Ok(Currency::Bmd);
        }
        if s == "bnd" {
            return Ok(Currency::Bnd);
        }
        if s == "bob" {
            return Ok(Currency::Bob);
        }
        if s == "brl" {
            return Ok(Currency::Brl);
        }
        if s == "bsd" {
            return Ok(Currency::Bsd);
        }
        if s == "bwp" {
            return Ok(Currency::Bwp);
        }
        if s == "bzd" {
            return Ok(Currency::Bzd);
        }
        if s == "cad" {
            return Ok(Currency::Cad);
        }
        if s == "cdf" {
            return Ok(Currency::Cdf);
        }
        if s == "chf" {
            return Ok(Currency::Chf);
        }
        if s == "clp" {
            return Ok(Currency::Clp);
        }
        if s == "cny" {
            return Ok(Currency::Cny);
        }
        if s == "cop" {
            return Ok(Currency::Cop);
        }
        if s == "crc" {
            return Ok(Currency::Crc);
        }
        if s == "cve" {
            return Ok(Currency::Cve);
        }
        if s == "czk" {
            return Ok(Currency::Czk);
        }
        if s == "djf" {
            return Ok(Currency::Djf);
        }
        if s == "dkk" {
            return Ok(Currency::Dkk);
        }
        if s == "dop" {
            return Ok(Currency::Dop);
        }
        if s == "dzd" {
            return Ok(Currency::Dzd);
        }
        if s == "eek" {
            return Ok(Currency::Eek);
        }
        if s == "egp" {
            return Ok(Currency::Egp);
        }
        if s == "etb" {
            return Ok(Currency::Etb);
        }
        if s == "eur" {
            return Ok(Currency::Eur);
        }
        if s == "fjd" {
            return Ok(Currency::Fjd);
        }
        if s == "fkp" {
            return Ok(Currency::Fkp);
        }
        if s == "gbp" {
            return Ok(Currency::Gbp);
        }
        if s == "gel" {
            return Ok(Currency::Gel);
        }
        if s == "gip" {
            return Ok(Currency::Gip);
        }
        if s == "gmd" {
            return Ok(Currency::Gmd);
        }
        if s == "gnf" {
            return Ok(Currency::Gnf);
        }
        if s == "gtq" {
            return Ok(Currency::Gtq);
        }
        if s == "gyd" {
            return Ok(Currency::Gyd);
        }
        if s == "hkd" {
            return Ok(Currency::Hkd);
        }
        if s == "hnl" {
            return Ok(Currency::Hnl);
        }
        if s == "hrk" {
            return Ok(Currency::Hrk);
        }
        if s == "htg" {
            return Ok(Currency::Htg);
        }
        if s == "huf" {
            return Ok(Currency::Huf);
        }
        if s == "idr" {
            return Ok(Currency::Idr);
        }
        if s == "ils" {
            return Ok(Currency::Ils);
        }
        if s == "inr" {
            return Ok(Currency::Inr);
        }
        if s == "isk" {
            return Ok(Currency::Isk);
        }
        if s == "jmd" {
            return Ok(Currency::Jmd);
        }
        if s == "jpy" {
            return Ok(Currency::Jpy);
        }
        if s == "kes" {
            return Ok(Currency::Kes);
        }
        if s == "kgs" {
            return Ok(Currency::Kgs);
        }
        if s == "khr" {
            return Ok(Currency::Khr);
        }
        if s == "kmf" {
            return Ok(Currency::Kmf);
        }
        if s == "krw" {
            return Ok(Currency::Krw);
        }
        if s == "kyd" {
            return Ok(Currency::Kyd);
        }
        if s == "kzt" {
            return Ok(Currency::Kzt);
        }
        if s == "lak" {
            return Ok(Currency::Lak);
        }
        if s == "lbp" {
            return Ok(Currency::Lbp);
        }
        if s == "lkr" {
            return Ok(Currency::Lkr);
        }
        if s == "lrd" {
            return Ok(Currency::Lrd);
        }
        if s == "lsl" {
            return Ok(Currency::Lsl);
        }
        if s == "ltl" {
            return Ok(Currency::Ltl);
        }
        if s == "lvl" {
            return Ok(Currency::Lvl);
        }
        if s == "mad" {
            return Ok(Currency::Mad);
        }
        if s == "mdl" {
            return Ok(Currency::Mdl);
        }
        if s == "mga" {
            return Ok(Currency::Mga);
        }
        if s == "mkd" {
            return Ok(Currency::Mkd);
        }
        if s == "mnt" {
            return Ok(Currency::Mnt);
        }
        if s == "mop" {
            return Ok(Currency::Mop);
        }
        if s == "mro" {
            return Ok(Currency::Mro);
        }
        if s == "mur" {
            return Ok(Currency::Mur);
        }
        if s == "mvr" {
            return Ok(Currency::Mvr);
        }
        if s == "mwk" {
            return Ok(Currency::Mwk);
        }
        if s == "mxn" {
            return Ok(Currency::Mxn);
        }
        if s == "myr" {
            return Ok(Currency::Myr);
        }
        if s == "mzn" {
            return Ok(Currency::Mzn);
        }
        if s == "nad" {
            return Ok(Currency::Nad);
        }
        if s == "ngn" {
            return Ok(Currency::Ngn);
        }
        if s == "nio" {
            return Ok(Currency::Nio);
        }
        if s == "nok" {
            return Ok(Currency::Nok);
        }
        if s == "npr" {
            return Ok(Currency::Npr);
        }
        if s == "nzd" {
            return Ok(Currency::Nzd);
        }
        if s == "pab" {
            return Ok(Currency::Pab);
        }
        if s == "pen" {
            return Ok(Currency::Pen);
        }
        if s == "pgk" {
            return Ok(Currency::Pgk);
        }
        if s == "php" {
            return Ok(Currency::Php);
        }
        if s == "pkr" {
            return Ok(Currency::Pkr);
        }
        if s == "pln" {
            return Ok(Currency::Pln);
        }
        if s == "pyg" {
            return Ok(Currency::Pyg);
        }
        if s == "qar" {
            return Ok(Currency::Qar);
        }
        if s == "ron" {
            return Ok(Currency::Ron);
        }
        if s == "rsd" {
            return Ok(Currency::Rsd);
        }
        if s == "rub" {
            return Ok(Currency::Rub);
        }
        if s == "rwf" {
            return Ok(Currency::Rwf);
        }
        if s == "sar" {
            return Ok(Currency::Sar);
        }
        if s == "sbd" {
            return Ok(Currency::Sbd);
        }
        if s == "scr" {
            return Ok(Currency::Scr);
        }
        if s == "sek" {
            return Ok(Currency::Sek);
        }
        if s == "sgd" {
            return Ok(Currency::Sgd);
        }
        if s == "shp" {
            return Ok(Currency::Shp);
        }
        if s == "sll" {
            return Ok(Currency::Sll);
        }
        if s == "sos" {
            return Ok(Currency::Sos);
        }
        if s == "srd" {
            return Ok(Currency::Srd);
        }
        if s == "std" {
            return Ok(Currency::Std);
        }
        if s == "svc" {
            return Ok(Currency::Svc);
        }
        if s == "szl" {
            return Ok(Currency::Szl);
        }
        if s == "thb" {
            return Ok(Currency::Thb);
        }
        if s == "tjs" {
            return Ok(Currency::Tjs);
        }
        if s == "top" {
            return Ok(Currency::Top);
        }
        if s == "try" {
            return Ok(Currency::Try);
        }
        if s == "ttd" {
            return Ok(Currency::Ttd);
        }
        if s == "twd" {
            return Ok(Currency::Twd);
        }
        if s == "tzs" {
            return Ok(Currency::Tzs);
        }
        if s == "uah" {
            return Ok(Currency::Uah);
        }
        if s == "ugx" {
            return Ok(Currency::Ugx);
        }
        if s == "usd" {
            return Ok(Currency::Usd);
        }
        if s == "uyu" {
            return Ok(Currency::Uyu);
        }
        if s == "uzs" {
            return Ok(Currency::Uzs);
        }
        if s == "vef" {
            return Ok(Currency::Vef);
        }
        if s == "vnd" {
            return Ok(Currency::Vnd);
        }
        if s == "vuv" {
            return Ok(Currency::Vuv);
        }
        if s == "wst" {
            return Ok(Currency::Wst);
        }
        if s == "xaf" {
            return Ok(Currency::Xaf);
        }
        if s == "xcd" {
            return Ok(Currency::Xcd);
        }
        if s == "xof" {
            return Ok(Currency::Xof);
        }
        if s == "xpf" {
            return Ok(Currency::Xpf);
        }
        if s == "yer" {
            return Ok(Currency::Yer);
        }
        if s == "zar" {
            return Ok(Currency::Zar);
        }
        if s == "zmw" {
            return Ok(Currency::Zmw);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl Currency {
    pub fn is_noop(&self) -> bool {
        matches!(self, Currency::Noop)
    }
}

/// The resource representing a payment "Customer".
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct Customer {
    /**
    * The resource representing a payment "Customer".
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The resource representing a payment "Customer".
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
    * The customer's address.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /**
    * The resource representing a payment "Customer".
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub balance: i64,

    /**
    * Time at which the object was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * Currency is the list of supported currencies.
    *  
    *  For more details see <https://support.stripe.com/questions/which-currencies-does-stripe-support>.
    */
    #[serde(default, skip_serializing_if = "Currency::is_noop")]
    pub currency: Currency,

    /**
    * The resource representing a payment "Customer".
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub delinquent: bool,

    /**
    * The resource representing a payment "Customer".
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,

    /**
    * The resource representing a payment "Customer".
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,

    /**
    * The resource representing a payment "Customer".
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
}

/**
* The environment the server is running in.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum Environment {
    #[serde(rename = "DEVELOPMENT")]
    Development,
    #[serde(rename = "PREVIEW")]
    Preview,
    #[serde(rename = "PRODUCTION")]
    Production,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Environment::Development => "DEVELOPMENT",
            Environment::Preview => "PREVIEW",
            Environment::Production => "PRODUCTION",
            Environment::Noop => "",
            Environment::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for Environment {
    fn default() -> Environment {
        Environment::Development
    }
}
impl std::str::FromStr for Environment {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "DEVELOPMENT" {
            return Ok(Environment::Development);
        }
        if s == "PREVIEW" {
            return Ok(Environment::Preview);
        }
        if s == "PRODUCTION" {
            return Ok(Environment::Production);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl Environment {
    pub fn is_noop(&self) -> bool {
        matches!(self, Environment::Noop)
    }
}

/// Metadata about our file system.
///
/// This is mostly used for internal purposes and debugging.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct FileSystemMetadata {
    /**
    * If the file system passed a sanity check.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub ok: bool,
}

/// Metadata about our currently running server.
///
/// This is mostly used for internal purposes and debugging.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct EngineMetadata {
    /**
    * If any async job is currently running.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub async_jobs_running: bool,

    /**
    * Metadata about our cache.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde()]
    pub cache: CacheMetadata,

    /**
    * The environment the server is running in.
    */
    #[serde(default, skip_serializing_if = "Environment::is_noop")]
    pub environment: Environment,

    /**
    * Metadata about our file system.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde()]
    pub fs: FileSystemMetadata,

    /**
    * The git hash of the server.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_hash: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde()]
    pub pubsub: Connection,
}

#[derive(Debug, Deserialize, thiserror::Error, PartialEq, Serialize)]
pub enum Error {
    /// An object needed as part of this operation was not found.
    #[error("Object not found: {message}")]
    ObjectNotFound {
        /// A message describing the problem.
        message: String,
    },
    /// An object already exists with the specified name or identifier.
    #[error("Object already exists: {message}")]
    ObjectAlreadyExists {
        /// A message describing the problem.
        message: String,
    },
    /// The request was well-formed, but the operation cannot be completed given
    /// the current state of the system.
    #[error("Invalid Request: {message}")]
    InvalidRequest {
        /// A message describing the problem.
        message: String,
    },
    /// Authentication credentials were required but either missing or invalid.
    /// The HTTP status code is called "Unauthorized", but it's more accurate to
    /// call it "Unauthenticated".
    #[error("Missing or invalid credentials")]
    Unauthenticated {
        /// An internal message.
        internal_message: String,
    },
    /// The specified input field is not valid.
    #[error("Invalid Value: {message}")]
    InvalidValue {
        /// A message describing the problem.
        message: String,
    },
    /// The request is not authorized to perform the requested operation.
    #[error("Forbidden")]
    Forbidden,

    /// The system encountered an unhandled operational error.
    #[error("Internal Error: {internal_message}")]
    InternalError {
        /// An internal message.
        internal_message: String,
    },
    /// The system (or part of it) is unavailable.
    #[error("Service Unavailable: {internal_message}")]
    ServiceUnavailable {
        /// An internal message.
        internal_message: String,
    },
    /// Method Not Allowed
    #[error("Method Not Allowed: {internal_message}")]
    MethodNotAllowed {
        /// An internal message.
        internal_message: String,
    },
}

impl Error {
    /// Returns whether the error is likely transient and could reasonably be
    /// retried
    pub fn retryable(&self) -> bool {
        match self {
            Error::ServiceUnavailable { .. } => true,

            Error::ObjectNotFound { .. }
            | Error::ObjectAlreadyExists { .. }
            | Error::Unauthenticated { .. }
            | Error::InvalidRequest { .. }
            | Error::InvalidValue { .. }
            | Error::Forbidden
            | Error::MethodNotAllowed { .. }
            | Error::InternalError { .. } => false,
        }
    }
}

impl From<ErrorResponse> for Error {
    /// Converts an `Error` error into an `HttpError`.  This defines how
    /// errors that are represented internally using `Error` are ultimately
    /// exposed to clients over HTTP.
    fn from(error: ErrorResponse) -> Error {
        if error.error_code == "ObjectNotFound" {
            return Error::ObjectNotFound {
                message: error.message,
            };
        }

        if error.error_code == "ObjectAlreadyExists" {
            return Error::ObjectAlreadyExists {
                message: error.message,
            };
        }

        if error.error_code == "Unauthorized" {
            return Error::Unauthenticated {
                internal_message: error.message,
            };
        }

        if error.error_code == "InvalidRequest" {
            return Error::InvalidRequest {
                message: error.message,
            };
        }

        if error.error_code == "InvalidValue" {
            return Error::InvalidValue {
                message: error.message,
            };
        }

        if error.error_code == "Forbidden" {
            return Error::Forbidden;
        }

        if error.error_code == "MethodNotAllowed" {
            return Error::MethodNotAllowed {
                internal_message: error.message,
            };
        }

        if error.error_code == "ServiceUnavailable" {
            return Error::ServiceUnavailable {
                internal_message: error.message,
            };
        }

        Error::InternalError {
            internal_message: error.message,
        }
    }
}

/// Identifies a type of API resource
#[derive(
    Clone,
    Copy,
    Debug,
    serde_with::DeserializeFromStr,
    Display,
    Eq,
    FromStr,
    Ord,
    PartialEq,
    PartialOrd,
    serde_with::SerializeDisplay,
)]
#[display(style = "kebab-case")]
pub enum ResourceType {
    /// An address.
    Address,
    /// An API call.
    #[display("api-call")]
    APICall,
    /// An API call price.
    #[display("api-call-price")]
    APICallPrice,
    /// An API call with price.
    #[display("api-call-with-price")]
    APICallWithPrice,
    /// An API token.
    #[display("api-token")]
    APIToken,
    /// An async API call.
    #[display("async-api-call")]
    AsyncAPICall,
    /// An extended user.
    ExtendedUser,
    /// A file conversion.
    FileConversion,
    /// A MailChimp subscriber.
    MailChimpSubscriber,
    /// A session.
    Session,
    /// A Stripe customer.
    StripeCustomer,
    /// A user.
    User,
    /// A Zendesk contact.
    ZendeskContact,
}

/// Error information from a response.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ErrorResponse {
    /**
    * Error information from a response.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_code: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub request_id: String,
}

/// Extended user information.
///
/// This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ExtendedUser {
    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,

    /**
    * The date and time the user was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discord: String,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,

    /**
    * The date and time the email address was verified.
    */
    #[serde()]
    pub email_verified: crate::utils::DisplayOptionDateTime,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub github: String,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image: String,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,

    /**
    * The user's MailChimp ID. This is mostly used for internal mapping.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mailchimp_id: String,

    /**
    * Extended user information.
    *  
    *  This is mostly used for internal purposes. It returns a mapping of the user's information, including that of our third party services we use for users: MailChimp, Stripe, and Zendesk.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,

    /**
    * The user's Stripe ID. This is mostly used for internal mapping.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stripe_id: String,

    /**
    * The date and time the user was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * The user's Zendesk ID. This is mostly used for internal mapping.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zendesk_id: String,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct ExtendedUserResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[tabled(skip)]
    pub items: Vec<ExtendedUser>,

    /**
    * token used to fetch the next page of results (if any)
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

/**
* The valid types of output file formats.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum FileOutputFormat {
    #[serde(rename = "dae")]
    Dae,
    #[serde(rename = "fbx")]
    Fbx,
    #[serde(rename = "fbxb")]
    Fbxb,
    #[serde(rename = "obj")]
    Obj,
    #[serde(rename = "step")]
    Step,
    #[serde(rename = "stl")]
    Stl,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FileOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FileOutputFormat::Dae => "dae",
            FileOutputFormat::Fbx => "fbx",
            FileOutputFormat::Fbxb => "fbxb",
            FileOutputFormat::Obj => "obj",
            FileOutputFormat::Step => "step",
            FileOutputFormat::Stl => "stl",
            FileOutputFormat::Noop => "",
            FileOutputFormat::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FileOutputFormat {
    fn default() -> FileOutputFormat {
        FileOutputFormat::Dae
    }
}
impl std::str::FromStr for FileOutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "dae" {
            return Ok(FileOutputFormat::Dae);
        }
        if s == "fbx" {
            return Ok(FileOutputFormat::Fbx);
        }
        if s == "fbxb" {
            return Ok(FileOutputFormat::Fbxb);
        }
        if s == "obj" {
            return Ok(FileOutputFormat::Obj);
        }
        if s == "step" {
            return Ok(FileOutputFormat::Step);
        }
        if s == "stl" {
            return Ok(FileOutputFormat::Stl);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl FileOutputFormat {
    pub fn is_noop(&self) -> bool {
        matches!(self, FileOutputFormat::Noop)
    }
}

/**
* The valid types of source file formats.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum FileSourceFormat {
    #[serde(rename = "dae")]
    Dae,
    #[serde(rename = "fbx")]
    Fbx,
    #[serde(rename = "obj")]
    Obj,
    #[serde(rename = "step")]
    Step,
    #[serde(rename = "stl")]
    Stl,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for FileSourceFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            FileSourceFormat::Dae => "dae",
            FileSourceFormat::Fbx => "fbx",
            FileSourceFormat::Obj => "obj",
            FileSourceFormat::Step => "step",
            FileSourceFormat::Stl => "stl",
            FileSourceFormat::Noop => "",
            FileSourceFormat::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for FileSourceFormat {
    fn default() -> FileSourceFormat {
        FileSourceFormat::Dae
    }
}
impl std::str::FromStr for FileSourceFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "dae" {
            return Ok(FileSourceFormat::Dae);
        }
        if s == "fbx" {
            return Ok(FileSourceFormat::Fbx);
        }
        if s == "obj" {
            return Ok(FileSourceFormat::Obj);
        }
        if s == "step" {
            return Ok(FileSourceFormat::Step);
        }
        if s == "stl" {
            return Ok(FileSourceFormat::Stl);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl FileSourceFormat {
    pub fn is_noop(&self) -> bool {
        matches!(self, FileSourceFormat::Noop)
    }
}

/// A file conversion.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct FileConversion {
    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The time and date the file conversion was completed.
    */
    #[serde()]
    pub completed_at: crate::utils::DisplayOptionDateTime,

    /**
    * The time and date the file conversion was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * The error the function returned, if any.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,

    /**
    * The converted file, if completed, base64 encoded.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub output: String,

    /**
    * The valid types of output file formats.
    */
    #[serde(default, skip_serializing_if = "FileOutputFormat::is_noop")]
    pub output_format: FileOutputFormat,

    /**
    * The valid types of source file formats.
    */
    #[serde(default, skip_serializing_if = "FileSourceFormat::is_noop")]
    pub src_format: FileSourceFormat,

    /**
    * The time and date the file conversion was started.
    */
    #[serde()]
    pub started_at: crate::utils::DisplayOptionDateTime,

    /**
    * The status of an async API call.
    */
    #[serde(default, skip_serializing_if = "ApiCallStatus::is_noop")]
    pub status: ApiCallStatus,

    /**
    * The time and date the file conversion was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * A file conversion.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

/// A file density result.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct FileDensity {
    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The time and date the density was completed.
    */
    #[serde()]
    pub completed_at: crate::utils::DisplayOptionDateTime,

    /**
    * The time and date the density was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * The resulting density.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub density: f64,

    /**
    * The error the function returned, if any.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,

    /**
    * A file density result.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub material_mass: f64,

    /**
    * The valid types of source file formats.
    */
    #[serde(default, skip_serializing_if = "FileSourceFormat::is_noop")]
    pub src_format: FileSourceFormat,

    /**
    * The time and date the density was started.
    */
    #[serde()]
    pub started_at: crate::utils::DisplayOptionDateTime,

    /**
    * The status of an async API call.
    */
    #[serde(default, skip_serializing_if = "ApiCallStatus::is_noop")]
    pub status: ApiCallStatus,

    /**
    * The time and date the density was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * A file density result.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

/// A file mass result.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct FileMass {
    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The time and date the mass was completed.
    */
    #[serde()]
    pub completed_at: crate::utils::DisplayOptionDateTime,

    /**
    * The time and date the mass was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * The error the function returned, if any.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,

    /**
    * The resulting mass.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub mass: f64,

    /**
    * A file mass result.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub material_density: f64,

    /**
    * The valid types of source file formats.
    */
    #[serde(default, skip_serializing_if = "FileSourceFormat::is_noop")]
    pub src_format: FileSourceFormat,

    /**
    * The time and date the mass was started.
    */
    #[serde()]
    pub started_at: crate::utils::DisplayOptionDateTime,

    /**
    * The status of an async API call.
    */
    #[serde(default, skip_serializing_if = "ApiCallStatus::is_noop")]
    pub status: ApiCallStatus,

    /**
    * The time and date the mass was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * A file mass result.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

/// A file volume result.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct FileVolume {
    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The time and date the volume was completed.
    */
    #[serde()]
    pub completed_at: crate::utils::DisplayOptionDateTime,

    /**
    * The time and date the volume was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * The error the function returned, if any.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,

    /**
    * The valid types of source file formats.
    */
    #[serde(default, skip_serializing_if = "FileSourceFormat::is_noop")]
    pub src_format: FileSourceFormat,

    /**
    * The time and date the volume was started.
    */
    #[serde()]
    pub started_at: crate::utils::DisplayOptionDateTime,

    /**
    * The status of an async API call.
    */
    #[serde(default, skip_serializing_if = "ApiCallStatus::is_noop")]
    pub status: ApiCallStatus,

    /**
    * The time and date the volume was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * A file volume result.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,

    /**
    * The resulting volume.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub volume: f64,
}

/// An invoice line item.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct InvoiceLineItem {
    /**
    * An invoice line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * An invoice line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,

    /**
    * An invoice line item.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub amount: i64,

    /**
    * Currency is the list of supported currencies.
    *  
    *  For more details see <https://support.stripe.com/questions/which-currencies-does-stripe-support>.
    */
    #[serde(default, skip_serializing_if = "Currency::is_noop")]
    pub currency: Currency,

    /**
    * An invoice line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub invoice_item: String,

    /**
    * An invoice line item.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,
}

/**
* An enum representing the possible values of an `Invoice`'s `status` field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum InvoiceStatus {
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "uncollectible")]
    Uncollectible,
    #[serde(rename = "void")]
    Void,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            InvoiceStatus::Deleted => "deleted",
            InvoiceStatus::Draft => "draft",
            InvoiceStatus::Open => "open",
            InvoiceStatus::Paid => "paid",
            InvoiceStatus::Uncollectible => "uncollectible",
            InvoiceStatus::Void => "void",
            InvoiceStatus::Noop => "",
            InvoiceStatus::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for InvoiceStatus {
    fn default() -> InvoiceStatus {
        InvoiceStatus::Deleted
    }
}
impl std::str::FromStr for InvoiceStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deleted" {
            return Ok(InvoiceStatus::Deleted);
        }
        if s == "draft" {
            return Ok(InvoiceStatus::Draft);
        }
        if s == "open" {
            return Ok(InvoiceStatus::Open);
        }
        if s == "paid" {
            return Ok(InvoiceStatus::Paid);
        }
        if s == "uncollectible" {
            return Ok(InvoiceStatus::Uncollectible);
        }
        if s == "void" {
            return Ok(InvoiceStatus::Void);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl InvoiceStatus {
    pub fn is_noop(&self) -> bool {
        matches!(self, InvoiceStatus::Noop)
    }
}

/// An invoice.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct Invoice {
    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub amount_due: i64,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub amount_paid: i64,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub amount_remaining: i64,

    /**
    * An invoice.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt_count: Option<u64>,

    /**
    * An invoice.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub attempted: bool,

    /**
    * Time at which the object was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * Currency is the list of supported currencies.
    *  
    *  For more details see <https://support.stripe.com/questions/which-currencies-does-stripe-support>.
    */
    #[serde(default, skip_serializing_if = "Currency::is_noop")]
    pub currency: Currency,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub invoice_pdf: String,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub invoice_url: String,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    pub lines: Vec<InvoiceLineItem>,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub number: String,

    /**
    * An invoice.
    */
    #[serde(
        default,
        deserialize_with = "crate::utils::deserialize_null_boolean::deserialize"
    )]
    pub paid: bool,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub receipt_number: String,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statement_descriptor: String,

    /**
    * The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    *  
    *  [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub subtotal: i64,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub tax: i64,

    /**
    * An invoice.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total: i64,
}

/// Jetstream configuration.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct JetstreamConfig {
    /**
    * Jetstream configuration.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub domain: String,

    /**
    * Jetstream configuration.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_memory: i64,

    /**
    * Jetstream configuration.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub max_storage: i64,

    /**
    * Jetstream configuration.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub store_dir: String,
}

/// Jetstream statistics.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct MetaClusterInfo {
    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub cluster_size: i64,

    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub leader: String,
}

/// Jetstream statistics.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct JetstreamStats {
    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub accounts: i64,

    /**
    * Jetstream statistics.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<JetstreamApiStats>,

    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub ha_assets: i64,

    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub memory: i64,

    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reserved_memory: i64,

    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub reserved_store: i64,

    /**
    * Jetstream statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub store: i64,
}

/// Jetstream API statistics.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct JetstreamApiStats {
    /**
    * Jetstream API statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub errors: i64,

    /**
    * Jetstream API statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub inflight: i64,

    /**
    * Jetstream API statistics.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_i64",
        deserialize_with = "crate::utils::deserialize_null_i64::deserialize"
    )]
    pub total: i64,
}

/// The parameters passed to login.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct LoginParams {
    /**
    * The session token we should set as a cookie.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub session: String,
}

/// Metadata about our currently running server.
///
/// This is mostly used for internal purposes and debugging.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct Metadata {
    /**
    * Metadata about our cache.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde()]
    pub cache: CacheMetadata,

    /**
    * Metadata about our currently running server.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde()]
    pub engine: EngineMetadata,

    /**
    * The environment the server is running in.
    */
    #[serde(default, skip_serializing_if = "Environment::is_noop")]
    pub environment: Environment,

    /**
    * Metadata about our file system.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde()]
    pub fs: FileSystemMetadata,

    /**
    * The git hash of the server.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_hash: String,

    /**
    * Metadata about a pub-sub connection.
    *  
    *  This is mostly used for internal purposes and debugging.
    */
    #[serde()]
    pub pubsub: Connection,
}

/// A payment intent response.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct PaymentIntent {
    /**
    * The client secret is used for client-side retrieval using a publishable key. The client secret can be used to complete payment setup from your frontend. It should not be stored, logged, or exposed to anyone other than the customer. Make sure that you have TLS enabled on any page that includes the client secret.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
}

/**
* An enum representing the possible values of an `PaymentMethod`'s `type` field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum PaymentMethodType {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PaymentMethodType::Card => "card",
            PaymentMethodType::Noop => "",
            PaymentMethodType::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PaymentMethodType {
    fn default() -> PaymentMethodType {
        PaymentMethodType::Card
    }
}
impl std::str::FromStr for PaymentMethodType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "card" {
            return Ok(PaymentMethodType::Card);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl PaymentMethodType {
    pub fn is_noop(&self) -> bool {
        matches!(self, PaymentMethodType::Noop)
    }
}

/// A payment method.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default)]
pub struct PaymentMethod {
    /**
    * A payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The billing information for payments.
    */
    #[serde()]
    pub billing_info: BillingInfo,

    /**
    * The card, if it is one. For our purposes, this is the only type of payment method that we support.
    */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,

    /**
    * Time at which the object was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * A payment method.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,

    /**
    * An enum representing the possible values of an `PaymentMethod`'s `type` field.
    */
    #[serde(
        default,
        skip_serializing_if = "PaymentMethodType::is_noop",
        rename = "type"
    )]
    pub type_: PaymentMethodType,
}

/**
* An enum representing the possible values of an `PaymentMethod`'s `type` field.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum PaymentMethodTypeCard {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for PaymentMethodTypeCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            PaymentMethodTypeCard::Card => "card",
            PaymentMethodTypeCard::Noop => "",
            PaymentMethodTypeCard::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for PaymentMethodTypeCard {
    fn default() -> PaymentMethodTypeCard {
        PaymentMethodTypeCard::Card
    }
}
impl std::str::FromStr for PaymentMethodTypeCard {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "card" {
            return Ok(PaymentMethodTypeCard::Card);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl PaymentMethodTypeCard {
    pub fn is_noop(&self) -> bool {
        matches!(self, PaymentMethodTypeCard::Noop)
    }
}

/// The response from the `/ping` endpoint.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Pong {
    /**
    * The pong response.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// An authentication session.
///
/// For our UIs, these are automatically created by Next.js.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct Session {
    /**
    * An authentication session.
    *  
    *  For our UIs, these are automatically created by Next.js.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The date and time the session was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * The date and time the session expires.
    */
    #[serde()]
    pub expires: crate::utils::DisplayOptionDateTime,

    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub session_token: String,

    /**
    * The date and time the session was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * An authentication session.
    *  
    *  For our UIs, these are automatically created by Next.js.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

/**
* The valid types of metric unit formats.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Tabled)]
pub enum UnitMetricFormat {
    #[serde(rename = "atto")]
    Atto,
    #[serde(rename = "centi")]
    Centi,
    #[serde(rename = "deca")]
    Deca,
    #[serde(rename = "deci")]
    Deci,
    #[serde(rename = "exa")]
    Exa,
    #[serde(rename = "femto")]
    Femto,
    #[serde(rename = "giga")]
    Giga,
    #[serde(rename = "hecto")]
    Hecto,
    #[serde(rename = "kilo")]
    Kilo,
    #[serde(rename = "mega")]
    Mega,
    #[serde(rename = "metric_unit")]
    MetricUnit,
    #[serde(rename = "micro")]
    Micro,
    #[serde(rename = "milli")]
    Milli,
    #[serde(rename = "nano")]
    Nano,
    #[serde(rename = "peta")]
    Peta,
    #[serde(rename = "pico")]
    Pico,
    #[serde(rename = "tera")]
    Tera,
    #[serde(rename = "")]
    Noop,
    #[serde(other)]
    FallthroughString,
}

impl std::fmt::Display for UnitMetricFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            UnitMetricFormat::Atto => "atto",
            UnitMetricFormat::Centi => "centi",
            UnitMetricFormat::Deca => "deca",
            UnitMetricFormat::Deci => "deci",
            UnitMetricFormat::Exa => "exa",
            UnitMetricFormat::Femto => "femto",
            UnitMetricFormat::Giga => "giga",
            UnitMetricFormat::Hecto => "hecto",
            UnitMetricFormat::Kilo => "kilo",
            UnitMetricFormat::Mega => "mega",
            UnitMetricFormat::MetricUnit => "metric_unit",
            UnitMetricFormat::Micro => "micro",
            UnitMetricFormat::Milli => "milli",
            UnitMetricFormat::Nano => "nano",
            UnitMetricFormat::Peta => "peta",
            UnitMetricFormat::Pico => "pico",
            UnitMetricFormat::Tera => "tera",
            UnitMetricFormat::Noop => "",
            UnitMetricFormat::FallthroughString => "*",
        }
        .fmt(f)
    }
}

impl Default for UnitMetricFormat {
    fn default() -> UnitMetricFormat {
        UnitMetricFormat::Atto
    }
}
impl std::str::FromStr for UnitMetricFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "atto" {
            return Ok(UnitMetricFormat::Atto);
        }
        if s == "centi" {
            return Ok(UnitMetricFormat::Centi);
        }
        if s == "deca" {
            return Ok(UnitMetricFormat::Deca);
        }
        if s == "deci" {
            return Ok(UnitMetricFormat::Deci);
        }
        if s == "exa" {
            return Ok(UnitMetricFormat::Exa);
        }
        if s == "femto" {
            return Ok(UnitMetricFormat::Femto);
        }
        if s == "giga" {
            return Ok(UnitMetricFormat::Giga);
        }
        if s == "hecto" {
            return Ok(UnitMetricFormat::Hecto);
        }
        if s == "kilo" {
            return Ok(UnitMetricFormat::Kilo);
        }
        if s == "mega" {
            return Ok(UnitMetricFormat::Mega);
        }
        if s == "metric_unit" {
            return Ok(UnitMetricFormat::MetricUnit);
        }
        if s == "micro" {
            return Ok(UnitMetricFormat::Micro);
        }
        if s == "milli" {
            return Ok(UnitMetricFormat::Milli);
        }
        if s == "nano" {
            return Ok(UnitMetricFormat::Nano);
        }
        if s == "peta" {
            return Ok(UnitMetricFormat::Peta);
        }
        if s == "pico" {
            return Ok(UnitMetricFormat::Pico);
        }
        if s == "tera" {
            return Ok(UnitMetricFormat::Tera);
        }
        anyhow::bail!("invalid string: {}", s);
    }
}
impl UnitMetricFormat {
    pub fn is_noop(&self) -> bool {
        matches!(self, UnitMetricFormat::Noop)
    }
}

/// A unit conversion.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct UnitConversion {
    /**
    * A uuid.
    *  
    *  A Version 4 UUID is a universally unique identifier that is generated using random numbers.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * The time and date the unit conversion was completed.
    */
    #[serde()]
    pub completed_at: crate::utils::DisplayOptionDateTime,

    /**
    * The time and date the unit conversion was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * The error the function returned, if any.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,

    /**
    * A unit conversion.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub input: f64,

    /**
    * The resulting value.
    */
    #[serde(
        default,
        skip_serializing_if = "crate::utils::zero_f64",
        deserialize_with = "crate::utils::deserialize_null_f64::deserialize"
    )]
    pub output: f64,

    /**
    * The valid types of metric unit formats.
    */
    #[serde(default, skip_serializing_if = "UnitMetricFormat::is_noop")]
    pub output_format: UnitMetricFormat,

    /**
    * The valid types of metric unit formats.
    */
    #[serde(default, skip_serializing_if = "UnitMetricFormat::is_noop")]
    pub src_format: UnitMetricFormat,

    /**
    * The time and date the unit conversion was started.
    */
    #[serde()]
    pub started_at: crate::utils::DisplayOptionDateTime,

    /**
    * The status of an async API call.
    */
    #[serde(default, skip_serializing_if = "ApiCallStatus::is_noop")]
    pub status: ApiCallStatus,

    /**
    * The time and date the unit conversion was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,

    /**
    * A unit conversion.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_id: String,
}

/// The user-modifiable parts of a User.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct UpdateUser {
    /**
    * The user-modifiable parts of a User.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,

    /**
    * The user-modifiable parts of a User.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discord: String,

    /**
    * The user-modifiable parts of a User.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,

    /**
    * The user-modifiable parts of a User.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub github: String,

    /**
    * The user-modifiable parts of a User.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,

    /**
    * The user-modifiable parts of a User.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,
}

/// A user.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct User {
    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,

    /**
    * The date and time the user was created.
    */
    #[serde()]
    pub created_at: crate::utils::DisplayOptionDateTime,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discord: String,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,

    /**
    * The date and time the email address was verified.
    */
    #[serde()]
    pub email_verified: crate::utils::DisplayOptionDateTime,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub first_name: String,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub github: String,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image: String,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_name: String,

    /**
    * A user.
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub phone: String,

    /**
    * The date and time the user was last updated.
    */
    #[serde()]
    pub updated_at: crate::utils::DisplayOptionDateTime,
}

/// A single page of results
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema, Default, Tabled)]
pub struct UserResultsPage {
    /**
    * list of items on this page of results
    */
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::utils::deserialize_null_vector::deserialize"
    )]
    #[tabled(skip)]
    pub items: Vec<User>,

    /**
    * token used to fetch the next page of results (if any)
    */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_page: String,
}

pub type Duration = i64;
pub type PhoneNumber = String;
pub type StatusCode = i32;
/// A uuid.
///
/// A Version 4 UUID is a universally unique identifier that is generated using random numbers.
pub type Uuid = String;
