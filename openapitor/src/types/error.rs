//! Error methods.

/// Error produced by generated client methods.
pub enum Error {
    /// The request did not conform to API requirements.
    InvalidRequest(String),

    #[cfg(feature = "retry")]
    /// A server error either due to the data, or with the connection.
    CommunicationError(reqwest_middleware::Error),

    /// A request error, caused when building the request.
    RequestError(reqwest::Error),

    /// An expected response whose deserialization failed.
    SerdeError {
        /// The error.
        error: format_serde_error::SerdeError,
        /// The response status.
        status: reqwest::StatusCode,
    },

    /// An expected error response.
    InvalidResponsePayload {
        #[cfg(feature = "retry")]
        /// The error.
        error: reqwest_middleware::Error,
        #[cfg(not(feature = "retry"))]
        /// The error.
        error: reqwest::Error,
        /// The full response.
        response: reqwest::Response,
    },

    /// An error from the server.
    Server {
        /// The text from the body.
        body: String,
        /// The response status.
        status: reqwest::StatusCode,
    },

    /// A response not listed in the API description. This may represent a
    /// success or failure response; check `status().is_success()`.
    UnexpectedResponse(reqwest::Response),
}

impl Error {
    /// Returns the status code, if the error was generated from a response.
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
            Error::Server { body: _, status } => Some(*status),
            Error::UnexpectedResponse(r) => Some(r.status()),
        }
    }

    /// Creates a new error from a response status and a serde error.
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

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError {
            error: format_serde_error::SerdeError::new(String::new(), e),
            status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        }
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
            Error::Server { body, status } => {
                write!(f, "Server Error: {} {}", status, body)
            }
            Error::UnexpectedResponse(r) => {
                write!(f, "Unexpected Response: {:?}", r)
            }
        }
    }
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
