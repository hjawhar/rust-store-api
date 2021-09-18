mod config;
mod db;
mod models;
mod routes;
mod services;
use crate::db::connect_to_mongodb;
use crate::routes::{add_users, get_users, hello, status};
use actix_web::{App, HttpServer};
use services::UserService;

use dotenv::dotenv;

pub struct ServiceContainer {
    user: UserService,
}

impl ServiceContainer {
    pub fn new(user: UserService) -> Self {
        ServiceContainer { user }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

// #[tokio::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    println!(
        "Starting server at http://{}:/{}/",
        config.server.host, config.server.port
    );

    let db = connect_to_mongodb("store")
        .await
        .expect("Couldn't connect to MongoDB server");

    HttpServer::new(move || {
        let service_container = ServiceContainer::new(UserService::new(db.clone()));
        App::new()
            .data(AppState { service_container })
            .service(hello)
            .service(status)
            .service(get_users)
            .service(add_users)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await?;

    Ok(())
}
