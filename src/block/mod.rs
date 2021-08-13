pub mod chain;

use std::fmt::{Formatter, Display};
use sha2::Digest;

pub struct Block {
    pub hash: Vec<u8>,
    pub data: Vec<u8>,
    pub prev_hash: Vec<u8>,
}

impl Block {
    pub fn derive_hash(&mut self) {
        let data = vec![&*self.data, &*self.prev_hash].join("".as_bytes());
        let hash = sha2::Sha256::digest(&data);
        self.hash = hash.to_vec();
    }
    pub fn new(data: String, prev_hash: Vec<u8>) -> Self {
        let mut block = Block {
            hash: Vec::new(),
            data: data.into_bytes(),
            prev_hash,
        };
        block.derive_hash();
        block
    }
    pub fn genesis() -> Self {
        Block::new("Genesis".to_string(), b"".to_vec())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Data In Block : {}\nHash : {}\nPrevious Hash : {}",
               String::from_utf8(self.data.clone()).unwrap(),
               hex::ToHex::encode_hex::<String>(&self.hash),
               hex::ToHex::encode_hex::<String>(&self.prev_hash),
        )
    }
}
