use crate::{block::Block, hash::sha256};
use num_bigint::BigInt;
use num_traits::{Num, One};

const TARGET_BITS: u32 = 2;
const MAX_NONCE: u64 = std::u64::MAX;

pub struct ProofOfWork<'a> {
    block: &'a Block,
    target: BigInt,
}

impl<'a> ProofOfWork<'a> {
    pub fn new(block: &'a Block) -> Self {
        let target = BigInt::one() << (256 - TARGET_BITS) as usize;
        ProofOfWork { block, target }
    }
}

impl<'a> ProofOfWork<'a> {
    fn prepare_data(&self, nonce: u64) -> Vec<u8> {
        [
            self.block.prev_block_hash.as_bytes(),
            self.block.data.as_bytes(),
            format!("{:#x}", self.block.timestamp).as_bytes(),
            format!("{:#x}", u64::from(TARGET_BITS)).as_bytes(),
            format!("{:#x}", nonce).as_bytes(),
        ]
        .concat()
    }
}

impl<'a> ProofOfWork<'a> {
    pub fn run(&self) -> (u64, String) {
        let mut hash = "".to_string();
        let mut nonce = 0u64;

        println!("Mining the block containing {}", self.block.data);
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = sha256(&data);
            print!("\r{}", hash);
            let hash_int = BigInt::from_str_radix(&hash, 16).expect("Should be a valid number");
            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        println!("\n");

        (nonce, hash)
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let hash = sha256(&data);
        let hash_int = BigInt::from_str_radix(&hash, 16).expect("Should be a valid number");
        hash_int < self.target
    }
}
