extern crate serde;

use color_eyre::Result;
use rocksdb::{DB};
use std::path::Path;
use chain::chain::BlockChain;

mod block;
mod pow;
mod chain;

const DB_PATH: &str = "./db/";

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    let path = Path::new(DB_PATH);
    let db = DB::open_default(path)?;

    let block_chain = BlockChain::new(db)?;
    block_chain.into_iter().for_each(|b| {
        println!("{}", b);
    });
    Ok(())
}
