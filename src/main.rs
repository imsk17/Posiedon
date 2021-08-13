mod block;

fn main() {
    let mut block_chain = block::chain::BlockChain::new();
    block_chain.add_block("First Block after Genesis".to_string());
    block_chain.add_block("Second Block after Genesis".to_string());
    block_chain.add_block("Third Block after Genesis".to_string());

    block_chain.blocks.iter().for_each(|b| {
        println!("{}", b);
    })
}
