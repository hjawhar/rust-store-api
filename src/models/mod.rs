use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
}
