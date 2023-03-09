
use super::schema::*;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable,Identifiable)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub age: i32,
    pub alive: bool,
}

#[derive(Deserialize,Serialize,Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertUser {
    pub name: String,
    pub age: i32,
    pub alive: bool,
}

#[derive(Deserialize,Serialize,Clone)]
#[serde(crate = "rocket::serde")]
#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: String,
    pub age: i32,
    pub alive: bool,
}