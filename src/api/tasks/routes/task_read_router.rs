use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use crate::api::tasks::services::tasks_repository_mongo::TasksRepositoryMongo;
use crate::core::tasks::services::tasks_repository::TasksRepository;

use crate::models::tasks::views::json_data_response::JsonDataResponse;
use crate::models::tasks::views::task_view::TaskView;

#[get("/hello-world")]
pub async fn hello() -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    Ok(Json(JsonDataResponse::new("hello world")))
}

#[get("/tasks")]
pub async fn tasks(
    tasks_repository: &State<TasksRepositoryMongo>
) -> Result<Json<Vec<TaskView>>, status::Custom<Json<JsonDataResponse>>> {
    Ok(
        Json(
            tasks_repository
                .fetch_many()
                .await
                .into_iter()
                .map(|task| {
                    task.into()
                })
                .collect::<Vec<_>>()
        )
    )
}