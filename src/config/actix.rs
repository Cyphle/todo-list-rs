use crate::http::handlers::examples::{echo, hello};
use actix_web::{App, HttpServer};

pub async fn config() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}