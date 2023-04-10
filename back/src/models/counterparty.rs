use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Queryable, Identifiable, Insertable, AsChangeset};
use serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;


#[derive(Deserialize,Serialize,Clone,JsonSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable,Identifiable)]
#[diesel(table_name=counterparties)]
#[diesel(belongs_to(Status))]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Vat))]
pub struct Counterparty {
    pub id: i32,
    pub inn: String,
    pub name: String,
    pub vat_id: i32,
    pub kpp: String,
    pub ogrn: String,
    pub bik: String,
    pub role_id: i32,
    pub status_id: i32,
}

#[derive(Deserialize,Serialize,Clone,JsonSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Insertable)]
#[diesel(table_name = counterparties)]
pub struct InsertCounterparty {
    pub inn: String,
    pub name: String,
    pub vat_id: i32,
    pub kpp: String,
    pub ogrn: String,
    pub bik: String,
    pub role_id: i32,
    pub status_id: i32,
}

#[derive(Deserialize,Serialize,Clone,JsonSchema)]
#[serde(crate = "rocket::serde")]
#[derive(AsChangeset)]
#[diesel(table_name = counterparties)]
pub struct UpdateCounterparty {
    pub inn: Option<String>,
    pub name: Option<String>,
    pub vat_id: Option<i32>,
    pub kpp: Option<String>,
    pub ogrn: Option<String>,
    pub bik: Option<String>,
    pub role_id: Option<i32>,
    pub status_id: Option<i32>,
}
