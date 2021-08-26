extern crate serde;

use color_eyre::Result;
use chain::chain::BlockChain;
use crate::tx::transaction::Transaction;

mod block;
mod pow;
mod chain;
mod tx;
mod utils;

const DB_PATH: &str = "./db/";

fn main() -> Result<()> {
    // Setup logging
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    // Create a Blockchain
    let block_chain = BlockChain::continue_bc("".parse()?)?;
    block_chain.into_iter().for_each(|b| {
        println!("{}", b);
    });
    Ok(())
}
