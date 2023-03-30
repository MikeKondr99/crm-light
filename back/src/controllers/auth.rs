use rocket_jwt::jwt;
use super::Routes;

// use crate::errors::{JsonResult, RequestError};
// use rocket::serde::{json::Json};

pub struct AuthController;

impl Routes for AuthController {
    fn routes() -> Vec<rocket::Route> {
        routes![create_token]
    }
}

const SECRET: &str = "asdfhgfdsdfgghvcv";

#[jwt(SECRET,exp=3600)]
pub struct UserClaim {
    login: String,
    role: String
}

#[post("/")]
fn create_token() -> String {
    let user_claim = UserClaim {
        login: "lox".into(),
        role: "admin".into(),
    };
    UserClaim::sign(user_claim) // Посылаем Jwt токен
}
