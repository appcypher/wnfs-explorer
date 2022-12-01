// @generated automatically by Diesel CLI.

diesel::table! {
    store (id) {
        id -> Int4,
        store_name -> Varchar,
        cid -> Bytea,
        bytes -> Bytea,
    }
}
