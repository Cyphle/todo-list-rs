use crate::http::handlers::examples::{echo, hello};
use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;
use crate::http::handlers::todo_lists::{create_todo_list, get_todo_list_by_id, get_todo_lists};
use crate::http::handlers::state::HandlerState;

// pub async fn config(db_connection: &DatabaseConnection) -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .app_data(web::Data::new(HandlerState {
//                 db_connection
//             }))
//             .service(hello)
//             .service(echo)
//             .service(get_todo_lists)
//             .service(get_todo_list_by_id)
//             .service(create_todo_list)
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }