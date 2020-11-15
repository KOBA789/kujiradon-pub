use std::{convert::TryInto, io::Write, io::{BufRead, BufReader}, net::TcpStream};

use serde::{Serialize, Deserialize};

use crate::query;

pub const TABLE_USERS: query::Key = query::Key(*b"users   ");
pub const TABLE_TWEETS: query::Key = query::Key(*b"tweets  ");
pub const TABLE_FOLLOWS: query::Key = query::Key(*b"follows ");

pub struct DB {
    conn: BufReader<TcpStream>,
}

impl DB {
    pub fn connect(addr: &str) -> anyhow::Result<Self> {
        let conn = BufReader::new(TcpStream::connect(addr)?);
        Ok(Self { conn })
    }

    pub fn request(&mut self, request: &query::Request) -> anyhow::Result<query::Response> {
        serde_json::to_writer(self.conn.get_ref(), &request)?;
        self.conn.get_ref().write_all(b"\n")?;
        let mut buf = String::new();
        self.conn.read_line(&mut buf)?;
        let response = serde_json::from_str(&buf)?;
        Ok(response)
    }

    pub fn get_item(
        &mut self,
        table_id: query::Key,
        key: query::Key,
    ) -> anyhow::Result<Option<query::Item>> {
        let input = query::GetItemInput { table_id, key };
        let response = self.request(&query::Request::GetItem(input))?;
        match response {
            query::Response::GetItem(output) => Ok(output.item),
            query::Response::Error(err) => Err(anyhow::Error::new(err)),
            _ => Err(anyhow::anyhow!("unexpected response type")),
        }
    }

    pub fn put_item(
        &mut self,
        table_id: query::Key,
        item: query::Item,
    ) -> anyhow::Result<()> {
        todo!();
    }

    pub fn scan_item(
        &mut self,
        table_id: query::Key,
        start: Option<query::Key>,
        backward: bool,
        limit: usize,
    ) -> anyhow::Result<Vec<query::Item>> {
        todo!();
    }
}

pub struct TweetKey {
    pub user_id: u32,
    pub timestamp: u32,
}

impl From<query::Key> for TweetKey {
    fn from(query::Key(bytes): query::Key) -> Self {
        let user_id_bytes: [u8; 4] = bytes[0..4].try_into().unwrap();
        let timestamp_bytes: [u8; 4] = bytes[4..8].try_into().unwrap();
        let user_id = u32::from_be_bytes(user_id_bytes);
        let timestamp = u32::from_be_bytes(timestamp_bytes);
        Self { user_id, timestamp }
    }
}
impl From<TweetKey> for query::Key {
    fn from(TweetKey { user_id, timestamp }: TweetKey) -> Self {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&user_id.to_be_bytes());
        bytes[4..8].copy_from_slice(&timestamp.to_be_bytes());
        Self(bytes)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub text: String,
}

pub struct UserKey {
    pub user_id: u32,
}
impl From<query::Key> for UserKey {
    fn from(query::Key(bytes): query::Key) -> Self {
        let user_id_bytes: [u8; 4] = bytes[0..4].try_into().unwrap();
        let user_id = u32::from_be_bytes(user_id_bytes);
        Self { user_id }
    }
}
impl From<UserKey> for query::Key {
    fn from(UserKey { user_id }: UserKey) -> Self {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&user_id.to_be_bytes());
        Self(bytes)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
}

pub struct FollowKey {
    pub source_id: u32,
    pub destination_id: u32,
}

impl From<query::Key> for FollowKey {
    fn from(query::Key(bytes): query::Key) -> Self {
        let source_id_bytes: [u8; 4] = bytes[0..4].try_into().unwrap();
        let destination_id_bytes: [u8; 4] = bytes[4..8].try_into().unwrap();
        let source_id = u32::from_be_bytes(source_id_bytes);
        let destination_id = u32::from_be_bytes(destination_id_bytes);
        Self { source_id, destination_id }
    }
}
impl From<FollowKey> for query::Key {
    fn from(FollowKey { source_id, destination_id }: FollowKey) -> Self {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&source_id.to_be_bytes());
        bytes[4..8].copy_from_slice(&destination_id.to_be_bytes());
        Self(bytes)
    }
}
