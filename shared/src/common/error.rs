use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("{0}")]
    InvalidState(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    InfrastructureError(String),
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::NotFound(_) => StatusCode::NOT_FOUND,
            APIError::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            APIError::InvalidState(_) => StatusCode::UNPROCESSABLE_ENTITY,
            APIError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            APIError::InfrastructureError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
