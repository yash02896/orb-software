use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, WrapErr};
use ssri::Hash;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entry {
    pub key: Key,
    pub hash: Hash,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Key(String);

impl Key {
    pub fn new(name: &str, version: &str) -> Self {
        Self(format!("{name}-{version}"))
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    pub async fn insert(&self, key: &Key, data: impl AsRef<[u8]>) -> Result<Hash> {
        let mut integrity = cacache::write(&self.path, &key, data)
            .await
            .wrap_err("failed to insert into store")?;
        Ok(integrity.hashes.pop().unwrap())
    }

    pub async fn get_path(&self, key: &Key) -> PathBuf {
        todo!()
    }
}
