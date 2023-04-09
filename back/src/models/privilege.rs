use crate::schema::*;
use super::User;
use diesel::{Queryable, Identifiable, Insertable, AsChangeset, Associations};
use serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;


#[derive(Deserialize,Serialize,Clone,JsonSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable,Identifiable)]
pub struct Privilege {
    pub id: i32,
    pub privilege_name: String,
}

#[derive(Deserialize,Serialize,Clone,JsonSchema,Associations)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable,Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Privilege))]
pub struct UserPrivilege {
    pub id: i32,
    pub user_id: i32,
    pub privilege_id: i32,
}