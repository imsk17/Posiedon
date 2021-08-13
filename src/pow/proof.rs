use crate::block::Block;
use num_bigint::{BigInt, Sign};
use sha2::Digest;
use std::ops::ShlAssign;
use std::cmp::Ordering;

const DIFFICULTY: i64 = 18;

pub struct ProofOfWork {
    pub block: Block,
    pub target: BigInt,
}

impl ProofOfWork {
    pub fn new(b: Block) -> ProofOfWork {
        let mut target: BigInt = 1i64.into();
        target.shl_assign((256 - DIFFICULTY) as usize);
        ProofOfWork {
            block: b,
            target,
        }
    }
    pub fn init_data(&self, nonce: i64) -> Vec<u8> {
        let data: Vec<u8> = [
            &*self.block.prev_hash,
            &*self.block.data,
            &nonce.to_be_bytes().to_vec(),
            &DIFFICULTY.to_be_bytes().to_vec()
        ].join("".as_bytes());
        return data;
    }

    pub fn run(&self) -> (i64, Vec<u8>) {
        let mut hash: Vec<u8> = vec![];
        let mut nonce = 0;
        while nonce < i64::MAX {
            let data = self.init_data(nonce);
            hash = sha2::Sha256::digest(&data).to_vec();
            print!("\r{}", hex::ToHex::encode_hex::<String>(&hash));
            let int_hash : BigInt = BigInt::from_bytes_be(Sign::Plus,&hash);
            if int_hash.cmp(&self.target) == Ordering::Less {
                break;
            } else {
                nonce += 1;
            }
        };
        println!();
        return (nonce, hash.clone());
    }
    pub fn validate(&self) -> bool {
        let data = self.init_data(self.block.nonce);
        let hash = sha2::Sha256::digest(&data);
        let int_hash : BigInt = BigInt::from_bytes_be(Sign::Plus,&hash);
        int_hash.cmp(&self.target) == Ordering::Less
    }
}