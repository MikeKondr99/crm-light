use okapi::openapi3::{OpenApi, SecurityScheme, SecuritySchemeData, Object, SecurityRequirement};
use rocket_jwt::jwt;
use rocket::post;
use schemars::JsonSchema;
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi, openapi_get_routes_spec,
    request::{OpenApiFromRequest, RequestHeaderInput}, 
};

const SECRET: &str = "asdfhgfdsdfgghvcv";

pub fn get_routes_and_docs() -> (Vec<rocket::Route>, OpenApi){
    openapi_get_routes_spec![create_token]
}


#[jwt(SECRET,exp=3600)]
#[derive(JsonSchema)]
pub struct UserClaim {
    login: String,
    role: String
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
                name: "Authorization".to_owned(),
                location: "header".to_owned(),
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
        role: "admin".into(),
    };
    UserClaim::sign(user_claim) // Посылаем Jwt токен
}
