// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
