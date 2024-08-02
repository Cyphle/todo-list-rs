// @generated automatically by Diesel CLI.

diesel::table! {
    todo_list_items (id) {
        id -> Int4,
        content -> Nullable<Text>,
        todo_list_id -> Int4,
    }
}

diesel::table! {
    todo_lists (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
    }
}

diesel::joinable!(todo_list_items -> todo_lists (todo_list_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo_list_items,
    todo_lists,
);
