//! Spin up a HTTPServer

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

use crate::config::CONFIG;
use crate::database::add_pool;
use crate::routes::routes;

pub fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth")
                    .max_age_time(chrono::Duration::minutes(20))
                    .secure(false),
            ))
            .configure(add_pool)
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(&CONFIG.server).unwrap()
    };

    server.run()
}
