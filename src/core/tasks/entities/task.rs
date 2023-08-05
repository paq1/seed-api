use mongodb::bson::{doc, Document};
use crate::models::tasks::views::task_view::TaskView;

#[derive(Clone)]
pub struct Task {
    pub id: String,
    pub url: String,
    pub http_method: String,
    pub repetition_seconds: Option<u32>,
    pub state: String
}


impl From<Task> for TaskView {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            url: value.url,
            http_method: value.http_method,
            repetition_seconds: value.repetition_seconds,
            state: value.state
        }
    }
}

impl From<Task> for Document {
    fn from(value: Task) -> Self {
        doc! {
            "id": value.id,
            "url": value.url,
            "http_method": value.http_method,
            "repetition_seconds": value.repetition_seconds,
            "state": value.state
        }
    }
}