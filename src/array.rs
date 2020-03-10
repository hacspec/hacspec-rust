//!
//! # Arrays
//!
//! This module implements fixed-length arrays and utility functions for it.
//!
//! Note that implementations have to be created with one of the provided macros
//! such that there's no documentation in here.
//!
//! You can find examples for the different types of arrays here:
//! * [DocSecretBytes](../struct.DocSecretBytes.html) for `bytes!(DocSecretBytes, 64)`
//! * [DocPublicBytes](../struct.DocPublicBytes.html) for `public_bytes!(DocPublicBytes, 64)`
//! * [DocSecretArray](../struct.DocSecretArray.html) for `array!(DocSecretArray, 64, U32)`
//! * [DocPublicArray](../struct.DocPublicArray.html) for `array!(DocPublicArray, 64, u32)`
//!
//! **Note** that all macros starting with an underscore (`_array_base` etc.)
//! are note intended for public use. Unfortunately it's not possible to hide
//! them.

#[macro_export]
macro_rules! _array_base {
    ($name:ident,$l:expr,$t:ty) => {
        /// Fixed length array.
        /// Because Rust requires fixed length arrays to have a known size at
        /// compile time there's no generic fixed length byte array here.
        /// Use this to define the fixed length byte arrays needed in your code.
        #[derive(Clone, Copy)]
        pub struct $name(pub [$t; $l]);

        impl From<[$t; $l]> for $name {
            fn from(v: [$t; $l]) -> Self {
                Self(v.clone())
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self([<$t>::default(); $l])
            }
            pub fn capacity() -> usize {
                $l
            }
            pub fn update<A: SeqTrait<$t>>(mut self, start: usize, v: A) -> Self {
                debug_assert!(self.len() >= start + v.len());
                for (i, b) in v.iter().enumerate() {
                    self[start + i] = *b;
                }
                self
            }
            pub fn update_sub<A: SeqTrait<$t>>(
                mut self,
                start_out: usize,
                v: A,
                start_in: usize,
                len: usize,
            ) -> Self {
                debug_assert!(self.len() >= start_out + len);
                debug_assert!(v.len() >= start_in + len);
                for (i, b) in v.iter().skip(start_in).take(len).enumerate() {
                    self[start_out + i] = *b;
                }
                self
            }
            pub fn to_bytes_be(&self) -> ByteSeq {
                const FACTOR: usize = core::mem::size_of::<$t>();
                let mut out = [0u8; $l * FACTOR];
                for i in 0..$l {
                    let tmp = <$t>::from(self[i]).to_be_bytes();
                    for j in 0..FACTOR {
                        out[i * FACTOR + j] = tmp[j];
                    }
                }
                out
            }
            // s.get_chunk(i: usize, block_size: usize)
            // s.update_chunk(i: usize, block_size: usize, v: array!)
            // s.chunks(block_size: usize)
        }

        impl Default for $name {
            fn default() -> Self {
                $name::new()
            }
        }
        impl AsMut<[$t]> for $name {
            fn as_mut(&mut self) -> &mut [$t] {
                &mut self.0
            }
        }
        impl SeqTrait<$t> for $name {
            fn raw<'a>(&'a self) -> &'a [$t] {
                &self.0
            }
            fn len(&self) -> usize {
                $l
            }
            fn iter(&self) -> std::slice::Iter<$t> {
                self.0.iter()
            }
        }

        // TODO: use macro for these and add asserts
        impl Index<usize> for $name {
            type Output = $t;
            fn index(&self, i: usize) -> &$t {
                &self.0[i]
            }
        }
        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, i: usize) -> &mut $t {
                &mut self.0[i]
            }
        }

        impl Index<u8> for $name {
            type Output = $t;
            fn index(&self, i: u8) -> &$t {
                &self.0[i as usize]
            }
        }
        impl IndexMut<u8> for $name {
            fn index_mut(&mut self, i: u8) -> &mut $t {
                &mut self.0[i as usize]
            }
        }
        impl Index<u32> for $name {
            type Output = $t;
            fn index(&self, i: u32) -> &$t {
                &self.0[i as usize]
            }
        }
        impl IndexMut<u32> for $name {
            fn index_mut(&mut self, i: u32) -> &mut $t {
                &mut self.0[i as usize]
            }
        }
        impl Index<i32> for $name {
            type Output = $t;
            fn index(&self, i: i32) -> &$t {
                &self.0[i as usize]
            }
        }
        impl IndexMut<i32> for $name {
            fn index_mut(&mut self, i: i32) -> &mut $t {
                &mut self.0[i as usize]
            }
        }
        impl Index<RangeFull> for $name {
            type Output = [$t];
            fn index(&self, r: RangeFull) -> &[$t] {
                &self.0[r]
            }
        }
        impl From<Vec<$t>> for $name {
            fn from(x: Vec<$t>) -> $name {
                debug_assert!(x.len() <= $l);
                let mut tmp = [<$t>::default(); $l];
                for (i, e) in x.iter().enumerate() {
                    tmp[i] = *e;
                }
                $name(tmp.clone())
            }
        }
        // TODO: use SeqSlice
        impl From<Seq<$t>> for $name {
            fn from(x: Seq<$t>) -> $name {
                debug_assert!(x.len() <= $l);
                let mut tmp = [<$t>::default(); $l];
                for (i, e) in x.iter().enumerate() {
                    tmp[i] = *e;
                }
                $name(tmp.clone())
            }
        }

        impl $name {
            // TODO: move to SeqTrait
            pub fn random() -> $name {
                let mut tmp = [<$t>::default(); $l];
                tmp.copy_from_slice(&$name::get_random_vec($l)[..$l]);
                Self(tmp.clone())
            }
            fn hex_string_to_vec(s: &str) -> Vec<$t> {
                debug_assert!(s.len() % std::mem::size_of::<$t>() == 0);
                let b: Result<Vec<$t>, ParseIntError> = (0..s.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map(<$t>::from))
                    .collect();
                b.expect("Error parsing hex string")
            }
        }

        /// Read hex string to Bytes.
        impl From<&str> for $name {
            fn from(s: &str) -> $name {
                let v = $name::hex_string_to_vec(s);
                let mut o = $name::new();
                debug_assert!(v.len() == $l);
                for i in 0..$l {
                    o[i] = v[i]
                }
                o
            }
        }

        // TODO: add other operators for point-wise operations and rotating
        /// Element wise xor of two arrays
        impl std::ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut out = Self::new();
                for (a, (b, c)) in out.0.iter_mut().zip(self.0.iter().zip(rhs.0.iter())) {
                    *a = *b ^ *c;
                }
                out
            }
        }
    };
}

#[macro_export]
/// This creates arrays for secret integers, i.e. `$t` is the secret integer
/// type and `$tbase` is the according Rust type.
macro_rules! _secret_array {
    ($name:ident,$l:expr,$t:ty, $tbase:ty) => {
        _array_base!($name, $l, $t);

        // TODO: add declassify

        /// Create an array from a regular Rust array.
        ///
        /// # Examples
        ///
        /// ```
        /// use hacspec::prelude::*;
        ///
        /// bytes!(Block, 5);
        /// let b = Block::from([1, 2, 3, 4, 5]);
        /// ```
        impl From<[$tbase; $l]> for $name {
            fn from(v: [$tbase; $l]) -> $name {
                debug_assert!(v.len() == $l);
                Self::from(
                    v[..]
                        .iter()
                        .map(|x| <$t>::classify(*x))
                        .collect::<Vec<$t>>(),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! _public_array {
    ($name:ident,$l:expr,$t:ty) => {
        _array_base!($name, $l, $t);
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
    };
}

// The following are the macros intended for use from the outside.

#[macro_export]
/// Create a new array with the given name, length, and type.
macro_rules! array {
    ($name:ident, $l:expr, U8) => {
        _secret_array!($name, $l, U8, u8);
        impl $name {
            // TODO: do we want this? If so, add a complete set.
            pub fn to_U32s_be(&self) -> Seq<U32> {
                let mut out = [U32::default(); $l / 4];
                for (i, block) in self.0.chunks(4).enumerate() {
                    out[i] = u32_from_be_bytes(block.into());
                }
                out.into()
            }
        }
    };
    ($name:ident, $l:expr, U16) => {
        _secret_array!($name, $l, U16, u16);
    };
    ($name:ident, $l:expr, U32) => {
        _secret_array!($name, $l, U32, u32);
    };
    ($name:ident, $l:expr, U64) => {
        _secret_array!($name, $l, U64, u64);
    };
    ($name:ident, $l:expr, U128) => {
        _secret_array!($name, $l, U128, u128);
    };
    ($name:ident, $l:expr, u8) => {
        _public_array!($name, $l, u8);
        impl $name {
            // TODO: do we want this? If so, add a complete set.
            pub fn to_u32s_be<T: SeqTrait>(&self) -> T {
                let mut out = [0u32; $l / 4];
                for (i, block) in self.0.chunks(4).enumerate() {
                    debug_assert!(block.len() == 4);
                    out[i] = u32::from_be_bytes(to_array(block));
                }
                out.into()
            }
            pub fn to_hex(&self) -> String {
                let strs: Vec<String> = self.0.iter().map(|b| format!("{:02x}", b)).collect();
                strs.join("")
            }
        }
    };
    ($name:ident, $l:expr, $t:ty) => {
        _public_array!($name, $l, $t);
    };
}

#[macro_export]
/// Convenience function to create a new byte array (of type `U8`) with the given
/// name and length.
macro_rules! bytes {
    ($name:ident, $l:expr) => {
        array!($name, $l, U8);
    };
}

#[macro_export]
/// Convenience function to create a new public byte array (of type `u8`) with
/// the given name and length.
macro_rules! public_bytes {
    ($name:ident, $l:expr) => {
        array!($name, $l, u8);
    };
}
