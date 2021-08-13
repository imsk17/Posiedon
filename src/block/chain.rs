use crate::block::Block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let new = Block::new(data, prev_block.hash.clone());
        self.blocks.push(new)
    }
    pub fn new() -> Self {
        return BlockChain {
            blocks: vec![Block::genesis()]
        };
    }
}