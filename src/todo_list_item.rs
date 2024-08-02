use diesel::prelude::*;
use crate::schema::{todo_list_items};
use crate::todo_list::{TodoList};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(TodoList))]
#[diesel(table_name = todo_list_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoListItem {
    pub id: i32,
    pub content: Option<String>,
    pub todo_list_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = todo_list_items)]
pub struct NewTodoListItems<'a> {
    pub content: &'a String,
    pub todo_list_id: &'a i32,
}

pub fn create_todo_list_item(conn: &mut PgConnection, task: &String, todo_list_id: &i32) -> TodoListItem {

    let new_item = NewTodoListItems { content:task, todo_list_id };

    diesel::insert_into(todo_list_items::table)
        .values(&new_item)
        .returning(TodoListItem::as_returning())
        .get_result(conn)
        .expect("Error saving new todo list item")
}