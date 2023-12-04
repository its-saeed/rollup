use crate::Block;
use anyhow::Result;
use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, FindOptions},
    sync::Client,
};

pub fn update_state(state: &mut [u64], block: &Block) {
    block.iter().for_each(|x| state[x.index as usize] = x.value);
}

pub fn persist_state(state: &[u64]) -> Result<()> {
    let docs = state
        .iter()
        .enumerate()
        .map(|(index, x)| doc! {"index": index.to_string(), "value": x.to_string()})
        .collect::<Vec<_>>();

    let client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    let client = Client::with_options(client_options)?;

    let db = client.database("chainway");
    let collection = db.collection::<Document>("states");
    collection.insert_many(docs, None)?;

    Ok(())
}

pub fn load_state() -> Result<[u64; 256]> {
    let mut state = [0_u64; 256];
    let client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    let client = Client::with_options(client_options)?;

    let db = client.database("chainway");
    let collection = db.collection::<Document>("states");
    let filter = doc! {};
    let find_options = FindOptions::builder().sort(doc! {}).build();

    let mut cursor = collection.find(filter, find_options)?;
    while let Some(value) = cursor.next() {
        let value = value?;
        let index = value
            .get("index")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<usize>()?;
        let value = value
            .get("value")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<u64>()?;
        state[index] = value;
    }

    Ok(state)
}
