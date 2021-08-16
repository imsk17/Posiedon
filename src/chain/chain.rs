use crate::block::Block;
use rocksdb::DB;
use tracing::info;
use eyre::Result;

pub struct BlockChain {
    pub last_hash: Vec<u8>,
    pub db: DB,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) -> Result<()> {
        match self.db.get(b"lh")? {
            Some(last_hash) => {
                let new = Block::new(data, last_hash.clone());
                self.db.put(&new.hash, new.to_bytes()?)?;
                self.db.put(b"lh", new.hash)?;
                self.last_hash = last_hash;
                Ok(())
            }
            None => {
                Err(eyre::eyre!("Last Hash not Found, Database is corrupted probably!"))
            }
        }
    }
    pub fn new(db: DB) -> Result<Self> {
        match db.get("lh")? {
            Some(last_hash) => {
                info!("Existing blockchain found in the db");
                Ok(BlockChain {
                    last_hash,
                    db,
                })
            }
            None => {
                info!("No existing blockchain found");
                let genesis = Block::genesis();
                info!("Genesis proved");
                db.put(&genesis.hash, genesis.to_bytes()?)?;
                db.put(b"lh", &genesis.hash)?;
                Ok(BlockChain {
                    last_hash: genesis.hash,
                    db,
                })
            }
        }
    }
}
