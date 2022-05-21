library eip191;

//! A library that calculates a hash for a given message
// The hash for the given message is calculated as
// keccack256("\x19Ethereum Signed Message:\32" + message))
use ::hash::{keccak256_with_offset, sha256};
use ::evm_address::EvmAddress;

// "\x19Ethereum Signed Message:\32" as a padded b256
const DOMAIN_SEPARATOR: b256 = 0x0000000019457468657265756d205369676e6564204d6573736167653a0a3332;

pub fn eip191_hash<T>(message: T) -> b256 {
    // Fuel uses SHA-256 for all hashing since it's standard
    let msg_hash = sha256(message);
    // Offset by 8, since the domain separator is padded
    keccak256_with_offset((domain_separator, msg_hash), 8)
}
