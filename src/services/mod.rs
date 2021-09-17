use crate::db::insert_one;
use mongodb::bson::doc;
use mongodb::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
}

pub struct UserService {
    db: mongodb::Database,
}

impl UserService {
    // pub fn new(collection: Collection<User>) -> UserService {
    //     UserService { collection }
    // }

    pub async fn create(self, name: String) -> Result<(), Error> {
        let user = User { name };
        insert_one(self.db, doc! {name}).await;
        Ok(())
    }

    // pub fn get(&self) -> Result<Option<OrderedDocument>, Error> {
    //     self.collection.find_one(doc! {}, None)
    // }
}
