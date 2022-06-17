use crate::auth::ApiKey;
use crate::entity::Task;
use crate::model::{CreateTask, CreateTaskResponse, TaskResponse, UpdateTask};
use crate::storage::TasksRepo;
use crate::utils::handle_error;
use chrono::Utc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, rust todo list!"
}

pub mod task {
    use super::*;

    #[post("/", format = "json", data = "<request>")]
    pub async fn create(
        _api_key: ApiKey<'_>,
        tasks_repo: &State<TasksRepo>,
        request: Json<CreateTask>,
    ) -> Result<Json<CreateTaskResponse>, Status> {
        let task = Task {
            id: None,
            description: request.description.clone(),
            is_complete: false,
            created_at: Utc::now(),
            updated_at: None,
        };

        tasks_repo
            .create_task(task.clone())
            .await
            .map(|id| Json(CreateTaskResponse { id }))
            .map_err(handle_error)
    }

    #[get("/", format = "json")]
    pub async fn get_all(tasks_repo: &State<TasksRepo>) -> Result<Json<Vec<TaskResponse>>, Status> {
        tasks_repo
            .get_all()
            .await
            .map(|r| {
                Json(
                    r.into_iter()
                        .map(|t| TaskResponse::from(t))
                        .collect::<Vec<_>>(),
                )
            })
            .map_err(handle_error)
    }

    #[get("/<id>", format = "json")]
    pub async fn get(
        tasks_repo: &State<TasksRepo>,
        id: &str,
    ) -> Result<Json<TaskResponse>, Status> {
        match tasks_repo.get_one(id).await {
            Ok(maybe_task) => match maybe_task {
                Some(task) => Ok(Json(task.into())),
                _ => Err(Status::NotFound),
            },
            Err(e) => Err(handle_error(e)),
        }
    }

    #[put("/<id>", format = "json", data = "<update>")]
    pub async fn update(
        _api_key: ApiKey<'_>,
        tasks_repo: &State<TasksRepo>,
        id: &str,
        update: Json<UpdateTask>,
    ) -> Status {
        match tasks_repo.update(id, &update.0).await {
            Ok(_) => Status::NoContent,
            Err(e) => handle_error(e),
        }
    }

    #[delete("/<id>", format = "json")]
    pub async fn delete(_api_key: ApiKey<'_>, tasks_repo: &State<TasksRepo>, id: &str) -> Status {
        match tasks_repo.delete(id).await {
            Ok(_) => Status::NoContent,
            Err(e) => handle_error(e),
        }
    }
}
