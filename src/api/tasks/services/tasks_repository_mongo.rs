use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use rocket::futures::{TryFutureExt, TryStreamExt};

use crate::api::app::mongo_component::ClientMongoComponent;
use crate::api::tasks::entities::task_dbo::TaskDbo;
use crate::core::tasks::entities::task::Task;
use crate::core::tasks::services::tasks_repository::TasksRepository;
use crate::models::tasks::errors::custom::CustomError;

pub struct TasksRepositoryMongo {
    pub collection: Collection<Document>,
}

impl ClientMongoComponent for TasksRepositoryMongo {}

#[async_trait]
impl TasksRepository for TasksRepositoryMongo {
    async fn insert_task(&self, task: Task) -> Result<(), CustomError> {
        if !self.exist(&task).await {
            self
                .insert_task_without_check(&task)
                .await
                .map(|_| ())
        } else {
            Err(CustomError::new("la tache existe déjà en base"))
        }
    }

    async fn delete_task(&self, id: &str) -> Result<(), CustomError> {
        if self.fetch_one_by_id(id).await.is_ok() {
            self.delete_task_without_check(id).await
        } else {
            Err(CustomError::new("la tache n'éxiste pas"))
        }
    }

    async fn fetch_many(&self) -> Vec<Task> {
        match self.find_all().await {
            Ok(tasks) => tasks,
            _ => {
                eprintln!("une erreur est survenue lors de la lecture");
                vec![]
            }
        }
    }

    async fn fetch_one_by_id(&self, id: &str) -> Result<Task, CustomError> {
        self.collection
            .find_one(
                Some(
                    doc! {
                        "id": id
                    }
                ),
                None
            )
            .await
            .map(|dbo_doc_opt|{
                dbo_doc_opt
                    .map(|dbo_doc| {
                        let task_dbo: TaskDbo = dbo_doc.into();
                        let task: Task = task_dbo.into();
                        Ok(task)
                    })
                    .unwrap_or(Err(CustomError::new("impossible de recupere la tache")))
            })
            .unwrap_or_else(|err| Err(CustomError::new(format!("{}", err.to_string()).as_str())))
    }

    async fn change_state(&self, id: &str, state: String) -> Result<(), CustomError> {
        let filter = doc! {
            "id": id
        };
        let update = doc! {
            "$set": {
                "state": state.as_str()
            }
        };
        self.collection
            .update_one(filter, update, None)
            .await
            .map(|_| ())
            .map_err(|_| CustomError::new("erreur lors de l'update"))
    }
}

impl TasksRepositoryMongo {

    async fn find_all(&self) -> Result<Vec<Task>, Error> {
        Ok(
            self.collection
                .find(None, None)
                .await?
                .try_collect::<Vec<Document>>()
                .await?
                .into_iter()
                .map(|doc| doc.into())
                .map(|dbo: TaskDbo| dbo.into())
                .collect::<Vec<Task>>()
                .into()
        )
    }

    pub async fn new() -> Result<Self, mongodb::error::Error> {
        Ok(
            Self {
                collection: {
                    Self::collection_tasks().await?
                }
            }
        )
    }

    async fn exist(&self, task: &Task) -> bool {
        self.fetch_one_by_id(task.id.as_str())
            .await
            .is_ok()
    }

    async fn insert_task_without_check(&self, task: &Task) -> Result<InsertOneResult, CustomError> {
        let document: Document = task.clone().into();
        self.collection
            .insert_one(document, None)
            .map_err(|_| CustomError::new("erreur lors de l'insertion en base"))
            .await
    }

    async fn delete_task_without_check(&self, id: &str) -> Result<(), CustomError> {
        let document: Document = doc! {"id": id};
        self.collection
            .delete_one(document, None)
            .await
            .map(|_| ())
            .map_err(|_| CustomError::new("erreur lors de la suppression"))
    }

}
