use std::{env, io};

use actix_web::{middleware, App, HttpServer};

#[macro_use]
extern crate serde_derive;

mod minner;
mod minner_controller;
mod utils;
mod wallet;
mod wallet_controller;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(wallet_controller::list_wallet)
            .service(wallet_controller::create_wallet)
            .service(wallet_controller::get_wallet)
            .service(minner_controller::list_minners)
            .service(minner_controller::create_minner)
            .service(minner_controller::get_minner)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
