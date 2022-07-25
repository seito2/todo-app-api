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

    let task_insertable = task_request.to_insertable();
    let res = task_repository.create(task_insertable);

    return match res {
        Ok(data) => Ok(data),
        Err(_) => Err(TaskApiError::DBError),
    };
}

pub fn delete_task(task_id: i32) -> Result<usize, TaskApiError> {
    let connection = establish_connection();
    let task_repository = TaskRepository { connection };

    let res = task_repository.remove(task_id);

    return match res {
        Ok(data) => {
            let result = if data == 0 {
                Err(TaskApiError::NotFound)
            } else {
                Ok(data)
            };

            result
        }
        Err(_) => Err(TaskApiError::DBError),
    };
}
