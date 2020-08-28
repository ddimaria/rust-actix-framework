//! Inject dotenv and env variables into the Config struct
//!
//! ```ignore
//! use actix_cors::Cors;
//! use actix_web::{middleware::Logger, App, HttpServer};
//! use listenfd::ListenFd;
//! use actix_framework::auth::get_identity_service;
//! use actix_framework::cache::add_cache;
//! use actix_framework::config::CONFIG;
//! use actix_framework::database::add_pool;
//! use actix_framework::routes::routes;
//! use actix_framework::state::new_state;
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!     dotenv::dotenv().ok();
//!     env_logger::init();
//!
//!     // Create the application state
//!     // String is used here, but it can be anything
//!     // Invoke in hanlders using data: AppState<'_, String>
//!     let data = new_state::<String>();
//!
//!     // Initialize the file system listener
//!     let mut listenfd = ListenFd::from_env();
//!     let mut server = HttpServer::new(move || {
//!         App::new()
//!             // Add the default logger
//!             .wrap(Logger::default())
//!
//!             // Accept all CORS
//!             // For more options, see https://docs.rs/actix-cors
//!             .wrap(Cors::new().supports_credentials().finish())
//!
//!             // Adds Identity Service for use in the Actix Data Extractor
//!             // In a handler, add "id: Identity" param for auto extraction
//!             .wrap(get_identity_service())
//!
//!             // Adds Application State for use in the Actix Data Extractor
//!             // In a handler, add "data: AppState<'_, String>" param for auto extraction
//!             .app_data(data.clone())
//!
//!             // Adds the Redis Cache for use in the Actix Data Extractor
//!             // In a handler, add "cache: Cache" param for auto extraction
//!             .configure(add_cache)
//!
//!             // Adds a Database Pool for use in the Actix Data Extractor
//!             // In a handler, add "pool: Data<PoolType>" param for auto extraction
//!             .configure(add_pool)
//!
//!             // Pull in default framework defaults
//!             // This can be removed if they're not needed
//!             .configure(routes)
//!     });
//!
//!     server = if let Some(l) = listenfd.take_tcp_listener(0)? {
//!         server.listen(l)?
//!     } else {
//!         server.bind(&CONFIG.server)?
//!     };
//!
//!     server.run().await
//! }
//!

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
pub mod paginate;
pub mod routes;
pub mod schema;
pub mod server;
pub mod state;
pub mod tests;
pub mod validate;
