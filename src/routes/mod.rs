use crate::models::Status;
use crate::models::User;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[get("/users")]
pub async fn get_users(app_data: web::Data<crate::AppState>) -> impl Responder {
    let f = app_data.service_container.user.get().await;
    let result = web::block(move || f).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/users")]
pub async fn add_users(
    app_data: web::Data<crate::AppState>,
    user: web::Json<User>,
) -> impl Responder {
    let f = app_data.service_container.user.create(&user.name).await;
    let result = web::block(move || f).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
