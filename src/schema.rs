// @generated automatically by Diesel CLI.

diesel::table! {
    todo_list (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
    }
}

diesel::table! {
    todo_list_item (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        content -> Nullable<Text>,
        todo_list_id -> Nullable<Int4>,
    }
}

diesel::joinable!(todo_list_item -> todo_list (todo_list_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo_list,
    todo_list_item,
);
