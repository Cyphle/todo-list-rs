// @generated automatically by Diesel CLI.

diesel::table! {
    to_do_list (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
    }
}

diesel::table! {
    to_do_list_item (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        content -> Nullable<Text>,
        to_do_list_id -> Nullable<Int4>,
    }
}

diesel::joinable!(to_do_list_item -> to_do_list (to_do_list_id));

diesel::allow_tables_to_appear_in_same_query!(
    to_do_list,
    to_do_list_item,
);
