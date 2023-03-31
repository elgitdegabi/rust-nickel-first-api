#[macro_use]
extern crate nickel;

use nickel::Nickel;

use config::constants::HOST;
use config::database::*;
use controller::controller::config_endpoints;
use log::info;

mod config;
mod controller;
mod model;
mod service;
mod repository;
mod schema;

fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

    unsafe {
        info!("DB pool - starting...");
        DB_POOL = Option::Some(create_db_pool());
        info!("DB pool - started OK: {:?}", DB_POOL.as_ref().unwrap());
    }

    info!("Nickel server - starting...");
    let server = Nickel::new();
    info!("Nickel server - started OK");

    info!("Nickel end-points configuration - starting...");
    config_endpoints(server).listen(HOST).unwrap();
    info!("Nickel end-points configuration - started OK");

    info!("app started OK");
}
