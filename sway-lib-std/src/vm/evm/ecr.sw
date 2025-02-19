library ecr;

dep evm_address;

use ::b512::B512;
use ::context::registers::error;
use ::ecr::{EcRecoverError, ec_recover};
use ::hash::keccak256;
use ::result::*;
use evm_address::*;

/// Recover the ethereum address derived from the private key used to sign a message.
/// Returns a `Result` to let the caller choose an error handling strategy.
pub fn ec_recover_address(signature: B512, msg_hash: b256) -> Result<EvmAddress, EcRecoverError> {
    let pub_key_result = ec_recover(signature, msg_hash);

    if let Result::Err(e) = pub_key_result {
        // propagate the error if it exists
        Result::Err(e)
    } else {
        let pub_key = pub_key_result.unwrap();

        // Note that Ethereum addresses are derived from the Keccak256 hash of the pubkey (not sha256)
        let address = keccak256(((pub_key.bytes)[0], (pub_key.bytes)[1]));

        Result::Ok(~EvmAddress::from(address))
    }
}
