// @generated automatically by Diesel CLI.

diesel::table! {
    animals (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    questions (id) {
        id -> Integer,
        content -> Text,
        yes_id -> Nullable<Integer>,
        no_id -> Nullable<Integer>,
        is_last -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    animals,
    questions,
);
