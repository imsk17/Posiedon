use crate::block::Block;
use rocksdb::DB;
use crate::chain::chain::BlockChain;
use std::rc::Rc;

pub struct BlockChainIterator {
    pub db: Rc<DB>,
    pub current_hash: Vec<u8>,
}

impl Iterator for BlockChainIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        match self.db.get(&self.current_hash).unwrap() {
            None => None,
            Some(enc) => {
                let block = Block::from_bytes(enc).unwrap();
                self.current_hash = block.prev_hash.clone();
                Some(block)
            }
        }
    }
}

impl IntoIterator for &BlockChain {
    type Item = Block;
    type IntoIter = BlockChainIterator;
    //? Idk why Self::IntoIter is not giving intellisense. For the time being, we will be using the raw type instead of the associated type.
    fn into_iter(self) -> BlockChainIterator {
        BlockChainIterator {
            db: Rc::clone(&self.db),
            current_hash: self.last_hash.to_owned(),
        }
    }
}
