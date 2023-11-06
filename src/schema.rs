// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        product -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    users,
);
