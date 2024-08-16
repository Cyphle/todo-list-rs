use serde::{Deserialize, Serialize};
use entity::todo_lists;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoListView {
    pub id: Option<i32>,
    pub title: String,
}

impl TodoListView {
    pub fn new(todo: todo_lists::Model) -> Self {
        Self { id: Some(todo.id), title: todo.title }
    }
}

#[derive(Clone, Debug)]
pub struct TodoList {
    pub title: String,
}

impl TodoList {
    pub fn new(todo: TodoListView) -> Self {
        Self { title: todo.title }
    }
}