#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(1, 1627504203, "Test Block".to_string(), "0".to_string());
        assert_eq!(block.index, 1);
        assert_eq!(block.data, "Test Block");
    }

    #[test]
    fn test_blockchain_initialization() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1); // Genesis block
        assert_eq!(blockchain.chain[0].data, "Genesis Block");
    }

    #[test]
    fn test_adding_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("Block 1".to_string());

        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.chain[1].data, "Block 1");
    }
}
