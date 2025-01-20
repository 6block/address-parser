use anyhow::anyhow;

use super::Token;

pub fn is_valid_iron_address(address: String) -> anyhow::Result<bool> {
    ironfish::PublicAddress::from_hex(&address)
        .map(|_| true)
        .map_err(|e| anyhow!(e.to_string()))
}

#[derive(Debug)]
pub struct Ironfish;

impl Token for Ironfish {
    const NAME: &'static str = "IRON";
}
