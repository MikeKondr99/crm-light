use crate::{errors::{JsonResult, RequestError}};
use diesel::prelude::*;
use crate::models::*;
use crate::schema::users::dsl::*;
use rocket::serde::{json::Json};
use super::{Routes, UserClaim};

pub struct UserController;

impl Routes for UserController {
    fn routes() -> Vec<rocket::Route> {
        routes![get_all,get,patch,post,delete]
    }
}

/// Сокращенная версия для получения пользователя по id
fn user_by_id(conn: &mut Conn,id: i32) -> Result<User,RequestError> {
    Ok(users.filter(user_id.eq(id)).first(conn)?)
}


#[get("/")]
fn get_all() -> JsonResult<Vec<User>> {
    let conn = &mut get_connection()?;
    let res = users.load::<User>(conn)?;
    Ok(Json(res))
}

#[get("/<id>")]
fn get(id:i32) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    let user = user_by_id(conn,id)?;
    Ok(Json(user))
}

// Нет проверки 
#[post("/", data = "<body>")]
fn post(body: Json<InsertUser>,user:UserClaim) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    let _ = diesel::insert_into(users)
        .values(&body.0)
        .execute(conn)?;
    let user = user_by_id(conn,1)?;
    Ok(Json(user))

}

#[patch("/<id>", data = "<body>")]
fn patch(id:i32,body: Json<UpdateUser>) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    let user = user_by_id(conn,id)?;
    diesel::update(&user).set(body.0).execute(conn)?;
    Ok(Json(user))
}

#[delete("/<id>")]
fn delete(id:i32) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    let user = user_by_id(conn,id)?;
    diesel::delete(&user).execute(conn)?;
    Ok(Json(user))
}