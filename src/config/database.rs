extern crate dotenv;

use std::env;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn connect() -> MysqlConnection {
    dotenv().ok();

    let host = env::var("MYSQL_HOST").expect("MYSQL_HOST must be set");
    let user = env::var("MYSQL_USER").expect("MYSQL_USER must be set");
    let pass = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set");
    let db_name = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE must be set");

    let mut db_url = String::from("mysql://");
    db_url.push_str(&user);
    db_url.push_str(":");
    db_url.push_str(&pass);
    db_url.push_str("@");
    db_url.push_str(&host);
    db_url.push_str("/");
    db_url.push_str(&db_name);

    return MysqlConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));
}
