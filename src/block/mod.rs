mod error_kind;

use std::fmt::{Formatter, Display};
use crate::pow::proof::ProofOfWork;
use serde::{Serialize, Deserialize};
use crate::block::error_kind::{BlockSerErrorKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
    pub prev_hash: Vec<u8>,
    pub nonce: i64,
}

impl Block {
    pub fn new(data: String, prev_hash: Vec<u8>) -> Self {
        let mut block = Block {
            hash: Vec::new(),
            data: data.into_bytes(),
            prev_hash,
            nonce: 0,
        };
        let pow = ProofOfWork::new(block.clone());
        let (nonce, hash) = pow.run();
        block.hash = hash;
        block.nonce = nonce;
        block
    }
    pub fn genesis() -> Self {
        Block::new("Genesis".to_string(), b"".to_vec())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, BlockSerErrorKind> {
        bincode::serialize(&self).map_err(|e| BlockSerErrorKind::FailedToSerializeErrorBlock(e.to_string()))
    }

    pub fn from_bytes(value: Vec<u8>) -> Result<Self, BlockSerErrorKind> {
        bincode::deserialize(&value).map_err(|e| BlockSerErrorKind::FailedToDeserializeErrorBlock(e.to_string()))
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Data In Block : {}\nHash : {}\nPrevious Hash : {}\nPow : {}\nNonce: {}",
               String::from_utf8(self.data.clone()).unwrap(),
               hex::ToHex::encode_hex::<String>(&self.hash),
               hex::ToHex::encode_hex::<String>(&self.prev_hash),
               ProofOfWork::new(self.clone()).validate(),
               self.nonce
        )
    }
}
