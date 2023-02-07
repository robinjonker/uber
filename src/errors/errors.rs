use std::string::ToString;

use thiserror::Error;

// pub(crate) type UberResult<T> = Result<T, UberError>;

pub trait UnwrapRequestField<T> {
    fn unwrap_request_field(self, field_name: &str) -> Result<T, UberError>;
}

impl<T> UnwrapRequestField<T> for Option<T> {
    fn unwrap_request_field(self, field_name: &str) -> Result<T, UberError> {
        match self {
            Some(t) => Ok(t),
            None => Err(UberError::BadInput(format!(
                "Request missing field '{}'",
                field_name
            ))),
        }
    }
}

/// Error enum for all cases of internal/external errors occurring during client execution
#[derive(Error, Debug)]
pub enum UberError {
    // Http-like errors
    #[error("Bad input - {0}")]
    BadInput(String),
    #[error("Unauthorized - {0}")]
    Unauthorized(String),
    #[error("Forbidden - {0}")]
    Forbidden(String),
    #[error("Invalid state - {0}")]
    InvalidState(String),
    #[error("Not found - {0}")]
    NotFound(String),
    #[error("Internal server error - {0}")]
    InternalServerError(String),
    #[error("Exists - {0}")]
    Exists(String),
    #[error("Not implemented - {0}")]
    NotImplemented(String),
    #[error("Timed out - {0}")]
    Timeout(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidRequest { expected: String, found: String },

    // Errors converted from others
    #[error("Json error - {0:?}")]
    JsonError(#[from] serde_json::Error),
    #[error("Urlencoded error - {0:?}")]
    UrlencodedError(#[from] serde_urlencoded::ser::Error),
    #[error("ReqwestError - {0:?}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("HeaderValue conversion error - {0:?}")]
    HeaderValueError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("ParseError - {0:?}")]
    ParseError(#[from] chrono::ParseError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub const UNAUTHORIZED: &str = "unauthorized";
pub const FORBIDDEN: &str = "forbidden";
pub const INVALID_INPUT: &str = "invalid-input";
pub const INVALID_STATE: &str = "invalid-state";
pub const NOT_FOUND: &str = "not-found";
pub const INTERNAL_SERVER_ERROR: &str = "internal-server-error";
pub const EXISTS: &str = "exists";
pub const NOT_IMPLEMENTED: &str = "not-implemented";

impl UberError {
    pub fn from_code<T: Into<String>>(code: &str, message: T) -> Self {
        match code {
            UNAUTHORIZED => Self::Unauthorized(message.into()),
            FORBIDDEN => Self::Forbidden(message.into()),
            INVALID_INPUT => Self::BadInput(message.into()),
            NOT_FOUND => Self::NotFound(message.into()),
            INVALID_STATE => Self::InvalidState(message.into()),
            INTERNAL_SERVER_ERROR => Self::InternalServerError(message.into()),
            EXISTS => Self::Exists(message.into()),
            NOT_IMPLEMENTED => Self::NotImplemented(message.into()),
            _ => Self::InternalServerError(message.into()),
        }
    }

    pub fn get_code(&self) -> String {
        match self {
            Self::Unauthorized(_) => UNAUTHORIZED.to_string(),
            Self::Forbidden(_) => FORBIDDEN.to_string(),
            Self::BadInput(_) => INVALID_INPUT.to_string(),
            Self::InvalidState(_) => INVALID_STATE.to_string(),
            Self::NotFound(_) => NOT_FOUND.to_string(),
            Self::InternalServerError(_) => INTERNAL_SERVER_ERROR.to_string(),
            Self::Exists(_) => EXISTS.to_string(),
            Self::NotImplemented(_) => NOT_IMPLEMENTED.to_string(),
            _ => INTERNAL_SERVER_ERROR.to_string(),
        }
    }
}

