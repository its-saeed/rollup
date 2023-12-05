use crate::lsmtree::{bytes::Bytes, BadProof, KVStore};

use sha2::Sha256;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum Error {
    NotFound,
    BadProof(BadProof),
}

impl From<BadProof> for Error {
    fn from(e: BadProof) -> Self {
        Error::BadProof(e)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Default)]
pub struct InMemoryHashMapStore {
    data: HashMap<Bytes, Bytes>,
    pins: VecDeque<Pin>,
}

impl InMemoryHashMapStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            pins: VecDeque::new(),
        }
    }
}

impl KVStore for InMemoryHashMapStore {
    type Error = Error;
    type Hasher = Sha256;

    fn get(&self, key: &[u8]) -> Result<Option<Bytes>, Self::Error> {
        Ok(self.data.get(key).map(core::clone::Clone::clone))
    }

    fn set(&mut self, key: Bytes, value: Bytes) -> Result<(), Self::Error> {
        self.data.insert(key.clone(), value.clone());
        if let Some(pin) = self.pins.back_mut() {
            let source_value = self.data.get(&key).map(core::clone::Clone::clone);
            pin.changes.push(Change {
                key,
                value: source_value,
            })
        }
        Ok(())
    }

    fn remove(&mut self, key: &[u8]) -> Result<Bytes, Self::Error> {
        let existing_value = self.data.get(key).cloned();
        let result = self.data.remove(key).ok_or(Error::NotFound);
        if result.is_ok() {
            if let Some(pin) = self.pins.back_mut() {
                pin.changes.push(Change {
                    key: Bytes::copy_from_slice(key),
                    value: existing_value,
                })
            }
        }
        result
    }

    fn contains(&self, key: &[u8]) -> Result<bool, Self::Error> {
        Ok(self.data.contains_key(key))
    }

    fn pin(&mut self) {
        if self.pins.len() >= 3 {
            self.pins.pop_front();
        }

        self.pins.push_back(Pin::new());
    }

    fn rollback(&mut self) {
        if let Some(pin) = self.pins.pop_back() {
            for action in pin.changes.iter().rev() {
                match action.value {
                    None => {
                        self.remove(&action.key).unwrap();
                    }
                    Some(ref value) => {
                        self.set(action.key.clone(), value.clone()).unwrap();
                    }
                };
            }
        }
    }

    fn finalize(&self) {
        println!("TODO Finalize");
    }
}

#[derive(Debug, Clone)]
struct Pin {
    pub changes: Vec<Change>,
}

impl Pin {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Change {
    pub key: Bytes,
    pub value: Option<Bytes>,
}
