//!
//! hacspec Rust library.
//!
extern crate rand;

use std::cmp::min;
use std::convert::AsMut;
use std::fmt;
use std::ops::{Add, Index, IndexMut, Range, RangeFull};

#[macro_export]
macro_rules! hacspec_imports {
    () => {
        use num::{BigUint, Num};
        use std::ops::*;
        use std::{cmp::min, cmp::PartialEq, fmt};
        use uint::*;
        use wrapping_arithmetic::wrappit;
    };
}

#[macro_export]
macro_rules! hacspec_crates {
    () => {
        extern crate num;
        extern crate uint;
        extern crate wrapping_arithmetic;
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

// ======================== Variable length arrays ========================== //

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
        self.b.extend(v.b.clone());
    }
    pub fn extend_from_slice(&mut self, v: &[u8]) {
        self.b.extend(v);
    }
    pub fn push(&mut self, v: u8) {
        self.b.push(v);
    }
    pub fn update(&mut self, start: usize, v: &[u8]) {
        assert!(self.b.len() >= start + v.len());
        for (i, b) in v.iter().enumerate() {
            self.b[start + i] = *b;
        }
    }
    pub fn raw<'a>(&'a self) -> &'a [u8] {
        &self.b
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
    /// **Panics** if `self` is too short `start-end` is not equal to the result length.
    pub fn to_array_part<A>(&self, start: usize, end: usize) -> A
    where
        A: Default + AsMut<[u8]>,
    {
        let mut a = A::default();
        <A as AsMut<[u8]>>::as_mut(&mut a).copy_from_slice(&self.b[start..end]);
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
    /// Get a u128 representing at most the first 16 byte of this byte vector.
    /// # PANICS
    /// Panics if there's nothing to convert, i.e. self.b.is_empty().
    pub fn to_le_uint(&self) -> u128 {
        assert!(!self.b.is_empty());
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
impl From<&[u8]> for Bytes {
    fn from(x: &[u8]) -> Bytes {
        Self { b: x.to_vec() }
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(x: Vec<u8>) -> Bytes {
        Self { b: x.clone() }
    }
}

// ========================== Fixed length arrays =========================== //

#[macro_export]
macro_rules! bytes {
    ($name:ident,$l:expr) => {
        /// Fixed length byte array.
        /// Because Rust requires fixed length arrays to have a known size at
        /// compile time there's no generic fixed length byte array here.
        /// Use this to define the fixed length byte arrays needed in your code.
        #[derive(Clone, Copy)]
        pub struct $name([u8; $l]);

        impl $name {
            pub fn new() -> Self {
                Self([0u8; $l])
            }
            pub fn capacity() -> usize {
                $l
            }
            pub fn random() -> Self {
                let mut tmp = [0u8; $l];
                tmp.copy_from_slice(&get_random_bytes($l)[..$l]);
                Self(tmp.clone())
            }
            pub fn from_array(v: [u8; $l]) -> Self {
                Self(v.clone())
            }
            pub fn from_slice(v: &[u8]) -> Self {
                assert!(v.len() == $l);
                let mut tmp = [0u8; $l];
                tmp.copy_from_slice(v);
                Self(tmp.clone())
            }
            pub fn from_slice_pad(v: &[u8]) -> Self {
                assert!(v.len() <= $l);
                let mut tmp = [0u8; $l];
                for i in 0..v.len() {
                    tmp[i] = v[i];
                }
                Self(tmp.clone())
            }
            /// This takes an arbitrary length slice and takes at most $l bytes
            /// zero-padded into $name.
            pub fn from_slice_lazy(v: &[u8]) -> Self {
                let mut tmp = [0u8; $l];
                for i in 0..min($l, v.len()) {
                    tmp[i] = v[i];
                }
                Self(tmp.clone())
            }
            pub fn update(&mut self, start: usize, v: &[u8]) {
                for (i, b) in v.iter().enumerate() {
                    self.0[start + i] = *b;
                }
            }
            pub fn raw<'a>(&'a self) -> &'a [u8] {
                &self.0
            }
            pub fn len(&self) -> usize {
                $l
            }
            pub fn word(&self, start: usize) -> [u8; 4] {
                assert!(self.0.len() >= start + 4);
                let mut res = [0u8; 4];
                res.copy_from_slice(&self.0[start..start + 4]);
                res
            }
            /// Get an array for the given range `r`.
            ///
            /// #Panics
            /// Panics if `self` is too short `start-end` is not equal to the result length.
            pub fn get<A>(&self, r: Range<usize>) -> A
            where
                A: Default + AsMut<[u8]>,
            {
                let mut a = A::default();
                <A as AsMut<[u8]>>::as_mut(&mut a).copy_from_slice(&self.0[r]);
                a
            }

            pub fn from_u64_slice_le(x: &[u64]) -> Self {
                let mut result: [u8; $l] = [0; $l];
                for i in (0..x.len()).rev() {
                    result[0 + (i * 8)] = (x[i] & 0xFFu64) as u8;
                    result[1 + (i * 8)] = ((x[i] & 0xFF00u64) >> 8) as u8;
                    result[2 + (i * 8)] = ((x[i] & 0xFF0000u64) >> 16) as u8;
                    result[3 + (i * 8)] = ((x[i] & 0xFF000000u64) >> 24) as u8;
                    result[4 + (i * 8)] = ((x[i] & 0xFF00000000u64) >> 32) as u8;
                    result[5 + (i * 8)] = ((x[i] & 0xFF0000000000u64) >> 40) as u8;
                    result[6 + (i * 8)] = ((x[i] & 0xFF000000000000u64) >> 48) as u8;
                    result[7 + (i * 8)] = ((x[i] & 0xFF00000000000000u64) >> 56) as u8;
                }
                Self(result.clone())
            }
        }

        impl Index<usize> for $name {
            type Output = u8;
            fn index(&self, i: usize) -> &u8 {
                &self.0[i]
            }
        }
        impl Index<u8> for $name {
            type Output = u8;
            fn index(&self, i: u8) -> &u8 {
                &self.0[usize::from(i)]
            }
        }
        impl Index<i32> for $name {
            type Output = u8;
            fn index(&self, i: i32) -> &u8 {
                &self.0[i as usize] // TODO: this conversion might be bad
            }
        }
        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, i: usize) -> &mut u8 {
                &mut self.0[i]
            }
        }
        impl Index<Range<usize>> for $name {
            type Output = [u8];
            fn index(&self, r: Range<usize>) -> &[u8] {
                &self.0[r]
            }
        }
        impl Index<RangeFull> for $name {
            type Output = [u8];
            fn index(&self, r: RangeFull) -> &[u8] {
                &self.0[r]
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0[..].fmt(f)
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0[..] == other.0[..]
            }
        }
        impl From<[u8; $l]> for $name {
            fn from(x: [u8; $l]) -> $name {
                $name(x.clone())
            }
        }
        impl From<&[u8]> for $name {
            fn from(x: &[u8]) -> $name {
                $name::from_slice(x)
            }
        }
        impl From<$name> for [u8; $l] {
            fn from(x: $name) -> [u8; $l] {
                x.0
            }
        }
        /// Build this array from a slice of the appropriate length of a u64s (little-endian).
        /// # PANICS
        /// Panics if the slice doesn't fit into this array.
        impl From<&[u64]> for $name {
            fn from(x: &[u64]) -> $name {
                assert!($l == x.len() * 8);
                $name::from_u64_slice_le(x)
            }
        }
        /// Build this array from an array of the appropriate length of a u64s (little-endian).
        /// # PANICS
        /// Panics if the slice doesn't fit into this array.
        impl From<[u64; $l / 8]> for $name {
            fn from(x: [u64; $l / 8]) -> $name {
                $name::from_u64_slice_le(&x)
            }
        }
    };
}

#[test]
fn test_bytes() {
    bytes!(TestBytes, 77);
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

// ============================== Wrapping u64 ============================== //

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct U64w(u64);
impl Into<u64> for U64w {
    fn into(self) -> u64 {
        self.0
    }
}
impl From<u64> for U64w {
    fn from(x: u64) -> U64w {
        U64w(x)
    }
}
impl From<usize> for U64w {
    fn from(x: usize) -> U64w {
        U64w(x as u64)
    }
}

impl<'a> Add<&'a U64w> for U64w {
    type Output = U64w;

    #[inline]
    fn add(self, other: &U64w) -> U64w {
        self.0.wrapping_add(other.0).into()
    }
}

impl Add<U64w> for U64w {
    type Output = U64w;

    #[inline]
    fn add(self, other: U64w) -> U64w {
        self.0.wrapping_add(other.0).into()
    }
}

#[test]
fn u64w_test() {
    let a = U64w(std::u64::MAX);
    let b = U64w(2);
    assert_eq!(U64w(1), a + &b);
}
