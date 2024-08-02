use diesel::prelude::*;
use crate::schema::{todo_list_items};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(TodoList))]
#[diesel(table_name = todo_list_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoListItem {
    pub id: i32,
    pub content: Option<String>,
    pub todo_list_id: i32,
}