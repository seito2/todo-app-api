use crate::schema::tasks;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct TaskEntity {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct TaskEntityInsertable {
    pub title: String,
}
