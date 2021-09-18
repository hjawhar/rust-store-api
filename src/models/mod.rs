use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
}

pub struct Err {}
impl From<mongodb::error::Error> for Err {
    fn from(_error: mongodb::error::Error) -> Self {
        Err {}
    }
}
