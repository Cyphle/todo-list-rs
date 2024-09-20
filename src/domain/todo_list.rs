
pub struct CreateTodoListCommand {
    pub title: String
}

#[derive(Debug, PartialEq)]
pub struct TodoList {
    pub id: i32,
    pub title: String
}