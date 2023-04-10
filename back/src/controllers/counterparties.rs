use crate::{errors::{JsonResult, MapStatus}};
use diesel::prelude::*;
use okapi::openapi3::OpenApi;
use crate::models::*;
use crate::schema::counterparties::{dsl as counterparty, dsl::counterparties};
use rocket::{serde::{json::Json},http::Status};
use super::auth::UserClaim;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub fn get_routes_and_docs() -> (Vec<rocket::Route>, OpenApi){
    openapi_get_routes_spec![get_all,get,delete,patch,post]
}

/// Сокращенная версия для получения пользователя по id
pub fn counterparty_by_id(conn: &mut Conn,id: i32) -> Result<Counterparty,Status> {
    counterparties.filter(counterparty::id.eq(id)).first(conn).status()
}


///
/// Получить список всех пользователей
/// 
#[openapi(tag = "Counterparties")]
#[get("/")]
pub fn get_all(claim:UserClaim) -> JsonResult<Vec<Counterparty>> {
    let conn = &mut get_connection()?;
    let res = counterparties.load::<Counterparty>(conn).status()?;
    Ok(Json(res))
}

///
/// Получить пользователя по id
/// 
#[openapi(tag = "Counterparties")]
#[get("/<id>")]
pub fn get(id:i32, claim:UserClaim) -> JsonResult<Counterparty> {
    let conn = &mut get_connection()?;
    let res = counterparty_by_id(conn,id)?;
    Ok(Json(res))
}

///
/// Создать пользователя
/// 
#[openapi(tag = "Counterparties")]
#[post("/", data = "<body>")]
pub fn post(body: Json<InsertCounterparty>,claim:UserClaim) -> JsonResult<Counterparty> {
    let conn = &mut get_connection()?;
    _ = diesel::insert_into(counterparties)
        .values(&body.0)
        .execute(conn).status()?;
    let res = counterparty_by_id(conn,1)?;
    Ok(Json(res))
}

///
/// Изменить пользователя
/// 
#[openapi(tag = "Counterparties")]
#[patch("/<id>", data = "<body>")]
pub fn patch(id:i32,body: Json<UpdateCounterparty>,claim:UserClaim) -> JsonResult<Counterparty> {
    let conn = &mut get_connection()?;
    let res = counterparty_by_id(conn,id)?;
    diesel::update(&res).set(body.0).execute(conn).status()?;
    Ok(Json(res))
}

///
/// Удалить пользователя
/// 
#[openapi(tag = "Counterparties")]
#[delete("/<id>")]
pub fn delete(id:i32,claim:UserClaim) -> JsonResult<Counterparty> {
    let conn = &mut get_connection()?;
    let res = counterparty_by_id(conn,id)?;
    diesel::delete(&res).execute(conn).status()?;
    Ok(Json(res))
}