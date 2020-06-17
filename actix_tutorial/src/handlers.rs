use crate::models::Status;
use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Pool, Client};
use crate::db;

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status{ status: String::from("Ok")})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder{
    let client: Client = 
        db_pool.get().await.expect("Error connecting to db");

    let results = db::get_todos(&client).await;

    match results {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
    
}