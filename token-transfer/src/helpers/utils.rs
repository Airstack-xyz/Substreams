use num_bigint::BigUint;

pub fn bigint_from_bytes(value: Vec<u8>) -> BigUint {
    BigUint::from_bytes_be(value.as_slice())
}