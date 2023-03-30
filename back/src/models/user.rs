use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable,Identifiable)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub patronymic: Option<String>,
    pub block: i32,
    pub last_active: NaiveDateTime,
    pub privilege_id: i32,
}

#[derive(Deserialize,Serialize,Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertUser {
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub patronymic: Option<String>,
}

#[derive(Deserialize,Serialize,Clone)]
#[serde(crate = "rocket::serde")]
#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub patronymic: Option<String>,
    pub block: Option<i32>,
    pub last_active: Option<NaiveDateTime>,
    pub privilege_id: i32,
}