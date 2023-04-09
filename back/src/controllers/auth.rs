use chrono::NaiveDateTime;
use rocket::{serde::{json::Json},http::Status};
use diesel::prelude::*;
use okapi::openapi3::{OpenApi, SecurityScheme, SecuritySchemeData, Object, SecurityRequirement};
use rocket_jwt::jwt;
use rocket::post;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{errors::{JsonResult, MapStatus}, models::{UserPrivilege, Privilege}};
use crate::{
    schema::users::{dsl as user, dsl::users},
    schema::user_privileges::{dsl as user_privilege, dsl::user_privileges},
    schema::privileges::{dsl as privilege, dsl::privileges},
    models::{User,get_connection},
};

use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi, openapi_get_routes_spec,
    request::{OpenApiFromRequest, RequestHeaderInput}, 
};

const SECRET: &str = "asdfhgfdsdfgghvcv";

pub fn get_routes_and_docs() -> (Vec<rocket::Route>, OpenApi){
    openapi_get_routes_spec![create_token,create_token2]
}

#[derive(Deserialize,Serialize,Clone,JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    pub token: String,
    pub privileges: Vec<String>,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub patronymic: Option<String>,
    pub last_active: NaiveDateTime,
}

#[derive(Deserialize,Serialize,Clone,JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[jwt(SECRET,exp=3600)]
#[derive(JsonSchema)]
pub struct UserClaim {
    login: String,
}

impl<'a> OpenApiFromRequest<'a> for UserClaim {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some("Requires an JWT token to access".to_owned()),
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".into(),
                location: "header".into(),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("JwtTokenAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "JwtTokenAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}

///
/// Получить JWT token без данных (тест)
/// 
#[openapi(tag="Auth")]
#[post("/")]
pub fn create_token() -> String {
    let user_claim = UserClaim {
        login: "lox".into(),
    };
    UserClaim::sign(user_claim) // Посылаем Jwt токен
}

///
/// Получить JWT token через логин и пароль
/// 
#[openapi(tag="Auth")]
#[post("/new", data = "<body>")]
pub fn create_token2(body:Json<LoginInfo>) -> JsonResult<UserInfo> {
    let conn = &mut get_connection()?;
    let user: User = users
        .filter(user::username.eq(&body.username))
        .filter(user::password.eq(&body.password))
        .first(conn).status()?;
    let pris: Vec<String> = user_privileges
        .inner_join(privileges.on(privilege::id.eq(user_privilege::privilege_id)))
        .filter(user_privilege::user_id.eq(user.id))
        .select(privilege::privilege_name)
        .load(conn).status()?;
    let user_claim = UserClaim {
        login: body.username.clone(),
    };
    Ok(Json(UserInfo {
        token: UserClaim::sign(user_claim),
        username: String::from(&body.username),
        privileges: pris,
        first_name: user.first_name,
        last_name: user.last_name,
        patronymic: user.patronymic,
        last_active: user.last_active,
    }))
    // UserClaim::sign(user_claim) // Посылаем Jwt токен
}
