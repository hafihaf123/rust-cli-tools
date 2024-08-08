// @generated automatically by Diesel CLI.

diesel::table! {
    questions (id) {
        id -> Integer,
        content -> Text,
        yes_id -> Nullable<Integer>,
        no_id -> Nullable<Integer>,
        is_last -> Integer,
    }
}
