use std::collections::LinkedList;

use log::info;

use crate::config::constants::DELETE_OK_STATUS;
use crate::model::user::User;

pub fn get_users() -> LinkedList<User> {
    info!("get_users - executed");
    return LinkedList::new();
}

pub fn get_user(user_id: i32) -> User {
    info!("get_user - executed for user id: {}", user_id);
    return User {
        id: Option::from(user_id),
        firstname: String::from("First Name"),
        lastname: String::from("Last Name"),
    };
}

pub fn add_user(user: User) -> User {
    info!("add_user - executed for user: {:?}", user);
    return user;
}

pub fn update_user(user_id: i32, user: User) -> User {
    info!("update_user - executed for user id: {} - user: {:?}", user_id, user);
    return User {
        id: Option::from(user_id),
        firstname: user.firstname,
        lastname: String::from("Updated Last Name"),
    };
}

pub fn delete_user(user_id: i32) -> String {
    info!("delete_user - executed for user id: {}", user_id);
    return String::from(DELETE_OK_STATUS);
}
