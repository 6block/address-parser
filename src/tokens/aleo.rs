use std::str::FromStr;

use snarkvm::prelude::{Address, MainnetV0};

use super::Token;

pub fn is_valid_aleo_address(address: String) -> anyhow::Result<bool> {
    Address::<MainnetV0>::from_str(&address).map(|_| true)
}

#[derive(Debug)]
pub struct Aleo;

impl Token for Aleo {
    const NAME: &'static str = "ALEO";
}
