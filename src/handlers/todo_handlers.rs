use actix_web::{get, post, web, HttpResponse, Responder};
use crate::services::todo_service::TodoListService;
use crate::models::todo::TodoListView;

#[get("/todos")]
async fn get_todos(service: web::Data<TodoListService>) -> impl Responder {
    match service.get_all_todos().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/todos/{id}")]
async fn get_todo_by_id(path: web::Path<i32>, service: web::Data<TodoListService>) -> impl Responder {
    match service.get_todo_by_id(path.into_inner()).await {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/todos")]
async fn create_todo(service: web::Data<TodoListService>, payload: web::Json<TodoListView>) -> impl Responder {
    match service.create_todo(payload.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}