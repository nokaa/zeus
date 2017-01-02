use ::Backend;
use ::error::*;

use std::path::Path;

pub struct BTree<'a> {
    // len: usize,
    key: &'a [u8],
    value: &'a [u8],
}

impl<'a> BTree<'a> {
    pub fn as_bytes(&self) -> Vec<u8> {
        // TODO(nokaa): The +10 should be + 2 * sizeof(usize)
        let len = self.key.len() + self.value.len() + 10;
        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        // TODO(nokaa): We need to properly convert usize to u8
        bytes.push(self.key.len() as u8);
        bytes.extend_from_slice(&[0, 0, 0, 0]);
        // TODO(nokaa): We need to properly convert usize to u8
        bytes.push(self.value.len() as u8);
        bytes.extend_from_slice(&[0, 0, 0, 0]);
        bytes.extend_from_slice(self.key);
        bytes.extend_from_slice(self.value);

        bytes
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let key_len = bytes[0] as usize;
        let value_len = bytes[6] as usize;
        let key = &bytes[10..key_len + 10];
        let value = &bytes[key_len + 10..value_len];

        BTree {
            key: key,
            value: value,
        }
    }
}

impl<'a> Backend for BTree<'a> {
    fn open(&self, path: &Path) -> Result<()> {
        Ok(())
    }

    fn close(&self) -> Result<()> {
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<()> {
        Ok(())
    }

    fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        Ok(())
    }

    fn delete(&self, key: &[u8]) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> String {
        String::from("B+ Tree")
    }
}
