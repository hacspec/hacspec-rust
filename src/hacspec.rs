//!
//! hacspec Rust library.
//!
#[macro_export]
macro_rules! hacspec_crates {
    () => {
        extern crate num;
        extern crate rand;
        extern crate secret_integers;
        extern crate uint;
        extern crate wrapping_arithmetic;
    };
}

#[macro_export]
macro_rules! hacspec_imports {
    () => {
        #[allow(unused_imports)]
        use num::{BigUint, Num, Zero};
        #[allow(unused_imports)]
        use secret_integers::*;
        #[allow(unused_imports)]
        use std::num::ParseIntError;
        #[allow(unused_imports)]
        use std::ops::*;
        #[allow(unused_imports)]
        use std::{cmp::min, cmp::PartialEq, fmt};
        #[allow(unused_imports)]
        use uint::{natmod_p::*, traits::*, uint_n::*};
        #[allow(unused_imports)]
        use wrapping_arithmetic::wrappit;
    };
}

hacspec_crates!();

hacspec_imports!();

fn hex_string_to_bytes(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0);
    let b: Result<Vec<u8>, ParseIntError> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    b.expect("Error parsing hex string")
}

/// Common trait for all byte arrays.
pub trait SeqTrait<T: Copy> {
    fn raw<'a>(&'a self) -> &'a [T];
    fn len(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<T>;
}

// ======================== Variable length arrays ========================== //

/// Variable length byte arrays.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Seq<T: Copy> {
    b: Vec<T>,
}

pub type Bytes = Seq<U8>;

impl<T: Copy + Default> Seq<T> {
    pub fn new_len(l: usize) -> Self {
        Self {
            b: vec![T::default(); l],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.b.is_empty()
    }
    pub fn from_array(v: &[T]) -> Self {
        Self { b: v.to_vec() }
    }
    pub fn len(&self) -> usize {
        self.b.len()
    }
    pub fn update(&mut self, start: usize, v: &dyn SeqTrait<T>) {
        assert!(self.len() >= start + v.len());
        for (i, b) in v.iter().enumerate() {
            self[start + i] = *b;
        }
    }
    pub fn update_sub(
        &mut self,
        start_out: usize,
        v: &dyn SeqTrait<T>,
        start_in: usize,
        len: usize,
    ) {
        assert!(self.len() >= start_out + len);
        assert!(v.len() >= start_in + len);
        for (i, b) in v.iter().skip(start_in).take(len).enumerate() {
            self[start_out + i] = *b;
        }
    }
    /// **Panics** if `self` is too short `start-end` is not equal to the result length.
    pub fn get<A: SeqTrait<T>>(&self, r: Range<usize>) -> A
    where
        A: Default + AsMut<[T]>,
    {
        let mut a = A::default();
        <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(&self[r]);
        a
    }
}

impl Seq<U8> {
    fn get_random_vec(l: usize) -> Vec<U8> {
        (0..l).map(|_| rand::random::<u8>()).map(|x| U8::classify(x)).collect()
    }

    pub fn random(l: usize) -> Self {
        Self {
            b: Seq::get_random_vec(l),
        }
    }
}

impl<T: Copy> SeqTrait<T> for Seq<T> {
    fn raw<'a>(&'a self) -> &'a [T] {
        &self.b
    }
    fn len(&self) -> usize {
        self.b.len()
    }
    fn iter(&self) -> std::slice::Iter<T> {
        self.b.iter()
    }
}

impl<T: Copy> Index<usize> for Seq<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.b[i]
    }
}

impl<T: Copy> IndexMut<usize> for Seq<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.b[i]
    }
}

impl<T: Copy> Index<Range<usize>> for Seq<T> {
    type Output = [T];
    fn index(&self, r: Range<usize>) -> &[T] {
        &self.b[r]
    }
}

impl<T: Copy> Index<RangeFull> for Seq<T> {
    type Output = [T];
    fn index(&self, _r: RangeFull) -> &[T] {
        &self.b[..]
    }
}

impl<T: Copy> IndexMut<Range<usize>> for Seq<T> {
    fn index_mut(&mut self, r: Range<usize>) -> &mut [T] {
        &mut self.b[r]
    }
}
impl<T: Copy> From<Vec<T>> for Seq<T> {
    fn from(x: Vec<T>) -> Seq<T> {
        Self { b: x.clone() }
    }
}
/// Read hex string to Bytes.
impl From<&str> for Seq<U8> {
    fn from(s: &str) -> Seq<U8> {
        Seq::from(hex_string_to_bytes(s).iter().map(|x| U8::classify(*x)).collect::<Vec<_>>())
    }
}

// ========================== Fixed length arrays =========================== //

#[macro_export]
macro_rules! bytes {
    ($name:ident, $l:expr) => {
        array!($name, $l, U8, u8);
        impl $name {
            /// Convert a `Field` to a byte array (little endian).
            /// TODO: The `From` trait doesn't work for this for some reason.
            pub fn from_field<T>(f: T) -> Self
            where
                T: Field,
            {
                $name::from(
                    (&f.to_bytes_le()[..])
                        .iter()
                        .map(|x| U8::classify(*x))
                        .collect::<Vec<U8>>(),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! array_base {
    ($name:ident,$l:expr,$t:ty, $tbase:ty) => {
        /// Fixed length byte array.
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
            pub fn from_seq_pad(v: &dyn SeqTrait<$t>) -> Self {
                assert!(v.len() <= $l);
                let mut tmp = [<$t>::default(); $l];
                for (i, x) in v.iter().enumerate() {
                    tmp[i] = *x;
                }
                Self(tmp.clone())
            }
            pub fn from_exact_seq(v: &dyn SeqTrait<$t>) -> Self {
                assert!(v.len() == $l);
                let mut tmp = [<$t>::default(); $l];
                for (i, x) in v.iter().enumerate() {
                    tmp[i] = *x;
                }
                Self(tmp.clone())
            }
            /// This takes an arbitrary length slice and takes at most $l bytes
            /// zero-padded into $name.
            pub fn from_slice_lazy(v: &[$t]) -> Self {
                let mut tmp = [<$t>::default(); $l];
                for i in 0..min($l, v.len()) {
                    tmp[i] = v[i];
                }
                Self(tmp.clone())
            }
            /// This takes an arbitrary length vec and takes at most $l bytes
            /// zero-padded into $name.
            pub fn from_vec_lazy(v: Vec<$t>) -> Self {
                let mut tmp = [<$t>::default(); $l];
                for i in 0..min($l, v.len()) {
                    tmp[i] = v[i];
                }
                Self(tmp.clone())
            }
            pub fn update(&mut self, start: usize, v: &dyn SeqTrait<$t>) {
                assert!(self.len() >= start + v.len());
                for (i, b) in v.iter().enumerate() {
                    self[start + i] = *b;
                }
            }
            pub fn update_sub(
                &mut self,
                start_out: usize,
                v: &dyn SeqTrait<$t>,
                start_in: usize,
                len: usize,
            ) {
                assert!(self.len() >= start_out + len);
                assert!(v.len() >= start_in + len);
                for (i, b) in v.iter().skip(start_in).take(len).enumerate() {
                    self[start_out + i] = *b;
                }
            }
            pub fn len(&self) -> usize {
                $l
            }
            /// Get an array for the given range `r`.
            ///
            /// #Panics
            /// Panics if `self` is too short `start-end` is not equal to the result length.
            pub fn get<A: SeqTrait<$t>>(&self, r: Range<usize>) -> A
            where
                A: Default + AsMut<[$t]>,
            {
                let mut a = A::default();
                <A as AsMut<[$t]>>::as_mut(&mut a).copy_from_slice(&self[r]);
                a
            }
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
        impl Index<Range<usize>> for $name {
            type Output = [$t];
            fn index(&self, r: Range<usize>) -> &[$t] {
                &self.0[r]
            }
        }
        impl IndexMut<Range<usize>> for $name {
            fn index_mut(&mut self, r: Range<usize>) -> &mut [$t] {
                &mut self.0[r]
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
                assert!(x.len() <= $l);
                let mut tmp = [<$t>::default(); $l];
                for (i, e) in x.iter().enumerate() {
                    tmp[i] = *e;
                }
                $name(tmp.clone())
            }
        }
        impl From<$name> for Vec<$t> {
            fn from(x: $name) -> Vec<$t> {
                x.0.to_vec()
            }
        }
        impl From<&[$t]> for $name {
            fn from(x: &[$t]) -> $name {
                $name::from_seq_pad(&Seq::from(x.to_vec()))
            }
        }
        impl From<$name> for [$t; $l] {
            fn from(x: $name) -> [$t; $l] {
                x.0
            }
        }

        impl $name {
            pub fn random() -> $name {
                let mut tmp = [<$t>::default(); $l];
                tmp.copy_from_slice(&$name::get_random_vec($l)[..$l]);
                Self(tmp.clone())
            }
        }

        /// Read hex string to Bytes.
        impl From<&str> for $name {
            fn from(s: &str) -> $name {
                let v = $name::hex_string_to_vec(s);
                let mut o = $name::new();
                assert!(v.len() == $l);
                for i in 0..$l {
                    o[i] = v[i]
                }
                o
            }
        }
    };
}

pub fn to_array<A: SeqTrait<T>, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}

#[macro_export]
macro_rules! array {
    ($name:ident,$l:expr,$t:ty, $tbase:ty) => {
        array_base!($name, $l, $t, $tbase);

        impl $name {
            fn hex_string_to_vec(s: &str) -> Vec<$t> {
                assert!(s.len() % std::mem::size_of::<$t>() == 0);
                let b: Result<Vec<$t>, ParseIntError> = (0..s.len())
                    .step_by(2)
                    .map(|i| <$tbase>::from_str_radix(&s[i..i + 2], 16).map(<$t>::classify))
                    .collect();
                b.expect("Error parsing hex string")
            }

            pub fn get_random_vec(l: usize) -> Vec<$t> {
                (0..l)
                    .map(|_| <$t>::classify(rand::random::<$tbase>()))
                    .collect()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0[..]
                    .iter()
                    .map(|x| <$t>::declassify(*x))
                    .collect::<Vec<_>>()
                    .fmt(f)
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0[..]
                    .iter()
                    .map(|x| <$t>::declassify(*x))
                    .collect::<Vec<_>>()
                    == other.0[..]
                        .iter()
                        .map(|x| <$t>::declassify(*x))
                        .collect::<Vec<_>>()
            }
        }
    };
}

#[macro_export]
macro_rules! public_array {
    ($name:ident,$l:expr,$t:ty) => {
        array_base!($name, $l, $t, $t);
        impl $name {
            fn hex_string_to_vec(s: &str) -> Vec<$t> {
                assert!(s.len() % std::mem::size_of::<$t>() == 0);
                let b: Result<Vec<$t>, ParseIntError> = (0..s.len())
                    .step_by(2)
                    .map(|i| <$t>::from_str_radix(&s[i..i + 2], 16))
                    .collect();
                b.expect("Error parsing hex string")
            }

            pub fn get_random_vec(l: usize) -> Vec<$t> {
                (0..l).map(|_| rand::random::<$t>()).collect()
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
    };
}

bytes!(U32Word, 4);
bytes!(U128Word, 16);
bytes!(U64Word, 8);
public_array!(Counter, 2, usize);

pub fn u32_to_le_bytes(x: U32) -> U32Word {
    U32Word([
        U8::from((x & U32::classify(0xFF000000u32)) >> 24),
        U8::from((x & U32::classify(0xFF0000u32)) >> 16),
        U8::from((x & U32::classify(0xFF00u32)) >> 8),
        U8::from(x & U32::classify(0xFFu32)),
    ])
}

pub fn u32_from_le_bytes(s: U32Word) -> U32 {
    U32::from_bytes_le(&s.0)[0]
}

pub fn u32_to_be_bytes(x: U32) -> U32Word {
    U32Word::from(U32::to_bytes_be(&[x]).as_slice())
}

pub fn u128_from_le_bytes(s: U128Word) -> U128 {
    U128::from_bytes_le(&s.0)[0]
}

pub fn u64_to_be_bytes(x: U64) -> U64Word {
    U64Word::from(U64::to_bytes_be(&[x]).as_slice())
}

pub fn u64_to_le_bytes(x: U64) -> U64Word {
    U64Word::from(U64::to_bytes_le(&[x]).as_slice())
}

pub fn u64_slice_to_le_u8s(x: &dyn SeqTrait<U64>) -> Bytes {
    let mut result = Bytes::new_len(x.len() * 8);
    for (i, v) in x.iter().enumerate().rev() {
        result[0 + (i * 8)] = U8::from(*v & U64::classify(0xFFu64));
        result[1 + (i * 8)] = U8::from((*v & U64::classify(0xFF00u64)) >> 8);
        result[2 + (i * 8)] = U8::from((*v & U64::classify(0xFF0000u64)) >> 16);
        result[3 + (i * 8)] = U8::from((*v & U64::classify(0xFF000000u64)) >> 24);
        result[4 + (i * 8)] = U8::from((*v & U64::classify(0xFF00000000u64)) >> 32);
        result[5 + (i * 8)] = U8::from((*v & U64::classify(0xFF0000000000u64)) >> 40);
        result[6 + (i * 8)] = U8::from((*v & U64::classify(0xFF000000000000u64)) >> 48);
        result[7 + (i * 8)] = U8::from((*v & U64::classify(0xFF00000000000000u64)) >> 56);
    }
    result
}
