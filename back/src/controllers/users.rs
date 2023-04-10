use crate::{errors::{JsonResult, MapStatus}};
use diesel::prelude::*;
use okapi::openapi3::OpenApi;
use crate::models::*;
use crate::schema::users::{dsl as user, dsl::users};
use rocket::{serde::{json::Json},http::Status};
use super::auth::UserClaim;
use rocket_okapi::{openapi, openapi_get_routes_spec};

pub fn get_routes_and_docs() -> (Vec<rocket::Route>, OpenApi){
    openapi_get_routes_spec![get_all,get,delete,patch,post]
}

/// Сокращенная версия для получения пользователя по id
pub fn user_by_id(conn: &mut Conn,id: i32) -> Result<User,Status> {
    users.filter(user::id.eq(id)).first(conn).status()
}


///
/// Получить список всех пользователей
/// 
#[openapi(tag = "Users")]
#[get("/")]
fn get_all(_user:UserClaim) -> JsonResult<Vec<User>> {
    let conn = &mut get_connection()?;
    let res = users.load::<User>(conn).status()?;
    Ok(Json(res))
}

///
/// Получить пользователя по id
/// 
#[openapi(tag = "Users")]
#[get("/<id>")]
pub fn get(id:i32,_user:UserClaim) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    let user = user_by_id(conn,id)?;
    Ok(Json(user))
}

///
/// Создать пользователя
/// 
#[openapi(tag = "Users")]
#[post("/", data = "<body>")]
pub fn post(body: Json<InsertUser>,_user:UserClaim) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    _ = diesel::insert_into(users)
        .values(&body.0)
        .execute(conn).status()?;
    let user = user_by_id(conn,1)?;
    Ok(Json(user))

}

///
/// Изменить пользователя
/// 
#[openapi(tag = "Users")]
#[patch("/<id>", data = "<body>")]
pub fn patch(id:i32,body: Json<UpdateUser>,_user:UserClaim) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    let user = user_by_id(conn,id)?;
    diesel::update(&user).set(body.0).execute(conn).status()?;
    Ok(Json(user))
}

///
/// Удалить пользователя
/// 
#[openapi(tag = "Users")]
#[delete("/<id>")]
pub fn delete(id:i32,_user:UserClaim) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    let user = user_by_id(conn,id)?;
    diesel::delete(&user).execute(conn).status()?;
    Ok(Json(user))
}