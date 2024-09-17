use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use sea_orm::{EntityTrait};

use entity::todo_lists::Entity as TodoLists;
mod config;
mod repositories;

use config::database_config;


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
    let db = database_config::connect().await;

    match db {
        Ok(db_connection) => {
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
    use sea_orm::{
        entity::prelude::*, entity::*,
        DatabaseBackend, MockDatabase, Transaction,
    };

    #[async_std::test]
    async fn test_find_cake() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db: &DatabaseConnection = &MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // First query result
                vec![bakery::Model {
                    id: 1,
                    name: "Happy Bakery".to_owned(),
                    profit_margin: 0.0,
                }],
            ])
            .into_connection();

        // Find a cake from MockDatabase
        // Return the first query result
        assert_eq!(
            cake::Entity::find().one(&db).await?,
            Some(cake::Model {
                id: 1,
                name: "New York Cheese".to_owned(),
            })
        );

        // Checking transaction log
        assert_eq!(
            db.into_transaction_log(),
            [
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "cake"."id", "cake"."name" FROM "cake" LIMIT $1"#,
                    [1u64.into()]
                )
            ]
        );

        Ok(())
    }

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