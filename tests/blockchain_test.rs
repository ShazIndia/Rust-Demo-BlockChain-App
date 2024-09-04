// Import the blockchain module from the main crate
use crate::blockchain::{Block, Blockchain};

#[cfg(test)]
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
