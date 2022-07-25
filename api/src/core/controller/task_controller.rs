use actix_web::{post, web, HttpResponse};

use crate::core::{
    dto::task::NewTaskDto, modules::errors::task_error::TaskApiError, service::task_service,
};

#[post("/create")]
async fn create(
    new_task_dto: web::Json<NewTaskDto>,
) -> actix_web::Result<HttpResponse, TaskApiError> {
    let res = task_service::create_task(new_task_dto.into_inner());

    return match res {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    };
}

pub fn get_controller() -> actix_web::Scope {
    return actix_web::web::scope("/task").service(create);
}
