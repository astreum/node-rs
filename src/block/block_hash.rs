use fides::hash::blake_3;
use fides::merkle_tree;
use super::Block;

impl Block {

    pub fn update_block_hash(&mut self) {

        self.block_hash = self.block_hash()
        
    }

    pub fn block_hash(&self) -> [u8; 32] {

        merkle_tree::root(blake_3, &[&self.details_hash, &self.signature])

    }

}
