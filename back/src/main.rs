pub mod schema;
pub mod controllers;
pub mod models;
pub mod errors;

use controllers::{UserController,AuthController,Routes};

// для env
use dotenvy::dotenv;

#[macro_use] extern crate rocket;



#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(None)
        .init();

    let rocket = rocket::build();

    // map controllers
    rocket
        .mount("/users/", UserController::routes())
        .mount("/auth", AuthController::routes())

}