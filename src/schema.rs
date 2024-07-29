// @generated automatically by Diesel CLI.

diesel::table! {
    todo_lists (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
    }
}
