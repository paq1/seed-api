use mongodb::bson::{doc, Document};
use crate::models::tasks::views::task_view::TaskView;

#[derive(Clone)]
pub struct Task {
    pub id: String,
    pub titre: String,
    pub description: String,
    pub tags: Vec<String>,
    pub state: String
}


impl From<Task> for TaskView {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            titre: value.titre,
            description: value.description,
            tags: value.tags,
            state: value.state
        }
    }
}

impl From<Task> for Document {
    fn from(value: Task) -> Self {
        doc! {
            "id": value.id,
            "titre": value.titre,
            "description": value.description,
            "tags": value.tags,
            "state": value.state
        }
    }
}