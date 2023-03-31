pub mod schema;
pub mod controllers;
pub mod models;
pub mod errors;

use rocket_okapi::{swagger_ui::{make_swagger_ui, SwaggerUIConfig}, mount_endpoints_and_merged_docs, settings::OpenApiSettings};
// для env
use dotenvy::dotenv;

use crate::controllers::{auth, users};

#[macro_use] extern crate rocket;

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/api/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(None)
        .init();

    let mut rocket = rocket::build();
    let config = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/api".to_owned(),config,
        "/auth" => auth::get_routes_and_docs(),
        "/users" => users::get_routes_and_docs(),
    };
    rocket.mount("/swagger",make_swagger_ui(&get_docs()))
}