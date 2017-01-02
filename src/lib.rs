#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod backend;
mod btree;
mod error;
// mod lsm;

pub use backend::Backend;
pub use btree::BTree;
pub use error::*;
// pub use lsm::LSM;

use std::path::Path;

pub struct DB<B: Backend> {
    backend: B,
}

impl<B: Backend> DB<B> {
    pub fn open(path: &Path, backend: B) -> Self {
        backend.open(path).unwrap();
        DB { backend: backend }
    }

    pub fn get(&self, key: &[u8]) -> Result<()> {
        self.backend.get(key)
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.backend.put(key, value)
    }

    pub fn delete(&self, key: &[u8]) -> Result<()> {
        self.backend.delete(key)
    }

    pub fn backend_type(&self) -> String {
        self.backend.name()
    }
}

// impl Drop for DB
