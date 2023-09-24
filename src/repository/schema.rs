// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        steam_id -> Nullable<Varchar>,
        #[max_length = 255]
        psn_auth_code -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
