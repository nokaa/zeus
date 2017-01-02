use ::Backend;

use std::path::Path;

pub struct LSM;

impl Backend for LSM {
    fn open(&self, path: &Path) -> Self {
        LSM
    }

    fn get(&self, key: &[u8]) {}

    fn put(&self, key: &[u8], value: &[u8]) {}

    fn delete(&self, key: &[u8]) {}
}
