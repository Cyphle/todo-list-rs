use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue::Set;
use entity::prelude::TodoLists;
use serde::Deserialize;
use entity::todo_lists;
use crate::db_connection::establish_connection;

mod db_connection;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/todos")]
async fn get_todos(db: web::Data<DatabaseConnection>) -> impl Responder {
    let todos = TodoLists::find().all(db.get_ref()).await.expect("Failed to fetch todos");
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
async fn get_todo_by_id(path: web::Path<i32>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let id = path.into_inner();
    let todo = TodoLists::find_by_id(id).one(db.get_ref()).await;

    match todo {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize, Debug)]
pub struct TodoView {
    pub title: String
}

#[derive(Clone, Debug)]
pub struct TodoList {
    pub title: String,
}

pub struct TodoListRepository {
    db: DatabaseConnection,
}

impl TodoListRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        TodoListRepository { db }
    }

    pub async fn save(&self, todo: TodoList) -> Result<(), sea_orm::DbErr> {
        let new_todo = todo_lists::ActiveModel {
            id: Default::default(),
            title: Set(todo.title),
        };

        if let Err(err) = new_todo.insert(&self.db).await {
            eprintln!("Error while saving todo : {:?}", err);
            return Err(err);
        }

        Ok(())
    }
}

impl TodoList {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

pub struct TodoListService {
    repository: TodoListRepository,
}

impl TodoListService {
    pub fn new(repository: TodoListRepository) -> Self {
        TodoListService { repository }
    }

    async fn create_todo(&self, dto: TodoView) -> Result<(), ()> {
        let new_todo = TodoList::new(dto.title);
        self.repository.save(new_todo).await.expect("TODO: panic message");
        Ok(())
    }
}

#[post("/todos")]
async fn create_todo(service: web::Data<TodoListService>, payload: web::Json<TodoView>) -> impl Responder {
    service.create_todo(payload.into_inner()).await.expect("TODO: panic message");
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // SEA ORM
    let connection: DatabaseConnection = match establish_connection().await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error while connecting to the database : {:?}", err);
            std::process::exit(1);
        }
    };

    let todo_repository = TodoListRepository::new(connection);
    let todo_service = web::Data::new(TodoListService::new(todo_repository));

    // ACTIX
    HttpServer::new(move || {
        App::new()
            .app_data(todo_service.clone())// Use of `.clone()` to duplicate the connection for each Actix thread
            .service(hello)
            .service(echo)
            .service(get_todos)
            .service(get_todo_by_id)
            .service(create_todo)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;

    mod actix_tests {
        use actix_web::{App, test};
        use actix_web::http::header::ContentType;

        use crate::{echo, hello};

        #[actix_web::test]
        async fn test_hello_get() {
            let app = test::init_service(App::new().service(hello)).await;
            let req = test::TestRequest::default()
                .insert_header(ContentType::plaintext())
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }

        #[actix_web::test]
        async fn test_echo_post() {
            let app = test::init_service(App::new().service(echo)).await;
            let req = test::TestRequest::post().uri("/echo").to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }

        #[actix_web::test]
        async fn test_echo_post_error() {
            let app = test::init_service(App::new().service(echo)).await;
            let req = test::TestRequest::post().uri("/").to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_client_error());
        }
    }

    // TODO tests SeaORM
}