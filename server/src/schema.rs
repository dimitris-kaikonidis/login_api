// @generated automatically by Diesel CLI.

diesel::table! {
    passwords (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        verifier -> Varchar,
        #[max_length = 255]
        salt -> Varchar,
    }
}

diesel::joinable!(passwords -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    passwords,
    users,
);
