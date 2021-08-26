use crate::block::Block;
use crate::tx::transaction::Transaction;
use crate::utils::db_exists;
use crate::DB_PATH;
use eyre::Result;
use rocksdb::DB;
use std::path::Path;
use std::rc::Rc;
use tracing::info;

pub struct BlockChain {
    pub last_hash: Vec<u8>,
    pub db: Rc<DB>,
}

impl BlockChain {
    pub fn add_block(&mut self, txns: Vec<Transaction>) -> Result<()> {
        match self.db.get(b"lh")? {
            Some(last_hash) => {
                let new = Block::new(txns, last_hash.clone());
                self.db.put(&new.hash, new.to_bytes()?)?;
                self.db.put(b"lh", new.hash)?;
                self.last_hash = last_hash;
                Ok(())
            }
            None => Err(eyre::eyre!(
                "Last Hash not Found, Database is corrupted probably!"
            )),
        }
    }
    pub fn new(address: String) -> Result<Self> {
        if db_exists() {
            info!("Blockchain Already Exists, Shutting Down!");
            std::process::exit(0)
        }
        let path = Path::new(DB_PATH);
        let db = DB::open_default(path)?;
        info!("No existing blockchain found");
        let genesis = Block::genesis(Transaction::coinbase_tx(
            address,
            "First Transaction from Genesis".to_string(),
        ));
        info!("Genesis proved");
        db.put(&genesis.hash, genesis.to_bytes()?)?;
        db.put(b"lh", &genesis.hash)?;
        Ok(BlockChain {
            last_hash: genesis.hash,
            db: Rc::new(db),
        })
    }
    pub fn continue_bc(_address: String) -> Result<Self> {
        if !db_exists() {
            info!("No existing blockchain found, create one!");
            std::process::exit(0)
        }
        let path = Path::new(DB_PATH);
        let db = DB::open_default(path)?;
        if let Some(last_hash) = db.get(b"lh")? {
            Ok(BlockChain {
                last_hash,
                db: Rc::new(db),
            })
        } else {
            Err(eyre::eyre!("Could not find last hash in the database."))
        }
    }
}
