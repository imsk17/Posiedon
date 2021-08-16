use crate::block::Block;
use rocksdb::DB;
pub struct BlockChain {
    pub last_hash: Vec<u8>,
    pub db: DB,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        if let Some(last_hash) = self.db.get(b"lh").expect("failed to get the key lh from db") {
            let new = Block::new(data, last_hash.clone());
            self.db.put(&new.hash, new.to_bytes().expect("failed to serialize block"))
                .expect("Failed to insert block");
            self.db.put(b"lh", new.hash)
                .expect("Failed to change last hash");
            self.last_hash = last_hash;
        } else {
            panic!("No blockchain found, quitting!")
        }
    }
    pub fn new(db: DB) -> Self {
        match db.get("lh") {
            Ok(Some(last_hash)) => {
                println!("Existing blockchain found in the db");
                BlockChain {
                    last_hash,
                    db,
                }
            }
            Ok(None) => {
                println!("No existing blockchain found");
                let genesis = Block::genesis();
                println!("Genesis proved");
                db.put(&genesis.hash, genesis.to_bytes().expect("Failed to serialize block")).expect("Failed to insert block into the database");
                db.put(b"lh", &genesis.hash).expect("Failed to change the lh in the database");
                BlockChain {
                    last_hash: genesis.hash,
                    db,
                }
            }
            Err(e) => {
                panic!("Failed to get last hash from the database {}", e)
            }
        }
    }
}
