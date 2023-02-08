// @generated automatically by Diesel CLI.

diesel::table! {
    service_contexts (id) {
        id -> Int4,
        maintenance -> Bool,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        completed -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    service_contexts,
    todos,
);
