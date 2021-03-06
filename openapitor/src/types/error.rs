//! Error methods.

/// Error produced by generated client methods.
pub enum Error {
    /// The request did not conform to API requirements.
    InvalidRequest(String),

    /// A server error either due to the data, or with the connection.
    CommunicationError(reqwest::Error),

    /// An expected response whose deserialization failed.
    SerdeError {
        /// The error.
        error: format_serde_error::SerdeError,
        /// The response status.
        status: reqwest::StatusCode,
    },

    /// An expected error response.
    InvalidResponsePayload {
        /// The error.
        error: reqwest::Error,
        /// The full response.
        response: reqwest::Response,
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
            Error::CommunicationError(e) => e.status(),
            Error::SerdeError { error: _, status } => Some(*status),
            Error::InvalidResponsePayload { error: _, response } => Some(response.status()),
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
