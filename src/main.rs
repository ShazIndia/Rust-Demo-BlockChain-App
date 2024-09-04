mod blockchain;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());

    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }
}
