library eip712;

//! A simple library for EIP-712 related operations
// The library includes a function that calculates the DOMAIN_SEPARATOR
// as well as a function that encodes a hased struct as an EIP-712 message
//
// It's important to note that the hashing of the name, version, and struct
// is left to the user to implement

use ::hash::{keccak256, keccak256_with_offset, sha256};
use ::evm_address::*;

// EIP712DOMAIN_TYPEHASH in bytes32
const EIP712DOMAIN_TYPE_HASH: b256 = keccak256_with_offset(0x0454950373132446f6d61696e28737472696e67206e616d652c737472696e672, 1);

pub struct EIP712Domain<T> {
    name: T,
    version: u8,
    chainId: u8,
    verifyingContract: b256,
}

// computes an EIP-712 domain separator given:
// 1. name
// 2. version
// 3. chainId
// 4. verifyingContract
pub fn compute_domain_separator(eip712_domain: EIP712Domain) -> b256 {
    // Hash the EIP-712 domain struct
    let hash = sha256(eip712_domain);
    // Offset by 1 since the domain typeHash is padded by 1
    keccak256_with_offset((EIP712DOMAIN_TYPE_HASH, hash), 1)
}

// return the hash of the fully encoded EIP-712 message for a given domain and struct hash
pub fn hash_typed_data(eip712_domain: EIP712Domain, struct_hash: b256) -> b256 {
    let bytes = "\x19\x01";
    let domain_separator = compute_domain_separator(EIP712Domain);
    keccak256((bytes, domain_separator, struct_hash))
}
