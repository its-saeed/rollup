use anyhow::Result;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOneOptions},
    sync::Client,
};

use crate::Storage;

pub fn persist(storage: &Storage) -> Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    let client = Client::with_options(client_options)?;

    let db = client.database("chainway");
    let collection = db.collection::<Storage>("storage");
    collection.insert_one(storage, None)?;

    Ok(())
}

pub fn load() -> Result<Storage> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    let client = Client::with_options(client_options)?;

    let db = client.database("chainway");
    let collection = db.collection::<Storage>("storage");
    let filter = doc! {};
    let find_options = FindOneOptions::builder().sort(doc! {}).build();

    let cursor = collection.find_one(filter, find_options)?;
    Ok(cursor.unwrap_or_default())
}
