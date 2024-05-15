use std::sync::Mutex;

use crate::block::Block;



#[derive(Debug)]
pub struct Chain {
    pub blocks: Mutex<Box<Vec<Block>>>,
}

impl Chain {
    pub fn new() -> Self {
        let data = "GenesisBlock".into();
        let previous_block_hash: Vec<u8> = Vec::with_capacity(0);

        let genesis_block = Block::new(data, previous_block_hash);
        let blocks = Mutex::new(Box::new(vec![genesis_block]));

        Self { blocks }
    }

    fn get_previous_block_hash(&self) -> Vec<u8> {
        self.blocks.lock().unwrap().last().unwrap().my_block_hash.clone()
    }

    fn add_block(&self, block: Block) {
        self.blocks.lock().unwrap().push(block);
    }

    pub fn add_block_with_data(&self, data: String) -> bool {
        let previous_block_hash = self.get_previous_block_hash();
        let block = Block::new(data, previous_block_hash);

        self.add_block(block);

        true
    }
}