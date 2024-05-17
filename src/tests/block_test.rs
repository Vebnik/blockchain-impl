
#[test]
pub fn test_create_genesis_block() {
    use crate::block::Block;
    use crate::transactions::Transactions;

    let data = Transactions::new("0x0".into(), "0x1".into(), 10u128);
    let previous_block_hash: Vec<u8> = Vec::with_capacity(0);

    let block = Block::new(data, previous_block_hash);

    dbg!(&block);

    assert_eq!(String::from_utf8(block.data).unwrap(), "GenesisBlock")
}