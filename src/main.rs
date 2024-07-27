mod db_connection;
mod schema;
mod todo_list;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::db_connection::establish_connection;
use diesel::prelude::*;
use self::todo_list::*;

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
    use self::schema::todo_lists::dsl::*;

    let connection = &mut establish_connection();

    create_todo_list(connection, "My first todo list");

    let results = todo_lists
        .limit(5)
        .select(TodoList::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} todo lists", results.len());
    for post in results {
        println!("{}", post.title.unwrap());
        println!("-----------\n");
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