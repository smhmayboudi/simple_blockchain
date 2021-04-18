use chrono::prelude::*;
use crate::consensus::proof_of_work::ProofOfWork;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Block {
    pub data: String,
    pub hash: String,
    pub nonce: i64,
    pub prev_block_hash: String,
    pub timestamp: i64,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &str) -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp_nanos();

        let mut block = Block {
            data: data.to_string(),
            hash: Default::default(),
            nonce: Default::default(),
            prev_block_hash: prev_block_hash.to_string(),
            timestamp,
        };

        let pow = ProofOfWork::new(&block);
        let (nonce, hash) = pow.run();

        block.hash = hash;
        block.nonce = nonce;
        block
    }

    pub fn new_genesis_block() -> Self {
        Block::new("Genesis Block", "")
    }
}
