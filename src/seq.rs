//!
//! # Sequences
//! 
//! This module implements variable-length sequences and utility functions for it.
//! 

use crate::prelude::*;

/// Variable length byte arrays.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Seq<T: Copy> {
    b: Vec<T>,
}

// TODO: Why ByteSeq with secret integers? Naming is odd
pub type ByteSeq = Seq<U8>;
pub type MyByteSeq = Seq<u8>;

impl<T: Copy + Default> Seq<T> {
    pub fn new() -> Self {
        Self {
            b: Vec::<T>::new()
        }
    }
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
    pub fn update<A: SeqTrait<T>>(mut self, start: usize, v: A) -> Self {
        debug_assert!(self.len() >= start + v.len());
        for (i, b) in v.iter().enumerate() {
            self[start + i] = *b;
        }
        self
    }
    pub fn update_sub<A: SeqTrait<T>>(
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
    pub fn update_element(
        mut self,
        start_out: usize,
        v: T,
    ) -> Self {
        debug_assert!(self.len() >= start_out + 1);
        self[start_out] = v;
        self
    }
    pub fn sub(self, start_out: usize, len: usize) -> Self {
        Self::from(self.b.iter().skip(start_out).map(|x| *x).take(len).collect::<Vec<T>>())
    }

    pub fn from_sub<A: SeqTrait<T>>(input: A, r: Range<usize>) -> Self {
        let mut a = Self::default();
        for (i, v) in r
            .clone()
            .zip(input.iter().skip(r.start).take(r.end - r.start))
        {
            a[i] = *v;
        }
        a
    }

    pub fn chunks(&self, chunk_size: usize) -> std::slice::Chunks<'_, T> {
        self.b.chunks(chunk_size)
    }
}

impl Seq<U8> {
    fn get_random_vec(l: usize) -> Vec<U8> {
        (0..l)
            .map(|_| rand::random::<u8>())
            .map(|x| U8::classify(x))
            .collect()
    }

    pub fn random(l: usize) -> Self {
        Self {
            b: Seq::get_random_vec(l),
        }
    }

    pub fn to_hex(&self) -> String {
        let strs: Vec<String> = self.b.iter()
                       .map(|b| format!("{:02x}", b))
                       .collect();
        strs.join("")
    }
}

impl Seq<u8> {
    pub fn to_hex(&self) -> String {
        let strs: Vec<String> = self.iter()
                       .map(|b| format!("{:02x}", b))
                       .collect();
        strs.join("")
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

impl<T: Copy> Index<u8> for Seq<T> {
    type Output = T;
    fn index(&self, i: u8) -> &T {
        &self.b[i as usize]
    }
}

impl<T: Copy> IndexMut<u8> for Seq<T> {
    fn index_mut(&mut self, i: u8) -> &mut T {
        &mut self.b[i as usize]
    }
}

impl<T: Copy> Index<u32> for Seq<T> {
    type Output = T;
    fn index(&self, i: u32) -> &T {
        &self.b[i as usize]
    }
}

impl<T: Copy> IndexMut<u32> for Seq<T> {
    fn index_mut(&mut self, i: u32) -> &mut T {
        &mut self.b[i as usize]
    }
}

impl<T: Copy> Index<i32> for Seq<T> {
    type Output = T;
    fn index(&self, i: i32) -> &T {
        &self.b[i as usize]
    }
}

impl<T: Copy> IndexMut<i32> for Seq<T> {
    fn index_mut(&mut self, i: i32) -> &mut T {
        &mut self.b[i as usize]
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

impl<T: Copy> From<Vec<T>> for Seq<T> {
    fn from(x: Vec<T>) -> Seq<T> {
        Self { b: x.clone() }
    }
}

impl<T: Copy> From<&[T]> for Seq<T> {
    fn from(x: &[T]) -> Seq<T> {
        Self { b: x.to_vec() }
    }
}

/// Read hex string to Bytes.
impl From<&str> for Seq<U8> {
    fn from(s: &str) -> Seq<U8> {
        Seq::from(
            hex_string_to_bytes(s)
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        )
    }
}
impl From<String> for Seq<U8> {
    fn from(s: String) -> Seq<U8> {
        Seq::<U8>::from(
            hex_string_to_bytes(&s)
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        )
    }
}
// TODO: duplicate code...
impl From<&str> for Seq<u8> {
    fn from(s: &str) -> Seq<u8> {
        Seq::<u8>::from(hex_string_to_bytes(s))
    }
}
impl From<String> for Seq<u8> {
    fn from(s: String) -> Seq<u8> {
        Seq::<u8>::from(hex_string_to_bytes(&s))
    }
}
