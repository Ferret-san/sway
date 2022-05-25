predicate;

use std::result::Result;
use std::b512::B512;
use std::tx::{tx_id, tx_predicate_data};
use std::vm::evm::evm_address::*;
use std::vm::evm::ecr::*;
use std::vm::evm::eip191::eip191_hash;

// NOTE: FOR DEMONSTRATION PURPOSES ONLY
// DO NOT USE IN PRODUCTION

// Replace with a hard coded public key for the owner!
const OWNER_PUB_KEY_HASH = 0xe4eab8f844a8d11b205fd137a1b7ea5ede26f651909505d99cf8b5c0d4c8e9c1;

fn main() -> bool {
    // A more proper predicate would read the signature from the `predicateData`
    let hi = 0xbd0c9b8792876713afa8bff383eebf31c43437823ed761cc3600d0016de5110c;
    let lo = 0x44ac566bd156b4fc71a4a4cb2655d3dd360c695edb17dc3b64d611e122fea23d;
    let signature: B512 = ~B512::from(hi, lo);

    let ethereum_address = ~EvmAddress::from(OWNER_PUB_KEY_HASH);

    let msg_hash = 0x8ddb13a2ab58f413bd3121e1ddc8b83a328f3b830d19a7c471f0be652d23bb0e;
    //let msg_hash = eip191_hash(tx_id());

    let result_address = ec_recover_evm_address(signature, msg_hash);

    let mut result = false;
    if let Result::Ok(address) = result_address {
        result = ethereum_address == address;
    } else {
        result = false;
    };

    result
}
