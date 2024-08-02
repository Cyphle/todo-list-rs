use diesel::prelude::*;
use crate::schema::todo_lists;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = todo_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoList {
    pub id: i32,
    pub title: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = todo_lists)]
pub struct NewTodoList<'a> {
    pub title: &'a str,
}

pub fn create_todo_list(conn: &mut PgConnection, title: &str) -> TodoList {
    use crate::schema::todo_lists;

    let new_post = NewTodoList { title };

    diesel::insert_into(todo_lists::table)
        .values(&new_post)
        .returning(TodoList::as_returning())
        .get_result(conn)
        .expect("Error saving new todo list")
}