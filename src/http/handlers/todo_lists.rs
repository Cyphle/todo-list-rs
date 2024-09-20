use crate::repositories;
use actix_web::{get, HttpResponse, Responder};

// TODO test these handlers
// TODO il faut utiliser les states pour injecter la connexion (https://actix.rs/docs/application)
#[get("/todo_lists")]
async fn get_todos() -> impl Responder {
    match repositories::todo_lists_repository::find_all().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// #[get("/todos/{id}")]
// async fn get_todo_by_id(path: web::Path<i32>, service: web::Data<TodoListService>) -> impl Responder {
//     match service.get_todo_by_id(path.into_inner()).await {
//         Ok(Some(todo)) => HttpResponse::Ok().json(todo),
//         Ok(None) => HttpResponse::NotFound().body("Todo not found"),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// #[post("/todos")]
// async fn create_todo(service: web::Data<TodoListService>, payload: web::Json<TodoListView>) -> impl Responder {
//     match service.create_todo(payload.into_inner()).await {
//         Ok(_) => HttpResponse::Created().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }