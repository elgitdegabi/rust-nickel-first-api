#[macro_use]
extern crate nickel;

use config::constants::HOST;
use config::database::connect;
use repository::user_repository::set_connection;
use controller::controller::config_endpoints;
use nickel::Nickel;

mod config;
mod controller;
mod model;
mod service;
mod repository;

fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

    let db_connection = connect();
    unsafe { set_connection(Some(db_connection)); }

    let server = Nickel::new();
    config_endpoints(server).listen(HOST).unwrap();
}
