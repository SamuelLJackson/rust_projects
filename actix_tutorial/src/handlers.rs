use crate::models::{Status, CreateTodoList, ResultResponse};
use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Pool, Client};
use std::io::ErrorKind::Other;
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

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder{

    let client: Client = 
        db_pool.get().await.expect("Error connecting to db");

    let results = db::get_items(&client, path.0).await;

    match results {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }    
}

pub async fn create_todo_list(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to db");

    let results = db::create_todo_list(&client, json.title.clone()).await;

    match results {
        Ok(todo_list) => HttpResponse::Ok().json(todo_list),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn mark_item_complete(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to db");

    let results = db::mark_item_complete(&client, path.0, path.1).await;

    match results {
        Ok(()) => HttpResponse::Ok().json(ResultResponse {success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse {success: false}),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
