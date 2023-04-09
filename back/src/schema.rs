// @generated automatically by Diesel CLI.

diesel::table! {
    privileges (id) {
        id -> Integer,
        privilege_name -> Text,
    }
}

diesel::table! {
    user_privileges (id) {
        id -> Integer,
        user_id -> Integer,
        privilege_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        patronymic -> Nullable<Text>,
        block -> Integer,
        last_active -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    privileges,
    user_privileges,
    users,
);
