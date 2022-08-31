//! Custom errors (ApiError)

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use log::*;
use uuid::parser::ParseError;

#[derive(Debug, Display, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    ParseError(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert DBErrors to ApiErrors
impl From<DBError> for ApiError {
    fn from(error: DBError) -> ApiError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        error!("Database Error {:?}", error);
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ApiError::BadRequest(message);
                }
                ApiError::InternalServerError("Unknown database error".into())
            }
            _ => ApiError::InternalServerError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ApiErrors
impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        error!("Pool Error Error {:?}", error);
        ApiError::PoolError(error.to_string())
    }
}

/// Convert ParseErrors to ApiErrors
impl From<ParseError> for ApiError {
    fn from(error: ParseError) -> ApiError {
        error!("Parse Error {:?}", error);
        ApiError::ParseError(error.to_string())
    }
}

// TODO: Add additional error mappings
/// Converts custom ApiError into Axum acceptable response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(error) => (StatusCode::BAD_REQUEST, error).into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
        }
    }
}
