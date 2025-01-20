use anyhow::anyhow;
use base58::FromBase58;
use blake2::digest::typenum::U64;
use blake2::digest::FixedOutput;
use blake2::{Blake2b, Digest};
use derive_more::derive::From;
use ss58_registry::Ss58AddressFormat;
use thiserror::Error;

use super::Token;

const PREFIX: &[u8] = b"SS58PRE";
const CHECKSUM_LEN: usize = 2;

/// An error type for SS58 decoding.
#[derive(Debug, Error)]
pub enum Ss58ParsingError {
    /// Base 58 requirement is violated
    #[error("Base 58 requirement is violated")]
    BadBase58,
    /// Length is bad
    #[error("Length is bad")]
    BadLength,
    /// Invalid SS58 prefix byte
    #[error("Invalid SS58 prefix byte")]
    InvalidPrefix,
    /// Disallowed SS58 Address Format for this datatype
    #[error("Disallowed SS58 Address Format for this datatype")]
    FormatNotAllowed,
    /// Invalid checksum
    #[error("Invalid checksum")]
    InvalidChecksum,
}

/// Some if the string is a properly encoded SS58Check address.
pub fn parse_ss58_reward_address(s: &str) -> Result<PublicKey, Ss58ParsingError> {
    let data = s.from_base58().map_err(|_| Ss58ParsingError::BadBase58)?;
    if data.len() < 2 {
        return Err(Ss58ParsingError::BadLength);
    }
    let (prefix_len, ident) = match data[0] {
        0..=63 => (1, data[0] as u16),
        64..=127 => {
            // weird bit manipulation owing to the combination of LE encoding and missing two
            // bits from the left.
            // d[0] d[1] are: 01aaaaaa bbcccccc
            // they make the LE-encoded 16-bit value: aaaaaabb 00cccccc
            // so the lower byte is formed of aaaaaabb and the higher byte is 00cccccc
            let lower = (data[0] << 2) | (data[1] >> 6);
            let upper = data[1] & 0b00111111;
            (2, (lower as u16) | ((upper as u16) << 8))
        }
        _ => return Err(Ss58ParsingError::InvalidPrefix),
    };
    if data.len() != prefix_len + PublicKey::SIZE + CHECKSUM_LEN {
        return Err(Ss58ParsingError::BadLength);
    }
    let format: Ss58AddressFormat = ident.into();
    if format.is_reserved() {
        return Err(Ss58ParsingError::FormatNotAllowed);
    }

    let hash = ss58hash(&data[0..PublicKey::SIZE + prefix_len]);
    let checksum = &hash[0..CHECKSUM_LEN];
    if data[PublicKey::SIZE + prefix_len..PublicKey::SIZE + prefix_len + CHECKSUM_LEN] != *checksum
    {
        // Invalid checksum.
        return Err(Ss58ParsingError::InvalidChecksum);
    }

    let bytes: [u8; PublicKey::SIZE] = data[prefix_len..][..PublicKey::SIZE]
        .try_into()
        .map_err(|_| Ss58ParsingError::BadLength)?;

    Ok(PublicKey::from(bytes))
}

fn ss58hash(data: &[u8]) -> [u8; 64] {
    let mut state = Blake2b::<U64>::new();
    state.update(PREFIX);
    state.update(data);
    state.finalize_fixed().into()
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, From)]
pub struct PublicKey([u8; PublicKey::SIZE]);

impl AsRef<[u8]> for PublicKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub struct Blake3Hash([u8; Blake3Hash::SIZE]);

impl Blake3Hash {
    /// Size of BLAKE3 hash output (in bytes).
    pub const SIZE: usize = 32;
}

impl AsRef<[u8]> for Blake3Hash {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Blake3Hash {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<&[u8; Self::SIZE]> for Blake3Hash {
    #[inline]
    fn from(value: &[u8; Self::SIZE]) -> Self {
        Self(*value)
    }
}

impl PublicKey {
    /// Public key size in bytes
    pub const SIZE: usize = 32;
}

pub fn is_valid_auto_address(address: String) -> anyhow::Result<bool> {
    parse_ss58_reward_address(&address)
        .map(|_| true)
        .map_err(|e| anyhow!(e.to_string()))
}

#[derive(Debug)]
pub struct Autonomys;

impl Token for Autonomys {
    const NAME: &'static str = "AI3";
}
