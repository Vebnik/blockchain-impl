use crate::{chain::Chain, transactions::Transactions};

pub mod block;
pub mod chain;
pub mod transactions;
pub mod tests;

fn main() {
    let blockchain = Chain::new();

    for i in 0..100000000 {
        let data = Transactions::new(
            "0x0".into(), format!("0x{i}"), 10u128
        );
        blockchain.add(data);
    }

    let genesis_block_data = blockchain.blocks.lock().unwrap().first().unwrap().data.clone();
    let blocks_count = blockchain.blocks.lock().unwrap().len();

    println!(
        "Genesis block data: {} | Blocks: {blocks_count}",
        String::from_utf8(genesis_block_data).unwrap()
    );
}
