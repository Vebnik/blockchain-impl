
#[test]
pub fn test_create_chain() {
    use crate::chain::Chain;

    let blockchain = Chain::new();

    for i in 0..1000 {
        let data = format!("block_{i}");
        blockchain.add_block_with_data(data);
    }

    dbg!(&blockchain);

    let genesis_block_data = blockchain.blocks.lock().unwrap().first().unwrap().data.clone();
    let blocks_count = blockchain.blocks.lock().unwrap().len();

    assert_eq!(String::from_utf8(genesis_block_data).unwrap(), "GenesisBlock");
    assert_eq!(blocks_count, 1001usize);
}