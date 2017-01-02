use ::error::*;

use std::path::Path;

pub trait Backend {
    fn open(&self, path: &Path) -> Result<()>;
    fn close(&self) -> Result<()>;
    fn get(&self, key: &[u8]) -> Result<()>;
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&self, key: &[u8]) -> Result<()>;
    fn name(&self) -> String;
}
