use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Request {
    GetItem(GetItemInput),
    PutItem(PutItemInput),
    ScanItem(ScanItemInput),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Key(pub [u8; 8]);
impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        hex::serialize_upper(self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Key(hex::deserialize(deserializer)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub key: Key,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct GetItemInput {
    pub table_id: Key,
    pub key: Key,
}

#[derive(Debug, Serialize)]
pub struct PutItemInput {
    pub table_id: Key,
    pub item: Item,
}

#[derive(Debug, Serialize)]
pub struct ScanItemInput {
    pub table_id: Key,
    pub start: Option<Key>,
    pub backward: bool,
    pub limit: usize,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    GetItem(GetItemOutput),
    PutItem(PutItemOutput),
    ScanItem(ScanItemOutput),
    Error(Error),
}

#[derive(Debug, Deserialize)]
pub struct GetItemOutput {
    pub item: Option<Item>,
}

#[derive(Debug, Deserialize)]
pub struct PutItemOutput;

#[derive(Debug, Deserialize)]
pub struct ScanItemOutput {
    pub items: Vec<Item>,
}

#[derive(Debug, Error, Deserialize)]
#[serde(tag = "error")]
pub enum Error {
    #[error("deadlock")]
    Deadlock,
    #[error("other error: {message}")]
    Other { message: String },
}
