//!
//! hacspec Rust library.
//! 
extern crate rand;

#[macro_export]
macro_rules! random_bytes {
    ($n:ident, $l:expr) => {
        let mut $n = [0u8; $l];
        $n.copy_from_slice(&get_random_bytes($l)[..$l]);
    };
}

pub fn get_random_bytes(l: usize) -> Vec<u8> {
    (0..l).map(|_| rand::random::<u8>()).collect()
}

pub fn to_u32l(x: &[u8]) -> u32 {
    assert!(x.len() == 4);
    (x[3] as u32) << 24 | (x[2] as u32) << 16 | (x[1] as u32) << 8 | (x[0] as u32)
}

pub fn from_u32l(x: u32) -> (u8, u8, u8, u8) {
    (
        ((x & 0xFF000000) >> 24) as u8,
        ((x & 0xFF0000) >> 16) as u8,
        ((x & 0xFF00) >> 8) as u8,
        (x & 0xFF) as u8,
    )
}
