// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        completed -> Bool,
    }
}
