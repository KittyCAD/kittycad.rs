// TODO: these all come from here we should import the package instead when its deps are not as annoying to deal with.
pub const ERROR: &str = r#"
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
"#;
