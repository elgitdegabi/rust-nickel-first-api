use std::collections::LinkedList;

use diesel::mysql::MysqlConnection;
use log::info;

use crate::config::constants::DELETE_OK_STATUS;
use crate::model::user::User;

static mut REPOSITORY_CONNECTION: Option<MysqlConnection> = None;

pub unsafe fn set_connection(connection: Option<MysqlConnection>) {
    REPOSITORY_CONNECTION = connection;
}

pub fn get_users() -> LinkedList<User> {
    info!("user_repository - get_users - executed");

    // TODO - TO BE IMPLEMENTED
    return LinkedList::new();
}

pub fn get_user(user_id: i32) -> User {
    info!("user_repository - get_user - executed for user id: {}", user_id);

    // TODO - TO BE IMPLEMENTED
    return User {
        id: Option::from(user_id),
        firstname: String::from("First Name"),
        lastname: String::from("Last Name"),
    };
}

pub fn add_user(user: User) -> User {
    info!("user_repository - add_user - executed for user: {:?}", user);

    // TODO - TO BE IMPLEMENTED
    return user;
}

pub fn update_user(user_id: i32, user: User) -> User {
    info!("user_repository - update_user - executed for user id: {} - user: {:?}", user_id, user);

    // TODO - TO BE IMPLEMENTED
    return User {
        id: Option::from(user_id),
        firstname: user.firstname,
        lastname: String::from("Updated Last Name"),
    };
}

pub fn delete_user(user_id: i32) -> String {
    info!("user_repository - delete_user - executed for user id: {}", user_id);

    // TODO - TO BE IMPLEMENTED
    return String::from(DELETE_OK_STATUS);
}
