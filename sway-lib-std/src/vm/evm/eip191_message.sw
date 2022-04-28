library eip191_message;

//! A library that calculates a hash for a given message
// The hash for the given message is calculated as
// keccack256("\x19Ethereum Signed Message:\n" + len(message) + message))
// or alternatively
// keccack256("\x19Ethereum Signed Message:\32" + message))
use ::hash::{HashMethod, hash_pair, hash_value};
use ::evm_address::EvmAddress;

pub fn text_hash(message: b256) -> b256 {
    // "\x19Ethereum Signed Message:\32" as a padded b256
    let prefix: b256 = 0x0000000019457468657265756d205369676e6564204d6573736167653a0a3332;
}
