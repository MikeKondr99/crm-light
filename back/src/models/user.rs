use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Queryable, Identifiable, Insertable, AsChangeset};
use serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;


#[derive(Deserialize,Serialize,Clone,JsonSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable,Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub patronymic: Option<String>,
    pub block: i32,
    pub last_active: NaiveDateTime,
}


#[derive(Deserialize,Serialize,Clone,JsonSchema)]
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

#[derive(Deserialize, Serialize, Clone,JsonSchema)]
#[serde(crate = "rocket::serde")]
#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub patronymic: Option<String>,
    pub block: Option<i32>,
    pub last_active: Option<NaiveDateTime>,
}

