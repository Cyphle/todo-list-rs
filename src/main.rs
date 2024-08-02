use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use diesel::prelude::*;

extern crate diesel;

use crate::db_connection::establish_connection;
use crate::schema::todo_lists::dsl::todo_lists;
use crate::todo_list_item::*;
use crate::todo_list::*;

mod db_connection;
mod schema;
mod todo_list;
mod todo_list_item;

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

    let connection = &mut establish_connection();

    let todo_list = create_todo_list(connection, "We need more productivity");

    let item : String = String::from("Write more code");

    create_todo_list_item(connection, &item, &todo_list.id);

    let todo_lists_result = todo_lists
        .limit(5)
        .select(TodoList::as_select())
        .load(connection)
        .expect("Error loading posts");

    let items_results = TodoListItem::belonging_to(&todo_lists_result)
        .select(TodoListItem::as_select())
        .load(connection)
        .expect("Error loading posts");

    // get items for a list
    let items_per_list = items_results
        .grouped_by(&todo_lists_result)
        .into_iter()
        .zip(todo_lists_result)
        .map(|(items, list)| (list, items))
        .collect::<Vec<(TodoList, Vec<TodoListItem>)>>();

    println!("Pages per book: \n {items_per_list:?}\n");

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
    use actix_web::{App, http::header::ContentType, test};

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