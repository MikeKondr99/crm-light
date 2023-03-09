// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        name -> Text,
        age -> Integer,
        alive -> Bool,
    }
}
