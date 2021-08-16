extern crate serde;

use rocksdb::{DB};
use std::path::Path;
use chain::chain::BlockChain;
mod block;
mod pow;
mod chain;

const DB_PATH: &str = "./db/";

fn main() {
    let path = Path::new(DB_PATH);
    let db = DB::open_default(path)
        .expect("Unable to open the database, Shutting Down!");

    let block_chain = BlockChain::new(db);
    block_chain.into_iter().for_each(|b| {
        println!("{}",b);
    });
}
