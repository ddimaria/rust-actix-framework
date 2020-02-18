#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

use crate::server::server;

pub mod auth;
pub mod cache;
pub mod config;
pub mod database;
pub mod errors;
pub mod extractors;
pub mod handlers;
pub mod helpers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod server;
pub mod state;
pub mod tests;
pub mod validate;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
