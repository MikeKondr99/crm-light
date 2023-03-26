// @generated automatically by Diesel CLI.

diesel::table! {
    data_users (id) {
        id -> Integer,
        login -> Nullable<Text>,
        password -> Nullable<Text>,
        name -> Nullable<Text>,
        surname -> Nullable<Text>,
        patronymic -> Nullable<Text>,
        block -> Integer,
        last_active -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        name -> Text,
        age -> Integer,
        alive -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    data_users,
    users,
);
