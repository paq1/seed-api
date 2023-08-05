use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};
use crate::core::tasks::entities::task::Task;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskDbo {
    pub _id: ObjectId,
    pub id: String, // id metier
    pub titre: String,
    pub description: String,
    pub tags: Vec<String>,
    pub state: String
}

impl From<TaskDbo> for Task {
    fn from(value: TaskDbo) -> Self {
        Self {
            id: value.id,
            titre: value.titre,
            description: value.description,
            tags: value.tags,
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