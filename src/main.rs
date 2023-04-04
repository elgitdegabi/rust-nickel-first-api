#[macro_use]
extern crate nickel;

use nickel::Nickel;

use config::constants::HOST;
use config::database::*;
use controller::controller::config_endpoints;
use log::info;
use dotenv::dotenv;
use std::env;

mod config;
mod controller;
mod model;
mod service;
mod repository;
mod schema;

fn main() {
    dotenv().ok();

    let log_config_file = env::var("LOG4RS_CONFIG_FILE").unwrap_or("logging_config.yaml".to_string());
    log4rs::init_file(log_config_file, Default::default()).unwrap();

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
