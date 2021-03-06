use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use actix::MailboxError;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use derive_more::{Display};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::convert::From;
use std::num::ParseIntError;
use std::sync::{MutexGuard, PoisonError};
use awc::error::SendRequestError;
use lettre::address::AddressError;
use redis::RedisError;
use validator::ValidationErrors;

#[derive(Debug, Display)]
pub enum Error {
    // 400
    #[display(fmt = "BadRequest ")]
    BadRequest(JsonValue),

    // 401
    #[display(fmt = "Unauthorized")]
    Unauthorized,

    // 403
    #[display(fmt = "Forbidden")]
    Forbidden,

    // 404
    #[display(fmt = "NotFound")]
    NotFound(JsonValue),

    // 422
    #[display(fmt = "UnprocessableEntity")]
    UnprocessableEntity(JsonValue),

    // 500
    #[display(fmt = "InternalServerError")]
    InternalServerError,
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::BadRequest(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            Error::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            Error::Forbidden => HttpResponse::Forbidden().json("Forbidden"),
            Error::NotFound(ref message) => HttpResponse::NotFound().json(message),
            Error::UnprocessableEntity(ref message) => {
                HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
            }
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<SendRequestError> for Error {
    fn from(_error: SendRequestError) -> Self {
        Error::InternalServerError
    }
}

impl From<awc::error::JsonPayloadError> for Error {
    fn from(_error: awc::error::JsonPayloadError) -> Self {
        Error::BadRequest( json!("BadRequest"))
    }
}

impl From<MailboxError> for Error {
    fn from(_error: MailboxError) -> Self {
        Error::InternalServerError
    }
}

impl From<RedisError> for Error {
    fn from(_error: RedisError) -> Self {
        Error::InternalServerError
    }
}

impl From<ParseIntError> for Error {
    fn from(_error: ParseIntError) -> Self {
        Error::InternalServerError
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessableEntity(json!({ "error": message }));
                }
                Error::InternalServerError
            }
            DieselError::NotFound => {
                Error::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => Error::InternalServerError,
        }
    }
}

impl From<PoolError> for Error {
    fn from(_error: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<PoisonError<MutexGuard<'_, redis::Connection>>> for Error {
    fn from(_error: PoisonError<MutexGuard<'_, redis::Connection>>) -> Self {
        Error::InternalServerError
    }
}

impl From<AddressError> for Error {
    fn from(_error: AddressError) -> Self {
        Error::BadRequest(json!("??????????????????"))
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        // transforms errors into objects that err_map can take
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors
                .iter()
                .map(|error| {
                    // dbg!(error) // <- Uncomment this if you want to see what error looks like
                    json!(error.message)
                })
                .collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        Error::UnprocessableEntity(json!({
            "errors": err_map,
        }))
    }
}
