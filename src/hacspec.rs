//!
//! hacspec Rust library.
//!
extern crate rand;

use std::cmp::min;
use std::convert::AsMut;
use std::ops::{Index, IndexMut, Range};

#[macro_export]
macro_rules! hacspec_imports {
    () => {
        use num::{BigUint, Num};
        use std::ops::*;
        use std::{cmp::PartialEq, cmp::min, fmt};
        use uint::*;
    };
}

#[macro_export]
macro_rules! hacspec_crates {
    () => {
        extern crate num;
        extern crate uint;
    };
}

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
    u32::from_le_bytes(to_array(&x[0..4]))
}

pub fn from_u32l(x: u32) -> (u8, u8, u8, u8) {
    (
        ((x & 0xFF000000) >> 24) as u8,
        ((x & 0xFF0000) >> 16) as u8,
        ((x & 0xFF00) >> 8) as u8,
        (x & 0xFF) as u8,
    )
}

/// Variable length byte arrays.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bytes {
    b: Vec<u8>,
}

impl Bytes {
    pub fn new_len(l: usize) -> Self {
        Self { b: vec![0u8; l] }
    }
    pub fn random(l: usize) -> Self {
        Self {
            b: get_random_bytes(l),
        }
    }
    pub fn from_vec(v: Vec<u8>) -> Self {
        Self { b: v.clone() }
    }
    pub fn from_array(v: &[u8]) -> Self {
        Self { b: v.to_vec() }
    }
    pub fn to_slice(&self) -> &[u8] {
        self.b.as_slice()
    }
    pub fn extend(&mut self, v: Bytes) {
        self.b.extend(v.b);
    }
    /// **Panics** if `self.len()` is not equal to the result length.
    pub fn to_array<A>(&self) -> A
    where
        A: Default + AsMut<[u8]>,
    {
        let mut a = A::default();
        <A as AsMut<[u8]>>::as_mut(&mut a).copy_from_slice(&self.b[..]);
        a
    }
    pub fn len(&self) -> usize {
        self.b.len()
    }
    pub fn split(&self, block_size: usize) -> Vec<Bytes> {
        let mut res = Vec::<Bytes>::new();
        for i in (0..self.b.len()).step_by(block_size) {
            res.push(Bytes::from_array(
                &self.b[i..min(i + block_size, self.b.len())],
            ));
        }
        res
    }
    /// Get bytes as u32.
    /// # PANICS
    /// Panics if self.len() != 4.
    pub fn to_u32l(&self) -> u32 {
        assert!(self.b.len() == 4);
        (self.b[3] as u32) << 24
            | (self.b[2] as u32) << 16
            | (self.b[1] as u32) << 8
            | (self.b[0] as u32)
    }
    /// Read a u32 into a byte array.
    pub fn from_u32l(x: u32) -> Self {
        Bytes {
            b: vec![
                ((x & 0xFF000000) >> 24) as u8,
                ((x & 0xFF0000) >> 16) as u8,
                ((x & 0xFF00) >> 8) as u8,
                (x & 0xFF) as u8,
            ],
        }
    }
    /// Get bytes as u128.
    /// # PANICS
    /// Panics if self.len() > 16.
    pub fn to_u128l(&self) -> u128 {
        assert!(self.b.len() <= 16);
        let mut r = self.b[0] as u128;
        for i in 1..self.b.len() {
            r |= (self.b[i] as u128) << i * 8;
        }
        r
    }
}

impl Index<usize> for Bytes {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.b[i]
    }
}

impl IndexMut<usize> for Bytes {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        &mut self.b[i]
    }
}

impl Index<Range<usize>> for Bytes {
    type Output = [u8];
    fn index(&self, r: Range<usize>) -> &[u8] {
        &self.b[r]
    }
}

impl IndexMut<Range<usize>> for Bytes {
    fn index_mut(&mut self, r: Range<usize>) -> &mut [u8] {
        &mut self.b[r]
    }
}

#[macro_export]
macro_rules! bytes {
    ($name:ident,$l:expr) => {
        /// Fixed length byte array.
        /// Because Rust requires fixed length arrays to have a known size at
        /// compile time there's no generic fixed length byte array here.
        /// Use this to define the fixed length byte arrays needed in your code.
        #[derive(Clone, Copy)]
        pub struct $name {
            b: [u8; $l],
        }

        impl $name {
            pub fn new() -> Self {
                Self { b: [0u8; $l] }
            }
            pub fn random() -> Self {
                let mut tmp = [0u8; $l];
                tmp.copy_from_slice(&get_random_bytes($l)[..$l]);
                Self { b: tmp.clone() }
            }
            pub fn from_array(v: [u8; $l]) -> Self {
                Self { b: v.clone() }
            }
            pub fn from_slice(v: &[u8]) -> Self {
                assert!(v.len() == $l);
                let mut tmp = [0u8; $l];
                tmp.copy_from_slice(v);
                Self { b: tmp.clone() }
            }
            pub fn len(&self) -> usize {
                $l
            }
            pub fn word(&self, start: usize) -> [u8; 4] {
                assert!(self.b.len() >= start + 4);
                let mut res = [0u8; 4];
                res.copy_from_slice(&self.b[start..start + 4]);
                res
            }
        }

        impl Index<usize> for $name {
            type Output = u8;
            fn index(&self, i: usize) -> &u8 {
                &self.b[i]
            }
        }
        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, i: usize) -> &mut u8 {
                &mut self.b[i]
            }
        }
        impl Index<Range<usize>> for $name {
            type Output = [u8];
            fn index(&self, r: Range<usize>) -> &[u8] {
                &self.b[r]
            }
        }
        impl Index<RangeFull> for $name {
            type Output = [u8];
            fn index(&self, r: RangeFull) -> &[u8] {
                &self.b[r]
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.b[..].fmt(f)
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.b[..] == other.b[..]
            }
        }
    };
}

pub fn to_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}
