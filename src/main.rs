use std::{env, io};

use actix_web::{middleware, web::Data, App, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};

#[macro_use]
extern crate serde_derive;

mod minner;
mod minner_controller;
mod schema;
mod utils;
mod wallet;
mod wallet_controller;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_to_pool(poo: Data<DBPool>) -> DBPooledConnection {
    poo.get().expect("Failed to reach DB connection pool.")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let database_url = dotenv::var("DATABASE_URL").expect("Database url not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to initialize DB connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
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
