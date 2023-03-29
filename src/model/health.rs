//extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub status: String,
}
