mod controllers;
mod cors;
mod errors;
mod models;
mod schema;

use rocket_okapi::{
    mount_endpoints_and_merged_docs,
    settings::OpenApiSettings,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
// для env
use dotenvy::dotenv;

use crate::{
    controllers::{auth, users, counterparties,vats,roles,counterparty_statuses},
    cors::Cors,
};

#[macro_use]
extern crate rocket;

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/api/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::builder().format_timestamp(None).init();

    let mut rocket = rocket::build();
    let config = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/api".to_owned(),config,
        "/auth" => auth::get_routes_and_docs(),
        "/users" => users::get_routes_and_docs(),
        "/counterparties" => counterparties::get_routes_and_docs(),
        "/roles" => roles::get_routes_and_docs(),
        "/vats" => vats::get_routes_and_docs(),
        "/counterparty_statuses" => counterparty_statuses::get_routes_and_docs(),

    };
    rocket
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .attach(Cors)
}
