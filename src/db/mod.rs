use crate::models::Err;
use crate::models::User;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::{options::ClientOptions, Client, Cursor};
// use mongodb::error::Error;

pub async fn connect_to_mongodb(name: &str) -> mongodb::error::Result<mongodb::Database> {
    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    // client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("DB name: {}", db_name);
    }

    let db = client.database(name);
    Ok(db)
}

pub async fn get_one(
    db: mongodb::Database,
    filter: mongodb::bson::Document,
) -> Result<Option<User>, Err> {
    let typed_collection = db.collection::<User>("users");
    let user = typed_collection.find_one(filter, None).await?;
    Ok(user)
}

pub async fn get_many(
    db: mongodb::Database,
    filter: mongodb::bson::Document,
) -> Result<Cursor<User>, Err> {
    let typed_collection = db.collection::<User>("users");
    let users = typed_collection.find(filter, None).await?;
    Ok(users)
}

pub async fn insert_one(db: mongodb::Database, entity: User) -> Result<InsertOneResult, Error> {
    let typed_collection = db.collection::<User>("users");
    typed_collection.insert_one(entity, None).await
}

pub async fn insert_many(db: mongodb::Database, entities: Vec<User>) -> Result<(), Err> {
    let typed_collection = db.collection::<User>("users");
    typed_collection.insert_many(entities, None).await?;
    Ok(())
}
