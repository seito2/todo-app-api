use serde::Serialize;

pub mod task_error;

#[derive(Debug, Serialize)]
struct ApiResponseBody {
    pub success: bool,
    pub errcode: u16,
    pub message: String,
}
