library hash;

use ::core::num::*;

// Should this be a trait eventually? Do we want to allow people to customize what `!` does?
// Scala says yes, Rust says perhaps...
pub fn not(a: bool) -> bool {
    // using direct asm for perf
    asm(r1: a, r2) {
        eq r2 r1 zero;
        r2: bool
    }
}

/// Returns the SHA-2-256 hash of `param` with a given offset
pub fn sha256_with_offset<T>(param: T, offset: u8) -> b256 {
    let mut result_buffer: b256 = ~b256::min();
    let size = __size_of::<T>() - offset;
    if !__is_reference_type::<T>() {
        asm(buffer, ptr: param, bytes: size, hash: result_buffer) {
            move buffer sp; // Make `buffer` point to the current top of the stack
            cfei i8; // Grow stack by 1 word
            add ptr ptr offset; // add offset to the pointer
            sw buffer ptr i0; // Save value in register at "ptr" to memory at "buffer"
            s256 hash buffer bytes; // Hash the next eight bytes starting from "buffer" into "hash"
            cfsi i8; // Shrink stack by 1 word
            hash: b256 // Return
        }
    } else {
        asm(hash: result_buffer, ptr: param, bytes: size) {
            add ptr ptr offset; // add offset to the pointer
            s256 hash ptr bytes; // Hash the next "size" number of bytes starting from "ptr" into "hash"
            hash: b256 // Return
        }
    }
}

/// Returns the SHA-2-256 hash of `param` with an offset of 0
pub fn sha256<T>(param: T) -> b256 {
    sha256_with_offset(param, 0)
}

/// Returns the KECCAK-256 hash of `param` with a given offset
pub fn keccak256_with_offset<T>(param: T, offset: u8) -> b256 {
    let mut result_buffer: b256 = ~b256::min();
    let size = __size_of::<T>() - offset;
    if !__is_reference_type::<T>() {
        asm(buffer, ptr: param, bytes: size, hash: result_buffer) {
            move buffer sp; // Make `buffer` point to the current top of the stack
            cfei i8; // Grow stack by 1 word
            add ptr ptr offset; // add offset to the pointer
            sw buffer ptr i0; // Save value in register at "ptr" to memory at "buffer"
            k256 hash buffer bytes; // Hash the next eight bytes starting from "buffer" into "hash"
            cfsi i8; // Shrink stack by 1 word
            hash: b256 // Return
        }
    } else {
        asm(hash: result_buffer, ptr: param, bytes: size) {
            add ptr ptr offset; // add offset to the pointer
            k256 hash ptr bytes; // Hash the next "size" number of bytes starting from "ptr" into "hash"
            hash: b256 // Return
        }
    }
}

/// Returns the KECCAK-256 hash of `param` with an offset of 0
pub fn keccak256<T>(param: T) -> b256 {
    keccak256_with_offset(param, 0)
}
