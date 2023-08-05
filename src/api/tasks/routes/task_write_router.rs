use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

use crate::api::tasks::services::tasks_repository_mongo::TasksRepositoryMongo;
use crate::core::tasks::entities::task::Task;
use crate::core::tasks::services::tasks_repository::TasksRepository;
use crate::models::tasks::commands::change_state_command::ChangeStateCommand;
use crate::models::tasks::commands::create_task_command::CreateTaskCommand;
use crate::models::tasks::views::json_data_response::JsonDataResponse;

#[post("/tasks/commands/create", data="<create_command>")]
pub async fn create(
    tasks_repository: &State<TasksRepositoryMongo>,
    create_command: Json<CreateTaskCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    let cmd = create_command.0;
    tasks_repository
        .insert_task(
            Task {
                id: Uuid::new_v4().to_string(),
                titre: cmd.titre,
                description: cmd.description,
                tags: cmd.tags,
                state: "TODO".to_string()
            }
        )
        .await
        .map(|_| Json(JsonDataResponse::new("inserted")))
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse::new(err.message.as_str())
                )
            )
        })
}

#[put("/tasks/commands/change-state/<id>", data="<change_state_command>")]
pub async fn change_state_task(
    tasks_repository: &State<TasksRepositoryMongo>,
    id: &str,
    change_state_command: Json<ChangeStateCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    tasks_repository
        .change_state(
            id, change_state_command.0.state
        )
        .await
        .map(|_| Json(JsonDataResponse::new("OK")))
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse::new(err.message.as_str())
                )
            )
        })
}
