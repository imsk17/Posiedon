use eyre::Result;
use rocksdb::{IteratorMode, DB};
use serde::{Deserialize, Serialize};

use crate::wallet::wallets::Wallet;

#[derive(Serialize, Deserialize)]
pub(crate) struct WalletDB {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub struct WalletStore {
    pub database: DB,
}

impl WalletStore {
    pub fn insert_wallet(&self, wallet: &Wallet) -> Result<()> {
        self.database.put(wallet.address(), wallet.to_bytes())?;
        Ok(())
    }

    pub fn get_wallet(&self, address: &str) -> Result<Wallet> {
        match self.database.get(address)? {
            None => {
                panic!("No wallet with this address found")
            }
            Some(b) => Ok(Wallet::from_bytes(&b)),
        }
    }
    pub fn get_all_wallets(&self) -> Vec<Wallet> {
        self.database
            .iterator(IteratorMode::Start)
            .map(|v| Wallet::from_bytes(&v.1))
            .collect()
    }
}
