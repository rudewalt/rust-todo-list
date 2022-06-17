use crate::entity::Task;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateTask {
    pub description: Option<String>,
    pub is_complete: Option<bool>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTask {
    pub description: String,
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    pub id: String,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub description: String,
    pub is_complete: bool,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        TaskResponse {
            id: task.id.map_or("".to_string(), |r| r.to_string()),
            description: task.description,
            created_at: task.created_at,
            updated_at: task.updated_at,
            is_complete: task.is_complete,
        }
    }
}
