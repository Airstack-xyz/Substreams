use num_bigint::BigUint;
use substreams::{log, proto, Hex};

pub fn bigint_from_bytes(value: Vec<u8>) -> BigUint {
    BigUint::from_bytes_be(value.as_slice())
}

pub fn format_address(address: String) -> String {
    format!("0x{}", address)
}
