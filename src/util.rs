//!
//! Utility functions for hacspec internally.
//! 

use std::num::ParseIntError;

pub fn hex_string_to_bytes(s: &str) -> Vec<u8> {
    debug_assert!(s.len() % 2 == 0);
    let b: Result<Vec<u8>, ParseIntError> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    b.expect("Error parsing hex string")
}
