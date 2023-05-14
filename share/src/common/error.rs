use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum APIError {
    #[display(fmt = "not found")]
    NotFound,

    #[display(fmt = "invalid argument")]
    InvalidArgument,

    #[display(fmt = "invalid state")]
    InvalidState,

    #[display(fmt = "unique constraint violation")]
    UniqueConstraint,

    #[display(fmt = "unauthorized")]
    Unauthorized,

    #[display(fmt = "infrastructure error")]
    InfrastructureError,

    #[display(fmt = "authentication error")]
    AuthenticationError,

    #[display(fmt = "unexpected behavior")]
    LogicError,
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::AuthenticationError => StatusCode::UNAUTHORIZED,
            APIError::InvalidArgument => StatusCode::BAD_REQUEST,
            APIError::InfrastructureError => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::InvalidState => StatusCode::UNPROCESSABLE_ENTITY,
            APIError::LogicError => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::NotFound => StatusCode::NOT_FOUND,
            APIError::Unauthorized => StatusCode::UNAUTHORIZED,
            APIError::UniqueConstraint => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
