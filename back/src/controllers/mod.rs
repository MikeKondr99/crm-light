pub mod auth;
pub mod users;

pub use auth::*;
pub use users::*;

pub trait Routes {
    fn routes() -> Vec<rocket::Route>;
}