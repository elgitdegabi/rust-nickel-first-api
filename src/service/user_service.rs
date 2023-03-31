use log::info;

use crate::model::user::{User, UserUpsert};
use crate::repository::*;

pub fn get_users() -> Vec<User> {
    info!("user_service - get_users - executed");
    return user_repository::get_users();
}

pub fn get_user(user_id: i64) -> User {
    info!("user_service - get_user - executed for user id: {}", user_id);
    return user_repository::get_user(user_id);
}

pub fn add_user(user: UserUpsert) -> User {
    info!("user_service - add_user - executed for user: {:?}", user);
    return user_repository::add_user(user);
}

pub fn update_user(user_id: i64, user: UserUpsert) -> User {
    info!("user_service - update_user - executed for user id: {} - user: {:?}", user_id, user);
    return user_repository::update_user(user_id, user);
}

pub fn delete_user(user_id: i64) -> String {
    info!("user_service - delete_user - executed for user id: {}", user_id);
    return user_repository::delete_user(user_id);
}
