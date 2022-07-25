use crate::core::entity::task::TaskEntity;
use crate::diesel::RunQueryDsl;
use crate::{core::entity::task::TaskEntityInsertable, schema::tasks};
use diesel::{PgConnection, QueryDsl};

pub struct TaskRepository {
    pub(crate) connection: PgConnection,
}

impl TaskRepository {
    pub fn create(
        &self,
        task_request: TaskEntityInsertable,
    ) -> Result<TaskEntity, diesel::result::Error> {
        diesel::insert_into(tasks::table)
            .values(task_request)
            .get_result(&self.connection)
    }

    pub fn remove(&self, task_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(tasks::table.find(task_id)).execute(&self.connection)
    }

    pub fn get_all(&self) -> Result<Vec<TaskEntity>, diesel::result::Error> {
        tasks::dsl::tasks.load(&self.connection)
    }
}
