//!
//! hacspec Rust library.
//!
extern crate rand;

use std::cmp::min;
use std::convert::AsMut;
use std::ops::{Index, IndexMut, Range, RangeFull};
use std::num::ParseIntError;

#[macro_export]
macro_rules! hacspec_imports {
    () => {
        use num::{BigUint, Num, Zero};
        use std::num::ParseIntError;
        use std::ops::*;
        use std::{cmp::min, cmp::PartialEq, fmt};
        use uint::{natmod_p::*, traits::*, uint_n::*};
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

pub fn hex_string_to_bytes(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0);
    let b: Result<Vec<u8>, ParseIntError> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    b.expect("Error parsing hex string")
}

/// Common trait for all byte arrays.
pub trait ByteArray {
    fn raw<'a>(&'a self) -> &'a [u8];
    fn len(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<u8>;
}

// ======================== Variable length arrays ========================== //

#[derive(Copy, Clone, Debug)]
pub struct ByteSlice<'a> {
    b: &'a[u8]
}
impl<'a> ByteSlice<'a> {
    fn new(b_in: &'a Bytes) -> ByteSlice<'a> {
        Self {
            b: &b_in[..]
        }
    }
}

/// Variable length byte arrays.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bytes {
    b: Vec<u8>,
}

impl Bytes {
    pub fn get_slice<'a>(&'a self) -> ByteSlice<'a> {
        ByteSlice::new(&self)
    }
    pub fn new_len(l: usize) -> Self {
        Self { b: vec![0u8; l] }
    }
    pub fn is_empty(&self) -> bool {
        self.b.is_empty()
    }
    pub fn from_array(v: &[u8]) -> Self {
        Self { b: v.to_vec() }
    }
    pub fn update_raw(&mut self, start: usize, v: &[u8]) {
        assert!(self.len() >= start + v.len());
        for (i, b) in v.iter().enumerate() {
            self[start + i] = *b;
        }
    }
    pub fn update_vec(&mut self, start: usize, v: Vec<u8>) {
        assert!(self.len() >= start + v.len());
        for (i, b) in v.iter().enumerate() {
            self[start + i] = *b;
        }
    }
    pub fn update(&mut self, start: usize, v: &dyn ByteArray) {
        assert!(self.len() >= start + v.len());
        for (i, b) in v.iter().enumerate() {
            self[start + i] = *b;
        }
    }
    /// **Panics** if `self` is too short `start-end` is not equal to the result length.
    pub fn get<A>(&self, r: Range<usize>) -> A
    where
        A: Default + AsMut<[u8]>,
    {
        let mut a = A::default();
        <A as AsMut<[u8]>>::as_mut(&mut a).copy_from_slice(&self[r]);
        a
    }
    pub fn split(&self, block_size: usize) -> Vec<Bytes> {
        let mut res = Vec::<Bytes>::new();
        for i in (0..self.len()).step_by(block_size) {
            res.push(Bytes::from_array(&self[i..min(i + block_size, self.len())]));
        }
        res
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
        assert!(!self.is_empty());
        let mut r = self[0] as u128;
        for i in 1..self.len() {
            r |= (self[i] as u128) << i * 8;
        }
        r
    }
}

impl ByteArray for Bytes {
    fn raw<'a>(&'a self) -> &'a [u8] {
        &self.b
    }
    fn len(&self) -> usize {
        self.b.len()
    }
    fn iter(&self) -> std::slice::Iter<u8> {
        self.b.iter()
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

impl Index<RangeFull> for Bytes {
    type Output = [u8];
    fn index(&self, _r: RangeFull) -> &[u8] {
        &self.b[..]
    }
}

impl IndexMut<Range<usize>> for Bytes {
    fn index_mut(&mut self, r: Range<usize>) -> &mut [u8] {
        &mut self.b[r]
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(x: Vec<u8>) -> Bytes {
        Self { b: x.clone() }
    }
}
impl Into<Vec<u8>> for Bytes {
    fn into(self) -> Vec<u8> {
        self.b.to_vec()
    }
}
/// Read hex string to Bytes.
impl From<&str> for Bytes {
    fn from(s: &str) -> Bytes {
        Bytes::from(hex_string_to_bytes(s))
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
            pub fn from_array(v: [u8; $l]) -> Self {
                Self(v.clone())
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
            /// This takes an arbitrary length vec and takes at most $l bytes
            /// zero-padded into $name.
            pub fn from_vec_lazy(v: Vec<u8>) -> Self {
                let mut tmp = [0u8; $l];
                for i in 0..min($l, v.len()) {
                    tmp[i] = v[i];
                }
                Self(tmp.clone())
            }
            pub fn update_raw(&mut self, start: usize, v: &[u8]) {
                for (i, b) in v.iter().enumerate() {
                    self[start + i] = *b;
                }
            }
            pub fn update_vec(&mut self, start: usize, v: Vec<u8>) {
                for (i, b) in v.iter().enumerate() {
                    self[start + i] = *b;
                }
            }
            pub fn len(&self) -> usize {
                $l
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
                <A as AsMut<[u8]>>::as_mut(&mut a).copy_from_slice(&self[r]);
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

            /// Convert a `Field` to a byte array (little endian).
            /// TODO: The `From` trait doesn't work for this for some reason.
            pub fn from_field<T>(f: T) -> Self
            where
                T: Field,
            {
                $name::from(&f.to_bytes_le()[..])
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name::new()
            }
        }
        impl AsMut<[u8]> for $name {
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }
        impl ByteArray for $name {
            fn raw<'a>(&'a self) -> &'a [u8] {
                &self.0
            }
            fn len(&self) -> usize {
                $l
            }
            fn iter(&self) -> std::slice::Iter<u8> {
                self.0.iter()
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
        impl From<Vec<u8>> for $name {
            fn from(x: Vec<u8>) -> $name {
                assert!(x.len() <= $l);
                let mut tmp = [0u8; $l];
                for (i, e) in x.iter().enumerate() {
                    tmp[i] = *e;
                }
                $name(tmp.clone())
            }
        }
        impl From<$name> for Vec<u8> {
            fn from(x: $name) -> Vec<u8> {
                x.0.to_vec()
            }
        }
        impl From<&[u8]> for $name {
            fn from(x: &[u8]) -> $name {
                $name::from_slice_pad(x)
            }
        }
        impl From<$name> for [u8; $l] {
            fn from(x: $name) -> [u8; $l] {
                x.0
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
        /// Read hex string to bytes.
        impl From<&str> for $name {
            fn from(s: &str) -> $name {
                $name::from(hex_string_to_bytes(s))
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
