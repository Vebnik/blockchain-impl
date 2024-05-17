use std::time::{SystemTime, UNIX_EPOCH};

use crate::transactions::Transactions;

#[derive(Debug, Clone)]
pub struct Header(pub String);

#[derive(Debug, Clone)]
pub struct Body(pub String);

#[derive(Debug, Clone)]
pub struct Block {
    pub timestamp: Vec<u8>,
    pub data: Vec<u8>,

    pub previous_block_hash: Vec<u8>,
    pub my_block_hash: Vec<u8>,
}

impl Block {
    pub fn new(data: Transactions, previous_block_hash: Vec<u8>) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Error on get SystemTime UNIX_EPOCH")
            .as_millis()
            .to_be_bytes()
            .to_vec();

        // let data = data.;

        let header = vec![
            timestamp.clone(),
            data.clone(),
            previous_block_hash.clone(),
        ].concat();

        let my_block_hash = sha256::digest(header).as_bytes().to_vec();

        Self { timestamp, data, previous_block_hash, my_block_hash }
    }
}