pub mod schema;
pub mod models;
pub mod errors;

//errors
use errors::{JsonResult, RequestError};

// для env
use dotenvy::dotenv;
use std::env; 

// diesel
use diesel::prelude::*;
use models::*;
use diesel_logger::LoggingConnection;

use schema::users::dsl::*;

// rocket
use rocket::serde::{json::Json};

#[macro_use] extern crate rocket;


type Conn = LoggingConnection<SqliteConnection>;
/// Выдаёт Connection для с БД
fn get_connection() -> Result<Conn,RequestError> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(LoggingConnection::new(SqliteConnection::establish(&database_url)?))
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
fn post(body: Json<InsertUser>) -> JsonResult<User> {
    let conn = &mut get_connection()?;
    let _ = diesel::insert_into(users)
        .values(&body.0)
        .execute(conn)?;
    Ok(Json(User {user_id:0,name:"TODO".to_owned(),age:0,alive:false}))
    // Не могу найти добавленного юзера
    //let user = user_by_id(conn,body.0.user_id)?;
    //Ok(Json(user))

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





#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(None)
        .init();

    rocket::build().mount("/users/", routes![get_all,get,post,patch,delete])
}