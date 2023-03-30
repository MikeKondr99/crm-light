// @generated automatically by Diesel CLI.

diesel::table! {
    privileges (privilege_id) {
        privilege_id -> Integer,
        privilege_name -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        username -> Text,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        patronymic -> Nullable<Text>,
        block -> Integer,
        last_active -> Timestamp,
        privilege_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    privileges,
    users,
);
