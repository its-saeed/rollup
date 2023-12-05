use crate::{create_merkle_tree, Block};
use anyhow::Result;
use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, FindOptions},
    sync::Client,
};
use rs_merkle::{algorithms::Sha256, MerkleTree};

pub struct State([u64; 256]);

impl State {
    pub fn new() -> Self {
        Self([0_u64; 256])
    }

    pub fn to_merkle_tree(&self) -> MerkleTree<Sha256> {
        create_merkle_tree(&self.0)
    }

    pub fn update(&mut self, block: &Block) {
        block
            .iter()
            .for_each(|x| self.0[x.index as usize] = x.value);
    }

    pub fn persist(&self) -> Result<()> {
        let docs = self
            .0
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

    pub fn load() -> Result<Self> {
        let mut state = [0_u64; 256];
        let client_options = ClientOptions::parse("mongodb://localhost:27017")?;

        let client = Client::with_options(client_options)?;

        let db = client.database("chainway");
        let collection = db.collection::<Document>("states");
        let filter = doc! {};
        let find_options = FindOptions::builder().sort(doc! {}).build();

        let cursor = collection.find(filter, find_options)?;
        for value in cursor {
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

        Ok(Self(state))
    }
}
