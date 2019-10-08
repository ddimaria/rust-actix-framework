use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError,
    NotFound(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized,
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Allow Actix to automatically convert ApiErrors to external Response Errors
impl ResponseError for ApiError {
    fn render_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            ApiError::InternalServerError => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            ApiError::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json::<ErrorResponse>(errors.to_vec().into())
            }
            ApiError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
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

impl From<DBError> for ApiError {
    fn from(error: DBError) -> ApiError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ApiError::BadRequest(message);
                }
                ApiError::InternalServerError
            }
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        ApiError::PoolError(error.to_string())
    }
}
