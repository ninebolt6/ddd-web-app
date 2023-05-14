use super::error::APIError;
use actix_web::HttpResponse;

pub type ResponseResult<T = HttpResponse> = std::result::Result<T, APIError>;
