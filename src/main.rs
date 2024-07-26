mod db_connection;
mod todo_list;
mod schema;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::db_connection::establish_connection;
use self::todo_list::*;
use diesel::prelude::*;

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
    use self::schema::todo_list::dsl::*;

        let connection = &mut establish_connection();
        let results = todo_list
            .limit(5)
            .select(TodoListEntity::as_select())
            .load(connection)
            .expect("Error loading todo list");

        println!("Displaying {} todo list", results.len());
        for todolist in results {
            println!("{}", todolist.title);
        }

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
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

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