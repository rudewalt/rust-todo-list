use crate::entity::Task;
use crate::model::UpdateTask;
use anyhow::Result;
use bson::Document;
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{Client, Collection};
use std::str::FromStr;

pub struct TasksRepo {
    tasks: Collection<Task>,
}

impl TasksRepo {
    pub fn init(client_options: ClientOptions) -> Result<Self> {
        let client = Client::with_options(client_options)?;
        let database = client.database("TasksDB");

        Ok(TasksRepo {
            tasks: database.collection("tasks"),
        })
    }

    fn build_by_id_query(id: &str) -> Result<Document> {
        let object_id = bson::oid::ObjectId::from_str(id)?;
        Ok(doc! { "_id": object_id })
    }

    pub async fn get_all(&self) -> Result<Vec<Task>> {
        let cursor = self.tasks.find(None, None).await?;
        let result = cursor.try_collect().await?;
        Ok(result)
    }

    pub async fn get_one(&self, id: &str) -> Result<Option<Task>> {
        let filter = Self::build_by_id_query(id)?;
        self.tasks
            .find_one(filter, None)
            .await
            .map_err(|e| e.into())
    }

    pub async fn create_task(&self, task: Task) -> Result<String> {
        self.tasks
            .insert_one(task, None)
            .await
            .map(|r| {
                let object_id = r
                    .inserted_id
                    .as_object_id()
                    .expect("Create task should return ObjectId");
                object_id.to_hex()
            })
            .map_err(|e| e.into())
    }

    pub async fn delete(&self, id: &str) -> Result<DeleteResult> {
        let query = Self::build_by_id_query(id)?;
        self.tasks
            .delete_one(query, None)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update(&self, id: &str, update_request: &UpdateTask) -> Result<UpdateResult> {
        let query = Self::build_by_id_query(id)?;
        let update = {
            let mut set_expr = doc! {"updated_at": Some(Utc::now())};
            if let Some(is_complete) = &update_request.is_complete {
                set_expr.insert("is_complete", is_complete);
            }
            if let Some(description) = &update_request.description {
                set_expr.insert("description", description);
            }
            doc! {"$set": set_expr }
        };

        self.tasks
            .update_one(query, update, None)
            .await
            .map_err(|e| e.into())
    }
}
