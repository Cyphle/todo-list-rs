use actix_web::{App, HttpServer, web};
use sea_orm::DatabaseConnection;
use crate::{
    repositories::todo_repository::TodoListRepository,
    db_connection::establish_connection,
    services::todo_service::TodoListService,
    config::Config
};
use crate::handlers::todo_handlers::{create_todo, get_todo_by_id, get_todos};

mod db_connection;
mod repositories;
mod services;
mod handlers;
mod models;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize configuration
    let config = Config::from_env();

    // Establish a database connection using the configuration
    let connection: DatabaseConnection = establish_connection(&config)
        .await
        .expect("Failed to connect to the database");

    // Initialize repositories and services
    let todo_repository = TodoListRepository::new(connection);
    let todo_service = web::Data::new(TodoListService::new(todo_repository));

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(todo_service.clone())
            .service(get_todos)
            .service(get_todo_by_id)
            .service(create_todo)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
