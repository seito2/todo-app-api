use serde::Deserialize;

use crate::core::entity::task::TaskEntityInsertable;

#[derive(Deserialize)]
pub struct NewTaskDto {
    pub title: String,
}

impl NewTaskDto {
    pub fn toInsertable(&self) -> TaskEntityInsertable {
        TaskEntityInsertable {
            title: self.title.clone(),
        }
    }
}
