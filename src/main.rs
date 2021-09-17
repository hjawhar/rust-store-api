mod config;
mod db;
mod models;
mod routes;

use crate::db::connect_to_mongodb;
use crate::routes::{hello, status};

use actix_web::{App, HttpServer};

use dotenv::dotenv;

pub fn printabc() {
    println!("{}", "abc")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    println!(
        "Starting server at http://{}:/{}/",
        config.server.host, config.server.port
    );

    connect_to_mongodb()
        .await
        .expect("Couldn't connect to MongoDB server");
    HttpServer::new(|| App::new().service(hello).service(status))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
