use crate::auth::ApiKeyValidator;
use crate::storage::TasksRepo;
use mongodb::options::ClientOptions;

#[macro_use]
extern crate rocket;

mod api;
mod auth;
mod entity;
mod model;
mod storage;
mod utils;

#[launch]
async fn rocket() -> _ {
    let tasks_repo = {
        let db_path = option_env!("DB_PATH").expect("DB_PATH evn not provided");
        let client_options = ClientOptions::parse(db_path).await.unwrap();
        TasksRepo::init(client_options).expect("Tasks repository not initialized")
    };

    let api_key_validator =
        ApiKeyValidator::from_string(option_env!("API_KEYS").expect("API_KEYS env not provided"));

    rocket::build()
        .manage(tasks_repo)
        .manage(api_key_validator)
        .mount("/", routes![api::index])
        .mount(
            "/task",
            routes![
                api::task::create,
                api::task::get_all,
                api::task::get,
                api::task::delete,
                api::task::update
            ],
        )
}
