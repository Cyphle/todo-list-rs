use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::to_do_list)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoListEntity {
    pub id: i32,
    pub title: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::to_do_list_item)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoListItemEntity {
    pub id: i32,
    pub title: String,
    pub content: String
}