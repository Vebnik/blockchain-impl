
#[test]
pub fn test_create_transactions() {
    use crate::transactions::Transactions;

    let tx = Transactions::new(
        "0x0".into(), 
        "0x1".into(), 
        10u128
    );

    assert_eq!(dbg!(tx).to, "0x1")
}