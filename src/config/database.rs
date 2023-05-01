extern crate dotenv;

use std::env;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use log::info;

pub static mut DB_POOL: Option<diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>> = None;

pub fn create_db_pool() -> diesel::r2d2::Pool<ConnectionManager<MysqlConnection>> {
    info!("create_db_pool - starting...");
    dotenv().ok();

    let host = env::var("MYSQL_HOST").expect("MYSQL_HOST must be set");
    info!("create_db_pool - host retrieved OK");

    let user = env::var("MYSQL_USER").expect("MYSQL_USER must be set");
    info!("create_db_pool - user retrieved OK");

    let pass = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set");
    info!("create_db_pool - pass retrieved OK");

    let db_name = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE must be set");
    info!("create_db_pool - database retrieved OK");

    let mut db_url = String::from("mysql://");
    db_url.push_str(&user);
    db_url.push(':');
    db_url.push_str(&pass);
    db_url.push('@');
    db_url.push_str(&host);
    db_url.push('/');
    db_url.push_str(&db_name);
    info!("create_db_pool - db url retrieved OK");

    let connection_manager = ConnectionManager::<MysqlConnection>::new(db_url);
    info!("create_db_pool - connection_manager created OK");

    diesel::r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(connection_manager)
        .expect("Error building pool or connecting to DB")
}

pub fn get_pool_connection() -> diesel::r2d2::PooledConnection<ConnectionManager<MysqlConnection>> {
    unsafe {
        return DB_POOL.as_ref().unwrap().get().unwrap();
    }
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes create_db_pool
     * Expectation:
     * A db_pool should be created
     */
    #[test]
    fn when_create_db_pool_should_create_db_pool() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes get_pool_connection before create the pool connection
     * Expectation:
     * A None value should be retrieved
     */
    #[test]
    fn when_get_pool_connection_and_pool_is_not_created_should_retrieve_none() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes get_pool_connection after create the pool connection
     * Expectation:
     * A pool connection should be retrieved
     */
    #[test]
    fn when_get_pool_connection_should_retrieve_the_connection_pool() {
        // TODO to be implemented
        assert_eq!(true, true);
    }
}
