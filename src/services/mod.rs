

use crate::models::User;
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::{bson::doc, error::Error, results::InsertOneResult};



use std::io::prelude::*;

pub struct UserService {
    db: mongodb::Database,
}

impl UserService {
    pub fn new(db: mongodb::Database) -> UserService {
        UserService { db }
    }

    pub async fn create(&self, name: &str) -> Result<InsertOneResult, Error> {

        let collection = self.db.collection::<User>("users");
        let user = User {
            id: None,
            name: String::from(name),
        };
        collection.insert_one(user, None).await
    }

    pub async fn get_one(&self, name: &str) -> mongodb::error::Result<Option<User>> {
        let collection = self.db.collection::<User>("users");
        let filter = doc! {
            name: String::from("Hassan")
        };
        let found = collection.find_one(filter,None).await;
        found
    }

    pub async fn get(&self) -> Result<Vec<User>, Error> {
        let collection = self.db.collection::<User>("users");
        let mut users = collection.find(None, None).await?;
         let v: Vec<_> = users.try_collect().await?;
        Ok(v)
    }
}
