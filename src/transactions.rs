
#[derive(Debug)]
pub struct Transactions {
    pub from: String,
    pub to: String,
    pub amount: u128,
}

impl Transactions {
    pub fn new(from: String, to: String, amount: u128) -> Self {
        Self { from, to, amount }
    }
}