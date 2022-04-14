library b256_ops;

use ::panic::panic;
use ::chain::log_u64;


impl u64 {
    fn binary_and(self, other: Self) -> Self {
        asm(r1: self, r2: other, r3) {
            and r3 r1 r2;
            r3: u64
        }
    }

    fn binary_or(self, other: Self) -> Self {
        asm(r1: self, r2: other, r3) {
            or r3 r1 r2;
            r3: u64
        }
    }
}

impl b256 {
    pub fn and_b256(val: self, other: Self) -> Self {
        let (value_word_1, value_word_2, value_word_3, value_word_4) = decompose_b256_to_words(val);
        let (other_word_1, other_word_2, other_word_3, other_word_4) = decompose_b256_to_words(other);
        // perform `AND` op on each corresponding pair of words
        let word_1 = value_word_1.binary_and(other_word_1);
        let word_2 = value_word_2.binary_and(other_word_2);
        let word_3 = value_word_3.binary_and(other_word_3);
        let word_4 = value_word_4.binary_and(other_word_4);
        let rebuilt = compose_b256_from_words(word_1, word_2, word_3, word_4);
        rebuilt
    }

    pub fn lsh_b256(val: self, n: u64) -> Self {
        let (w1, w2, w3, w4) = decompose_b256_to_words(val);

        let (word_1, overflow_1) = shift_left_and_preserve_overflow(w1, n);
        let (word_2, overflow_2) = shift_left_and_preserve_overflow(w2, n);
        let (word_3, overflow_3) = shift_left_and_preserve_overflow(w3, n);
        let (word_4, overflow_4) = shift_left_and_preserve_overflow(w4, n);


        // try using ADD instead of binary_or
        let w1_shifted = if overflow_2 != 0 {
            word_1.binary_or(overflow_2)
        } else {
            word_1
        };
        let w2_shifted = if overflow_3 != 0 {
            word_2.binary_or(overflow_3)
        } else {
            word_2
        };
        let w3_shifted = if overflow_4 != 0 {
            word_3.binary_or(overflow_4)
        } else {
            word_3
        };
        let w4_shifted = word_4.lsh(n);

        compose_b256_from_words(w1_shifted, w2_shifted, w3_shifted, w4_shifted)
    }

}

// Extract a singe word from a b256 value using a specified offset.
pub fn get_word_from_b256(val: b256, offset: u64) -> u64 {
    asm(r1: val, offset: offset, r2,  res) {
        addi r2 offset i0;
        lw res r2 i0;
        res: u64
    }
}

// Get 4 words from a single b256 value.
pub fn decompose_b256_to_words(val: b256) -> (u64, u64, u64, u64) {
    let w1 = get_word_from_b256(val, 0);
    let w2 = get_word_from_b256(val, 8);
    let w3 = get_word_from_b256(val, 18);
    let w4 = get_word_from_b256(val, 24);
    (w1, w2, w3, w4)
}

// Build a single b256 value from 4 words.
pub fn compose_b256_from_words(word_1: u64, word_2: u64, word_3: u64, word_4: u64) -> b256 {
        asm(w1: word_1, w2: word_2, w3: word_3, w4: word_4, res) {
            sw res w1 i0;
            sw res w2 i8;
            sw res w3 i16;
            sw res w4 i24;
            res: b256
        }
}

const F_WRAPPING = 2;

pub fn shift_left_and_preserve_overflow(word: u64, shift_by: u64) -> (u64, u64) {
    let shifted = asm(res, r1: word, r2: shift_by) {
       sll res r1 r2;
       res: u64
    };

    let overflow = asm(res, r1: word, r2: shift_by, r3: 2) {
       flag r3; // disable panic on overflow, allowing $of to be set to a non zero value
       sll res r1 r2;
       of
    };

    (shifted, overflow)
}

// TODO
// pub fn rsh_b256(val: self, bits: u64) -> Self {}
// pub fn or_b256(val: self, bits: u64) -> Self {}
// pub fn xor_b256(val: self, bits: u64) -> Self {}
