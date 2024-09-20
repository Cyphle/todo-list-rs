use actix_web::Responder;
use sea_orm::EntityTrait;

mod config;
mod repositories;
mod dto;
mod domain;
mod http;

use config::database_config;
use http::handlers::examples::{echo, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database_config::connect().await.unwrap();
    let static_db = Box::leak(Box::new(db));

    config::actix::config(static_db).await
}