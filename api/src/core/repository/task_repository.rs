use crate::core::entity::task::TaskEntity;
use crate::diesel::RunQueryDsl;
use crate::{core::entity::task::TaskEntityInsertable, schema};
use diesel::PgConnection;

pub struct TaskRepository {
    pub(crate) connection: PgConnection,
}

impl TaskRepository {
    pub fn create(
        &self,
        task_request: TaskEntityInsertable,
    ) -> Result<TaskEntity, diesel::result::Error> {
        diesel::insert_into(schema::tasks::table)
            .values(task_request)
            .get_result(&self.connection)
    }
}
