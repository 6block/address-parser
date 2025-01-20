use std::str::FromStr;

use anyhow::anyhow;
use qubic_types::QubicId;

use super::Token;

#[derive(Debug)]
pub struct Qubic;

impl Token for Qubic {
    const NAME: &'static str = "QUBIC";
}

pub fn is_valid_qubic_address(address: String) -> anyhow::Result<bool> {
    QubicId::from_str(&address)
        .map(|_| true)
        .map_err(|e| anyhow!(e.to_string()))
}
