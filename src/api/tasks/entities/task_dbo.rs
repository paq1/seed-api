use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};
use crate::core::tasks::entities::task::Task;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskDbo {
    pub _id: ObjectId,
    pub id: String,
    pub url: String,
    pub http_method: String,
    pub repetition_seconds: Option<u32>,
    pub state: String
}

impl From<TaskDbo> for Task {
    fn from(value: TaskDbo) -> Self {
        Self {
            id: value.id,
            url: value.url,
            http_method: value.http_method,
            repetition_seconds: value.repetition_seconds,
            state: value.state
        }
    }
}

impl From<Document> for TaskDbo {
    fn from(value: Document) -> Self {
        mongodb::bson::from_bson(Bson::Document(value))
            .unwrap()
    }
}