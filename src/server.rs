//! Spin up a HTTPServer

use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

use crate::config::CONFIG;
use crate::database::{init_pool, Pool};
use crate::routes::routes;

pub fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool: Pool = init_pool().expect("Failed to create connection pool");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(&CONFIG.server).unwrap()
    };

    server.run()
}
