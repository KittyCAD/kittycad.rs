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
        error: SerdeError,
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
        /// The full response, boxed to keep the error compact.
        response: Box<reqwest::Response>,
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
    UnexpectedResponse {
        /// HTTP status response code from server
        status: reqwest::StatusCode,
        /// URL that caused the error.
        url: String,
        /// HTTP response body from the server, rendered as text.
        body: String,
        /// HTTP headers, boxed to keep the error compact.
        headers: Box<reqwest::header::HeaderMap>,
    },
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
            Error::UnexpectedResponse { status, .. } => Some(*status),
        }
    }

    /// Creates a new error from a response status and a serde error.
    pub fn from_serde_error(e: SerdeError, status: reqwest::StatusCode) -> Self {
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
            error: SerdeError::new(String::new(), e),
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
            Error::UnexpectedResponse {
                headers,
                status,
                body,
                url,
            } => {
                write!(
                    f,
                    "Unexpected Response for {url} (HTTP {}). Headers: {:?}, body: {}",
                    status, headers, body
                )
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

/// A JSON deserialization error formatted with context from the response body.
#[derive(Debug)]
pub struct SerdeError {
    input: String,
    message: String,
    line: Option<usize>,
    column: Option<usize>,
    contextualize: bool,
    context_lines: usize,
    context_characters: usize,
}

impl SerdeError {
    /// Creates an error with three context lines and 30 context characters.
    pub fn new(input: String, error: serde_json::Error) -> Self {
        Self {
            input,
            message: error.to_string(),
            line: Some(error.line()),
            column: Some(error.column()),
            contextualize: true,
            context_lines: 3,
            context_characters: 30,
        }
    }

    /// Enables or disables context lines and shortening of long error lines.
    pub fn set_contextualize(&mut self, contextualize: bool) -> &mut Self {
        self.contextualize = contextualize;
        self
    }

    /// Returns whether contextualization is enabled, which is true by default.
    pub fn get_contextualize(&self) -> bool {
        self.contextualize
    }

    /// Sets the number of lines shown before and after the error line.
    pub fn set_context_lines(&mut self, context_lines: usize) -> &mut Self {
        self.context_lines = context_lines;
        self
    }

    /// Returns the number of context lines, which is three by default.
    pub fn get_context_lines(&self) -> usize {
        self.context_lines
    }

    /// Sets the number of characters shown on each side of a long-line error.
    pub fn set_context_characters(&mut self, context_characters: usize) -> &mut Self {
        self.context_characters = context_characters;
        self
    }

    /// Returns the number of context characters, which is 30 by default.
    pub fn get_context_characters(&self) -> usize {
        self.context_characters
    }
}

impl std::fmt::Display for SerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        use unicode_segmentation::UnicodeSegmentation;

        let error_line = self.line.unwrap_or_default();
        let error_column = self.column.unwrap_or_default();
        let skip = error_line.saturating_sub(self.context_lines.saturating_add(1));
        let take = self.context_lines.saturating_mul(2).saturating_add(1);
        let lines: Vec<_> = self
            .input
            .lines()
            .enumerate()
            .skip(skip)
            .take(take)
            .collect();
        if lines.is_empty() {
            return writeln!(f, "{}", self.message.red().bold());
        }

        let indentation = lines
            .iter()
            .map(|(_, line)| line.chars().take_while(|c| c.is_whitespace()).count())
            .min()
            .unwrap_or_default();
        let separator = " | ".blue().bold();
        let width = error_line.to_string().len();
        writeln!(f)?;

        for (index, line) in lines {
            let text = line
                .chars()
                .skip(indentation)
                .collect::<String>()
                .replace('\t', " ");
            let line_number = index + 1;
            if line_number != error_line {
                if self.contextualize {
                    writeln!(f, " {:>width$}{}{}", "", separator, text.yellow())?;
                }
                continue;
            }

            let length = self.context_characters.saturating_mul(2).saturating_add(1);
            let shorten = self.contextualize && text.len() > length;
            let start = if shorten {
                error_column.saturating_sub(self.context_characters.saturating_add(1))
            } else {
                0
            };
            let before = start != 0;
            let after = shorten && start.saturating_add(length) < text.graphemes(true).count();
            let text = if shorten {
                text.graphemes(true)
                    .skip(start)
                    .take(length)
                    .collect::<String>()
            } else {
                text
            };

            write!(f, " {}{}", line_number.to_string().blue().bold(), separator)?;
            if before {
                write!(f, "{}", "...".blue().bold())?;
            }
            write!(f, "{}", text)?;
            if after {
                write!(f, "{}", "...".blue().bold())?;
            }
            writeln!(f)?;

            let column = error_column
                .saturating_sub(start)
                .saturating_sub(indentation)
                .saturating_add(if before { 3 } else { 0 });
            let pointer = format!("{:>column$}^ {}", "", self.message);
            writeln!(f, " {:>width$}{}{}", "", separator, pointer.red().bold())?;
        }
        Ok(())
    }
}

impl std::error::Error for SerdeError {}
