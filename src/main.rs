use actix_web::{App, HttpServer, Responder};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use entity::todo_lists::Entity as TodoLists;
mod config;
mod repositories;
mod dto;
mod domain;
mod http;

use config::database_config;
use http::handlers::examples::{echo, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database_config::connect().await;

    // match db {
    //     Ok(db_connection) => {
    //         // Write
    //         /*
    //         let temp_list = todo_lists::ActiveModel {
    //             title: Set("My list".to_owned()),
    //             ..Default::default() // all other attributes are `NotSet`
    //         };
    //         let my_ilist = temp_list.insert(&db_connection).await;
    //          */
    //         // Read
    //         // Migrator::up(&connection, None).await?; To launch from code see https://www.sea-ql.org/SeaORM/docs/migration/running-migration/
    //         let todo_list = TodoLists::find_by_id(1).one(&db_connection).await;
    //         match todo_list {
    //             Ok(result) => {
    //                 match result {
    //                     None => {
    //                         println!("No result found")
    //                     }
    //                     Some(res) => {
    //                         println!("{:?}", res);
    //                     }
    //                 }
    //             }
    //             Err(error) => {
    //                 panic!("{:?}", error);
    //             }
    //         }
    //     }
    //     Err(_) => {
    //         println!("Error connecting to the database");
    //     }
    // }
    // println!("Au revoir");

    config::actix::config(&db.unwrap()).await
}