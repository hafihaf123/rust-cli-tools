// @generated automatically by Diesel CLI.

diesel::table! {
    animals (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    questions (id) {
        id -> Integer,
        question -> Text,
        yes_id -> Nullable<Integer>,
        no_id -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    animals,
    questions,
);
