extern crate serde;

use chain::chain::BlockChain;
use color_eyre::Result;

mod block;
mod chain;
mod pow;
mod tx;
mod utils;
mod wallet;

const DB_PATH: &str = "./db/chain";

fn main() -> Result<()> {
    // Setup logging
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    // Create a Blockchain
    let block_chain = BlockChain::new("".parse()?)?;
    block_chain.into_iter().for_each(|b| {
        println!("{}", b);
    });
    Ok(())
}
