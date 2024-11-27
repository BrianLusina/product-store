// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        cost -> Float8,
        active -> Bool,
    }
}
