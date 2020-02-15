//! Spin up a HTTPServer

use crate::auth::get_identity_service;
use crate::config::CONFIG;
use crate::database::add_pool;
use crate::routes::routes;
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(get_identity_service())
            .configure(add_pool)
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l).unwrap()
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}
