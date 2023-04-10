use crate::{errors::{JsonResult, MapStatus}};
use diesel::prelude::*;
use okapi::openapi3::OpenApi;
use crate::models::*;
use crate::schema::counterparty_statuses::dsl::counterparty_statuses;
use rocket::serde::json::Json;
use super::auth::UserClaim;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub fn get_routes_and_docs() -> (Vec<rocket::Route>, OpenApi){
    openapi_get_routes_spec![get_all]
}

///
/// Получить список всех пользователей
/// 
#[openapi(tag = "Statuses")]
#[get("/")]
pub fn get_all(_claim:UserClaim) -> JsonResult<Vec<CounterpartyStatus>> {
    let conn = &mut get_connection()?;
    let res = counterparty_statuses.load::<CounterpartyStatus>(conn).status()?;
    Ok(Json(res))
}