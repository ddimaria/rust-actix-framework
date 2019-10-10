#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

use crate::server::server;

mod auth;
mod config;
mod database;
mod errors;
pub mod handlers;
mod helpers;
mod models;
mod routes;
mod schema;
mod server;
mod tests;
mod validate;

fn main() -> std::io::Result<()> {
    server()
}
