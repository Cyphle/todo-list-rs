use crate::http::handlers::state::HandlerState;
use crate::repositories;
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::domain::todo_list::CreateTodoListCommand;
use crate::dto::requests::todo_lists::CreateTodoListRequest;

#[get("/todo_lists")]
async fn get_todo_lists(state: web::Data<HandlerState>) -> impl Responder {
    match repositories::todo_lists::find_all(&state.db_connexion).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/todo_lists/{id}")]
async fn get_todo_list_by_id(path: web::Path<i32>, state: web::Data<HandlerState>) -> impl Responder {
    match repositories::todo_lists::find_one_by_id(&state.db_connexion, path.into_inner()).await {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/todo_lists")]
async fn create_todo_list(payload: web::Json<CreateTodoListRequest>, state: web::Data<HandlerState>) -> impl Responder {
    match repositories::todo_lists::create(
        &state.db_connexion,
        CreateTodoListCommand {
            title: payload.into_inner().title.to_owned(),
        },
    ).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::header::ContentType;
    use actix_web::{test, App};
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase};
    use crate::{echo, hello};
    use crate::http::handlers::state::HandlerState;
    use crate::http::handlers::todo_lists::{create_todo_list, get_todo_list_by_id, get_todo_lists};
    use actix_web::{get, post, web, HttpResponse, Responder};

    fn get_mock_database() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                vec![entity::todo_lists::Model {
                    id: 1,
                    title: "New York Cheese".to_owned(),
                }],
            ])
            .into_connection()
    }

    #[actix_web::test]
    async fn should_get_todo_lists() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(HandlerState {
                    db_connexion: get_mock_database()
                }))
                .service(get_todo_lists)
        ).await;
        let req = test::TestRequest::get().uri("/todo_lists")
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn should_get_one_todo_list_by_id() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(HandlerState {
                    db_connexion: get_mock_database()
                }))
                .service(get_todo_list_by_id)
        ).await;
        let req = test::TestRequest::get().uri("/todo_lists/123")
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn should_create_todo_list() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(HandlerState {
                    db_connexion: get_mock_database()
                }))
                .service(create_todo_list)
        ).await;

        let req = test::TestRequest::post()
            .set_payload("{ \"title\": \"Bonjour\" }")
            .insert_header(ContentType::json())
            .uri("/todo_lists").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    mod actix_tests {

        // #[actix_web::test]
        // async fn test_echo_post() {
        //     let app = test::init_service(App::new().service(echo)).await;
        //     let req = test::TestRequest::post().uri("/echo").to_request();
        //     let resp = test::call_service(&app, req).await;
        //     assert!(resp.status().is_success());
        // }
        //
        // #[actix_web::test]
        // async fn test_echo_post_error() {
        //     let app = test::init_service(App::new().service(echo)).await;
        //     let req = test::TestRequest::post().uri("/").to_request();
        //     let resp = test::call_service(&app, req).await;
        //     assert!(resp.status().is_client_error());
        // }
    }
}