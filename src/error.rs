use actix_web::HttpResponse;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum CustomError {
    NotFound,
    Internal,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "Error -> Not Found"),
            CustomError::Internal => write!(f, "Error -> Server Error"),
        }
    }
}

impl actix_web::ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::NotFound => {
                HttpResponse::NotFound().body(format!("{}", CustomError::NotFound))
            }
            CustomError::Internal => {
                HttpResponse::InternalServerError().body(format!("{}", CustomError::Internal))
            }
        }
    }
}
