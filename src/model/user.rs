use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};
use schema::user;
use chrono::NaiveDateTime;

use crate::schema;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: i64,
    pub user_name: Option<String>,
    pub user_alias: Option<String>,
    pub user_address: Option<String>,
    pub user_create_ts: Option<NaiveDateTime>,
    pub user_update_ts: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = user)]
pub struct UserUpsert {
    pub user_name: Option<String>,
    pub user_alias: Option<String>,
    pub user_address: Option<String>,
    pub user_create_ts: Option<NaiveDateTime>,
    pub user_update_ts: Option<NaiveDateTime>,
}
