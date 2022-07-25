use actix_web::Result;

use crate::core::{
    dto::task::NewTaskDto,
    entity::task::TaskEntity,
    modules::{connection::establish_connection, errors::task_error::TaskApiError},
    repository::task_repository::TaskRepository,
};

pub fn create_task(task_request: NewTaskDto) -> Result<TaskEntity, TaskApiError> {
    let connection = establish_connection();
    let task_repository = TaskRepository { connection };

    let task_insertable = task_request.toInsertable();
    let res = task_repository.create(task_insertable);

    return match res {
        Ok(data) => Ok(data),
        Err(_) => Err(TaskApiError::DBError),
    };
}
