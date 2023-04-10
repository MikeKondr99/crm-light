// @generated automatically by Diesel CLI.

diesel::table! {
    counterparties (id) {
        id -> Integer,
        inn -> Text,
        name -> Text,
        vat_id -> Integer,
        kpp -> Text,
        ogrn -> Text,
        bik -> Text,
        role_id -> Integer,
        status_id -> Integer,
    }
}

diesel::table! {
    counterparty_statuses (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    privileges (id) {
        id -> Integer,
        privilege_name -> Text,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        name -> Text,
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

diesel::table! {
    vats (id) {
        id -> Integer,
        name -> Text,
        value -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    counterparties,
    counterparty_statuses,
    privileges,
    roles,
    user_privileges,
    users,
    vats,
);
