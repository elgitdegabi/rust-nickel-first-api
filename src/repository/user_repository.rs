use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use log::info;
use self::schema::user::dsl::*;

use crate::config::database::get_pool_connection;
use crate::config::constants::DELETE_OK_STATUS;
use crate::model::user::User;
use crate::model::user::UserUpsert;
use crate::schema;

pub fn get_users() -> Vec<User> {
    let result = user
        .get_results::<User>(&mut get_pool_connection());
    info!("user_repository - get_users - executed");

    return result.unwrap();
}

pub fn get_user(user_id_value: i64) -> User {
    let result = user
        .filter(user_id.eq(user_id_value))
        .get_result::<User>(&mut get_pool_connection());
    info!("user_repository - get_user - executed for user id: {}", user_id_value);

    return result.unwrap();
}

pub fn add_user(user_to_add: UserUpsert) -> User {
    diesel::insert_into(user)
        .values(&user_to_add)
        .execute(&mut get_pool_connection())
        .expect("add_user execution fails");
    info!("user_repository - add_user - executed for user: {:?}", user_to_add);

    // mysql doesn't support RETURNING clause (https://diesel.rs/guides/all-about-inserts.html)
    return get_user_by_alias(user_to_add.user_alias.unwrap());
}

pub fn update_user(user_id_value: i64, user_to_update: UserUpsert) -> User {
    diesel::update(user)
        .filter(user_id.eq(user_id_value))
        .set(&user_to_update)
        .execute(&mut get_pool_connection())
        .expect("update_user execution fails");
    info!("user_repository - update_user - executed for user id: {} - user: {:?}", user_id_value, user_to_update);

    // mysql doesn't support RETURNING clause (https://diesel.rs/guides/all-about-inserts.html)
    return get_user(user_id_value);
}

pub fn delete_user(user_id_value: i64) -> String {
    diesel::delete(user)
        .filter(user_id.eq(user_id_value))
        .execute(&mut get_pool_connection())
        .expect("delete_user execution fails");
    info!("user_repository - delete_user - executed for user id: {}", user_id_value);

    return String::from(DELETE_OK_STATUS);
}

// based on assumption: alias is unique
fn get_user_by_alias(user_alias_value: String) -> User {
    let result = user
        .filter(user_alias.eq(user_alias_value))
        .get_result::<User>(&mut get_pool_connection());

    return result.unwrap();
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes get_users with valid parameters
     * Expectation:
     * A list of users should be retrieved
     */
    #[test]
    fn when_get_users_with_valid_parameters_should_return_user_list() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes get_user with valid parameters
     * Expectation:
     * An user should be retrieved
     */
    #[test]
    fn when_get_user_with_valid_parameters_should_return_user() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes add_user with valid parameters
     * Expectation:
     * An user should be created
     */
    #[test]
    fn when_add_user_with_valid_parameters_should_create_user() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes update_user with valid parameters
     * Expectation:
     * An user should be updated
     */
    #[test]
    fn when_update_user_with_valid_parameters_should_update_user() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes delete_user with valid parameters
     * Expectation:
     * An user should be deleted
     */
    #[test]
    fn when_delete_user_with_valid_parameters_should_delete_user() {
        // TODO to be implemented
        assert_eq!(true, true);
    }
}
