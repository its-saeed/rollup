use crate::{Block, InMemoryHashMapStore, SparseMerkleTree};
use anyhow::Result;
use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, FindOptions},
    sync::Client,
};

pub struct State(SparseMerkleTree<InMemoryHashMapStore>);

impl State {
    pub fn new() -> Self {
        Self(SparseMerkleTree::<InMemoryHashMapStore>::new_with_stores(
            InMemoryHashMapStore::new(),
            InMemoryHashMapStore::new(),
        ))
    }

    pub fn update(&mut self, block: &Block) {
        block.iter().for_each(|x| {
            self.0
                .update(
                    &x.index.to_be_bytes(),
                    bytes::Bytes::copy_from_slice(&x.value.to_be_bytes()),
                )
                .unwrap()
        });
    }

    pub fn root_hash(&self) -> String {
        hex::encode(self.0.root())
    }

    pub fn pin(&mut self) {
        self.0.pin();
    }

    pub fn rollback(&mut self, n: usize) {
        for _ in 0..n {
            self.0.rollback();
        }
    }

    pub fn persist(&self) -> Result<()> {
        todo!()
        // let docs = self
        //     .0
        //     .iter()
        //     .enumerate()
        //     .map(|(index, x)| doc! {"index": index.to_string(), "value": x.to_string()})
        //     .collect::<Vec<_>>();

        // let client_options = ClientOptions::parse("mongodb://localhost:27017")?;

        // let client = Client::with_options(client_options)?;

        // let db = client.database("chainway");
        // let collection = db.collection::<Document>("states");
        // collection.insert_many(docs, None)?;

        // Ok(())
    }

    pub fn load() -> Result<Self> {
        todo!()
        // let mut state = [0_u64; 256];
        // let client_options = ClientOptions::parse("mongodb://localhost:27017")?;

        // let client = Client::with_options(client_options)?;

        // let db = client.database("chainway");
        // let collection = db.collection::<Document>("states");
        // let filter = doc! {};
        // let find_options = FindOptions::builder().sort(doc! {}).build();

        // let cursor = collection.find(filter, find_options)?;
        // for value in cursor {
        //     let value = value?;
        //     let index = value
        //         .get("index")
        //         .unwrap()
        //         .as_str()
        //         .unwrap()
        //         .parse::<usize>()?;
        //     let value = value
        //         .get("value")
        //         .unwrap()
        //         .as_str()
        //         .unwrap()
        //         .parse::<u64>()?;
        //     state[index] = value;
        // }

        // Ok(Self(state))
    }
}
