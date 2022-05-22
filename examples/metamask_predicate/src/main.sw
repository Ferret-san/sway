predicate;

use std::result::Result;
use std::b512::B512;
use std::tx::tx_id;
use std::vm::evm::evm_address::*;
use std::vm::evm::ecr::*;
use std::vm::evm::eip191::eip191_hash;

// NOTE: FOR DEMONSTRATION PURPOSES ONLY
// DO NOT USE IN PRODUCTION

// Replace with a hard coded public key for the owner!
const OWNER = ~EvmAddress::from(0x0000000000000000000000000000000000000000000000000000000000000000);

fn main() -> bool {
    // A more proper predicate would read the signature from the `predicateData`
    let hi = 0xbd0c9b8792876713afa8bff383eebf31c43437823ed761cc3600d0016de5110c;
    let lo = 0x44ac566bd156b4fc71a4a4cb2655d3dd360c695edb17dc3b64d611e122fea23d;
    let signature: B512 = ~B512::from(hi, lo);

    let msg_hash = eip191_hash(tx_id());

    let result_address = ec_recover_address(signature, msg_hash);
    if let Result::Ok(address) = result_address {
        return OWNER == address.value
    } else {
        return false
    };
}
