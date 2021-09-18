use crate::models::User;
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::{bson::doc, error::Error, results::InsertOneResult};

pub struct UserService {
    db: mongodb::Database,
}

impl UserService {
    pub fn new(db: mongodb::Database) -> UserService {
        UserService { db }
    }

    pub async fn create(&self, name: &str) -> Result<InsertOneResult, Error> {
        let user = User {
            name: String::from(name),
        };
        let collection = self.db.collection::<User>("users");
        collection.insert_one(user, None).await
    }

    pub async fn get(&self) -> Result<Vec<User>, Error> {
        let collection = self.db.collection::<User>("users");
        let filter = doc! {};
        let mut users = collection.find(filter, None).await?;
        let mut _users: Vec<User> = Vec::new();
        while let Some(book) = users.try_next().await? {
            println!("title: {}", book.name);
            _users.push(book);
        }

        Ok(_users)
    }
}
