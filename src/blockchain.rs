use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, timestamp: u128, data: String, previous_hash: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, data, previous_hash));
        let result = hasher.finalize();
        let hash = format!("{:x}", result);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
        }
    }

    fn create_genesis_block() -> Block {
        Block::new(0, 0, "Genesis Block".to_string(), "0".to_string())
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            1627504203,
            data,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(1, 1627504203, "Test Block".to_string(), "0".to_string());
        assert_eq!(block.index, 1);
        assert_eq!(block.data, "Test Block");
        assert_eq!(block.previous_hash, "0");
    }

    #[test]
    fn test_blockchain_add_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("Test Block 1".to_string());
        blockchain.add_block("Test Block 2".to_string());

        assert_eq!(blockchain.chain.len(), 3); // Genesis block + 2 added blocks
        assert_eq!(blockchain.chain[1].data, "Test Block 1");
        assert_eq!(blockchain.chain[2].data, "Test Block 2");
    }
}