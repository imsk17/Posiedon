mod error_kind;

use std::fmt::{Formatter, Display};
use crate::pow::proof::ProofOfWork;
use serde::{Serialize, Deserialize};
use crate::block::error_kind::{BlockSerErrorKind};
use crate::tx::transaction::Transaction;
use sha2::Digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub prev_hash: Vec<u8>,
    pub nonce: i64,
}

impl Block {
    pub fn new(txs: Vec<Transaction>, prev_hash: Vec<u8>) -> Self {
        let mut block = Block {
            hash: Vec::new(),
            transactions: txs,
            prev_hash,
            nonce: 0,
        };
        let pow = ProofOfWork::new(block.clone());
        let (nonce, hash) = pow.run();
        block.hash = hash;
        block.nonce = nonce;
        block
    }
    pub fn genesis(coinbase: Transaction) -> Self {
        Block::new(vec![coinbase], b"".to_vec())
    }


    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut tx_hashes = vec![];
        self.transactions.iter().for_each(|t| {
            tx_hashes.push(t.id.clone())
        });
        let d = tx_hashes.join(vec![].as_slice());
        let hash = sha2::Sha256::digest(&d);
        hash.to_vec()
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
               "Txns In Block : [{:?}]\nHash : {}\nPrevious Hash : {}\nPow : {}\nNonce: {}",
               self.transactions,
               hex::ToHex::encode_hex::<String>(&self.hash),
               hex::ToHex::encode_hex::<String>(&self.prev_hash),
               ProofOfWork::new(self.clone()).validate(),
               self.nonce
        )
    }
}
