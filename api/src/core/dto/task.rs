use serde::Deserialize;

use crate::core::entity::task::TaskEntityInsertable;

#[derive(Deserialize)]
pub struct NewTaskDto {
    pub title: String,
}

impl NewTaskDto {
    pub fn to_insertable(&self) -> TaskEntityInsertable {
        TaskEntityInsertable {
            title: self.title.clone(),
        }
    }
}
