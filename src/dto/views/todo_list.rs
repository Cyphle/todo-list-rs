use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TodoListView {
    pub id: i32,
    pub title: String,
}