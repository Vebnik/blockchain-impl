use crate::chain::Chain;

pub mod block;
pub mod chain;
pub mod tests;

fn main() {
    let blockchain = Chain::new();

    for i in 0..1000000 {
        let data = format!("block_{i}");
        blockchain.add_block_with_data(data);
    }

    let genesis_block_data = blockchain.blocks.lock().unwrap().first().unwrap().data.clone();
    let blocks_count = blockchain.blocks.lock().unwrap().len();

    println!(
        "Genesis block data: {} | Blocks: {blocks_count}",
        String::from_utf8(genesis_block_data).unwrap()
    );
}
