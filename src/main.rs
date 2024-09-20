use actix_web::{web, App, HttpServer, Responder};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use entity::todo_lists::Entity as TodoLists;
mod config;
mod repositories;
mod dto;
mod domain;
mod http;

use config::database_config;
use http::handlers::examples::{echo, hello};
use crate::http::handlers::state::HandlerState;
use crate::http::handlers::todo_lists::{create_todo_list, get_todo_list_by_id, get_todo_lists};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO il faut que ça soit static : https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
    let db = database_config::connect().await.unwrap();

    // config::actix::config(&db.unwrap()).await

    // HttpServer::new(|| {
    //     App::new()
    //         .app_data(web::Data::new(HandlerState {
    //             db_connection: &db
    //         }))
    //         .service(hello)
    //         .service(echo)
    //         .service(get_todo_lists)
    //         .service(get_todo_list_by_id)
    //         .service(create_todo_list)
    // })
    //     .bind(("127.0.0.1", 8080))?
    //     .run()
    //     .await

    Ok(())
}