use crate::core::tasks::entities::task::Task;
use crate::models::tasks::errors::custom::CustomError;

#[async_trait]
pub trait TasksRepository {
    async fn insert_task(&self, task: Task) -> Result<(), CustomError>;
    async fn delete_task(&self, id: &str) -> Result<(), CustomError>;
    async fn fetch_many(&self) -> Vec<Task>;
    async fn fetch_one_by_id(&self, id: &str) -> Result<Task, CustomError>;
    async fn change_state(&self, id: &str, state: String) -> Result<(), CustomError>;
}