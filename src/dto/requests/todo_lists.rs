#[derive(serde::Deserialize)]
pub struct CreateTodoListRequest {
    pub title: String,
}