use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use sea_orm::{ActiveModelTrait, Database, EntityTrait};
use sea_orm::ActiveValue::Set;

use entity::todo_lists;
use entity::todo_lists::Entity as TodoLists;
use migration::MigratorTrait;
mod config;
use crate::config::database_config;
mod db_connection;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database_config::connect();

    match db {
        Ok(db_connection) => {
            let temp_list = todo_lists::ActiveModel {
                title: Set("My list".to_owned()),
                ..Default::default() // all other attributes are `NotSet`
            };
            let my_ilist = temp_list.insert(&db_connection).await;

            // Read
            // Migrator::up(&connection, None).await?; To launch from code see https://www.sea-ql.org/SeaORM/docs/migration/running-migration/
            let todo_list = TodoLists::find_by_id(1).one(&db_connection).await;
            match todo_list {
                Ok(result) => {
                    match result {
                        None => {
                            println!("No result found")
                        }
                        Some(res) => {
                            println!("{:?}", res);
                        }
                    }
                }
                Err(error) => {
                    panic!("{:?}", error);
                }
            }
        }
        Err(_) => {}
    }
    println!("Au revoir");

    // ACTIX
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
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