mod aleo;
mod autonomys;
mod ironfish;
mod qubic;

pub use aleo::*;
pub use autonomys::{is_valid_auto_address, Autonomys};
pub use ironfish::*;
pub use qubic::*;

pub trait Token: 'static + Send + Sync + std::fmt::Debug {
    const NAME: &'static str;
}

#[cfg(test)]
mod tests {
    use crate::{is_valid_aleo_address, is_valid_auto_address, is_valid_iron_address};

    #[test]
    fn aleo() {
        assert!(is_valid_aleo_address(
            "aleo1666y6x0qcxa7syyahys6tzalp3aqppqwj7tdf6purwtlyjkpwsxs3wtxlp".to_string()
        )
        .is_ok());
    }

    #[test]
    fn iron() {
        assert!(is_valid_iron_address(
            "dcda1b0fb6158266c800c4223c433cd9c032d8508e85c0193e08d0dd5ead68ba".to_string()
        )
        .is_ok());
    }

    #[test]
    fn auto() {
        assert!(is_valid_auto_address(
            "sudP7gCmjbUHTCezsHPSzEUfcWkCAe3d7fa3FYRSn3z3tqSHP".to_string()
        )
        .is_ok());
    }
}
