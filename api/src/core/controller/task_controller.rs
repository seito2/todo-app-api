use actix_web::{delete, post, web, HttpRequest, HttpResponse};

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

#[delete("/{task_id}")]
async fn delete(req: HttpRequest) -> actix_web::Result<HttpResponse, TaskApiError> {
    let task_id: i32 = req.match_info().query("task_id").parse().unwrap();

    let res = task_service::delete_task(task_id);

    return match res {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => Err(e),
    };
}

pub fn get_controller() -> actix_web::Scope {
    return actix_web::web::scope("/task")
        .service(create)
        .service(delete);
}
