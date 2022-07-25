use actix_web::{http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use std::fmt::Debug;

use super::ApiResponseBody;

#[derive(Debug, Display, Error)]
pub enum TaskApiError {
    #[display(fmt = "DBError.")]
    DBError,
    #[display(fmt = "not found.")]
    NotFound,
}

impl actix_web::error::ResponseError for TaskApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            TaskApiError::DBError => StatusCode::NOT_ACCEPTABLE,
            TaskApiError::NotFound => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match *self {
            TaskApiError::DBError => {
                HttpResponse::build(self.status_code()).json(ApiResponseBody {
                    success: false,
                    errcode: 406,
                    message: String::from("DBError"),
                })
            }
            TaskApiError::NotFound => {
                HttpResponse::build(self.status_code()).json(ApiResponseBody {
                    success: false,
                    errcode: 404,
                    message: String::from("not found."),
                })
            }
        }
    }
}
