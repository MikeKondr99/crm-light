use crate::{errors::{JsonResult, MapStatus}};
use diesel::prelude::*;
use okapi::openapi3::OpenApi;
use crate::models::*;
use crate::schema::vats::dsl::vats;
use rocket::serde::{json::Json};
use super::auth::UserClaim;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub fn get_routes_and_docs() -> (Vec<rocket::Route>, OpenApi){
    openapi_get_routes_spec![get_all]
}

///
/// Получить список всех пользователей
/// 
#[openapi(tag = "Vats")]
#[get("/")]
pub fn get_all(_claim:UserClaim) -> JsonResult<Vec<Vat>> {
    let conn = &mut get_connection()?;
    let res = vats.load::<Vat>(conn).status()?;
    Ok(Json(res))
}