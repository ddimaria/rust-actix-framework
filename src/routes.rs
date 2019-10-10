//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    auth::{login, logout},
    health::get_health,
    user::{create_user, get_user, get_users},
};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to_async(get_health))
        .service(
            web::scope("/api/v1")
                // AUTH routes
                .service(
                    web::scope("/auth")
                        .route("/login", web::post().to_async(login))
                        .route("/logout", web::get().to_async(logout)),
                )
                // USER routes
                .service(
                    web::scope("/user")
                        .route("/{id}", web::get().to_async(get_user))
                        .route("", web::get().to_async(get_users))
                        .route("", web::post().to_async(create_user)),
                ),
        );
}
