mod config;
mod models;

use crate::models::Status;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    println!(
        "Starting server at http://{}:/{}/",
        config.server.host, config.server.port
    );

    HttpServer::new(|| App::new().service(hello).service(status))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
