use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
}
