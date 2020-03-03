#![allow(dead_code)]

//!
//! # Polynomials
//!
//! This module implements polynomials ℤn[x]/mℤ[x].
//! Polynomials are variable sized only for now.
//!
//! Coefficients are currently stored as u128 or i128.
//!
//! **TODO: If necessary, we could extend the definition to larger integers.**
//!
//! Three different types of polynomials are supported:
//! * Polynomial rings over ℤ
//! * Quotient ring over ℤn
//! * Quotient ring over ℤn/mℤ
//!
//! # Polynomial rings over ℤ
//! This most basic form is implemented over basic sequences `Seq<T>`.
//! Addition, Subtraction, Multiplication, and Division with remainder are supported.
//!
//! **Note:** This is currently only implemented for `Seq<u128>` and `Seq<i128>`.
//!
//! ## Examples
//!
//! ```
//! use hacspec::prelude::*;
//!
//! let x = Seq::<u128>::from_array(&[5, 2, 7, 8, 9]); // 5 + 2x + 7x² + 8x³ + 9x⁴
//! let y = Seq::<u128>::from_array(&[2, 1, 0, 2, 4]); // 2 + 1x       + 2x³ + 4x⁴
//! let z = x.clone() * y.clone();
//! let z = x.clone() + y.clone();
//! let z = x.clone() - y.clone();
//! // You can not divide polynomials over ℤ. This will panic.
//! // let (q, r) = x.clone() / y.clone();
//! ```
//!
//! # Quotient ring over ℤn
//! Quotient ring arithmetic is implemented on a separate type `Poly<T>`, which
//! behaves similar to sequences but can store the modulus `n` as well.
//! Addition, Subtraction, Multiplication, and Division with remainder are supported.
//!
//! **Note:** This is currently only implemented for `Poly<u128>` and `Poly<i128>`.
//!
//! ## Examples
//!
//! ```
//! use hacspec::prelude::*;
//!
//! let x = Poly::<u128>::from_array(&[5, 2, 7, 8, 9], 11); // 5 + 2x + 7x² + 8x³ + 9x⁴
//! let y = Poly::<u128>::from_array(&[2, 1, 0, 2, 4], 11); // 2 + 1x       + 2x³ + 4x⁴
//! let z = x.clone() * y.clone();
//! let z = x.clone() + y.clone();
//! let z = x.clone() - y.clone();
//! let (q, r) = x.clone() / y.clone();
//! ```
//!
//! # Quotient ring over ℤn/mℤ[x]
//! Quotient ring arithmetic is implemented on a separate type `Poly<T>`, which
//! behaves similar to sequences but can store the modulus `n` and irreducible
//! polynomial `irr` as well.
//! Addition, Subtraction, Multiplication, and Division with remainder are supported.
//!
//! **Note:** This is currently only implemented for `Poly<u128>` and `Poly<i128>`.
//!
//! ## Examples
//!
//! ```
//! use hacspec::prelude::*;
//!
//! let irr = [1, 3, 5, 0, 8, 6];
//! let x = Poly::<u128>::new_full(&irr, &[5, 2, 7, 8, 9], 11); // 5 + 2x + 7x² + 8x³ + 9x⁴
//! let y = Poly::<u128>::new_full(&irr, &[2, 1, 0, 2, 4], 11); // 2 + 1x       + 2x³ + 4x⁴
//! let z = x.clone() * y.clone();
//! let z = x.clone() + y.clone();
//! let z = x.clone() - y.clone();
//! let (q, r) = x.clone() / y.clone();
//! ```
//!

use std::ops::{Add, Div, Mul, Sub};
use rand::Rng;

use crate::seq::*;
use crate::integer::*;

///! First we implement all functions on slices of T.
///! Note that this is equivalent to ℤn[x] (or ℤ[x] depending, depending on T).

/// Rust's built-in modulo (x % n) is signed. This lifts x into ℤn+.
pub(crate) fn signed_mod(x: i128, n: i128) -> i128 {
    let mut ret = x % n;
    while ret < 0 {
        ret += n;
    }
    ret
}

fn pad<T: TRestrictions<T>>(v: &[T], l: usize) -> Vec<T> {
    let mut out = v.to_vec();
    for _ in out.len()..l {
        out.push(T::default());
    }
    out
}

fn truncate<T: TRestrictions<T>>(v: &[T]) -> Vec<T> {
    let (d, c) = leading_coefficient(v);
    println!("d: {:?}, c: {:x?}", d, c);
    let mut out = vec![T::default(); d + 1];
    for (a, &b) in out.iter_mut().zip(v.iter()) {
        *a = b;
    }
    out
}

fn monomial<T>(c: T, d: usize) -> Vec<T>
where
    T: TRestrictions<T>,
{
    let mut p = vec![T::default(); d + 1];
    p[d] = c;
    p
}

macro_rules! normalize {
    ($x:expr, $y:expr) => {{
        let max_len = std::cmp::max($x.len(), $y.len());
        (pad($x, max_len), pad($y, max_len))
    }};
}

pub fn leading_coefficient<T: TRestrictions<T>>(x: &[T]) -> (usize, T) {
    let zero = T::default();
    let mut degree: usize = 0;
    let mut coefficient = T::default();
    for (i, &c) in x.iter().enumerate() {
        if c != zero {
            degree = i;
            coefficient = c;
        }
    }
    (degree, coefficient)
}
pub fn poly_sub<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> Vec<T> {
    let (x, y) = normalize!(x, y);
    debug_assert!(x.len() == y.len());
    let mut out = vec![T::default(); x.len()];
    for (a, (&b, &c)) in out.iter_mut().zip(x.iter().zip(y.iter())) {
        *a = b.sub_mod(c, n);
    }
    out
}

pub fn poly_add<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> Vec<T> {
    let (x, y) = normalize!(x, y);
    debug_assert!(x.len() == y.len());
    let mut out = vec![T::default(); x.len()];
    for (a, (&b, &c)) in out.iter_mut().zip(x.iter().zip(y.iter())) {
        *a = b.add_mod(c, n);
    }
    out
}

/// Polynomial multiplication using operand scanning without modulo.
pub(crate) fn poly_mul_plain<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> Vec<T> {
    let mut out = vec![T::default(); x.len() + y.len()];
    for i in 0..x.len() {
        for j in 0..y.len() {
            out[i + j] = out[i + j] + x[i] * y[j];
        }
    }
    out
}

/// Polynomial multiplication using operand scanning.
/// This is very inefficient and prone to side-channel attacks.
pub(crate) fn poly_mul_op_scanning<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> Vec<T> {
    let mut out = vec![T::default(); x.len() + y.len()];
    for i in 0..x.len() {
        for j in 0..y.len() {
            out[i + j] = out[i + j].add_mod(x[i].mul_mod(y[j], n), n);
        }
    }
    out
}

/// Polynomial multiplication using sparse multiplication.
/// This can be more efficient than operand scanning but also prone to side-channel
/// attacks.
pub fn poly_mul<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> Vec<T> {
    let mut out = vec![T::default(); x.len() + y.len()];
    for adx in x
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x))
        .filter(|(_, &x)| x != T::default())
    {
        for bdx in y
            .iter()
            .enumerate()
            .map(|(i, x)| (i, x))
            .filter(|(_, &x)| x != T::default())
        {
            out[adx.0 + bdx.0] = out[adx.0 + bdx.0].add_mod(adx.1.mul_mod(*bdx.1, n), n);
        }
    }
    out
}

pub fn random_poly<T: TRestrictions<T>>(l: usize, min: i128, max: i128) -> Seq<T> {
    let mut rng = rand::thread_rng();
    (0..l)
        .map(|_| T::from_signed_literal(rng.gen_range(min, max)))
        .collect::<Vec<T>>()
        .into()
}

/// Euclidean algorithm to compute quotient `q` and remainder `r` of x/y.
///
/// Returns (quotient, remainder)
///
/// **Panics** when division isn't possible.
///
pub fn euclid_div<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> (Vec<T>, Vec<T>) {
    let (x, y) = normalize!(x, y);
    let mut q = vec![T::default(); x.len()];
    let mut r = x.clone();
    let (d, c) = leading_coefficient(&y);
    let (mut r_d, mut r_c) = leading_coefficient(&r);

    while r_d >= d && !is_zero(&r) {
        let idx = r_d - d;

        let c_idx = if n == T::default() {
            // In ℤ we try this. It might not work.
            r_c / c
        } else {
            // r_c / c in ℤn is r_c * 1/c.
            r_c * T::inv(c, n)
        };
        if c_idx == T::default() {
            panic!("c_idx is 0; can't divide these two polynomials");
        }

        let s = monomial(c_idx, idx);
        q = poly_add(&q[..], &s[..], n);
        let sy = poly_mul(&s[..], &y[..], n);
        r = poly_sub(&r, &sy, n);

        let tmp = leading_coefficient(&r);
        r_d = tmp.0;
        r_c = tmp.1;
    }

    (q, r)
}

fn is_zero<T: TRestrictions<T>>(v: &[T]) -> bool {
    for &x in v {
        if x != T::default() {
            return false;
        }
    }
    return true;
}

fn poly_z_inv<T: TRestrictions<T>>(v: &[T], n: T) -> Vec<T> {
    v.iter().map(|&x| T::inv(x, n)).collect::<Vec<T>>()
}

/// Extended euclidean algorithm to compute the inverse of x in ℤ/n
///
/// **Panics** if x is not invertible.
///
pub(crate) fn extended_euclid_invert<T: TRestrictions<T>>(x: T, n: T, signed: bool) -> T {
    let mut t = T::default();
    let mut r = n;
    let mut new_t = T::from_literal(1);
    let mut new_r = x;

    while new_r != T::default() {
        let q: T = r / new_r;

        let tmp = new_r.clone();
        new_r = r.sub_lift(q * new_r, n);
        r = tmp;

        let tmp = new_t.clone();
        new_t = t.sub_lift(q * new_t, n);
        t = tmp;
    }

    if r > T::from_literal(1) && x != T::default() {
        panic!("{:x?} is not invertible in ℤ/{:x?}", x, n);
    }
    println!("{:?}", t);
    if t < T::default() {
        if signed {
            t = t.abs()
        } else {
            t = t + n
        };
    };

    t
}

/// Extended euclidean algorithm to compute the inverse of x in yℤ[x]
fn extended_euclid<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> Result<Vec<T>, &'static str> {
    let (x, y) = normalize!(x, y);

    let mut new_t = vec![T::default(); x.len()];
    new_t[0] = T::from_literal(1);

    let mut new_r = x.clone();

    let mut t = vec![T::default(); x.len()];
    let mut r = y.clone();

    while !is_zero(&new_r) {
        let q = euclid_div(&r, &new_r, n).0;

        let tmp = new_r.clone();
        new_r = poly_sub(&r, &poly_mul(&q, &new_r, n), n);
        r = tmp;

        let tmp = new_t.clone();
        new_t = poly_sub(&t, &poly_mul(&q, &new_t, n), n);
        t = tmp;
    }

    if leading_coefficient(&r).0 > 0 {
        return Err("Could not invert the polynomial");
    }

    Ok(poly_mul(&t, &poly_z_inv(&r, n), n))
}

/// The poly struct.
/// Note that while this is a polynomial over ℤn[x]/mℤ[x] it is not necessarily reduced
/// by mℤ[x], i.e. poly might be larger. Call `reduce` to make sure poly is in ℤn/mℤ.
///
/// Use arithmetic operations on `Seq<T>` to use non-quotient rings.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Poly<T: TRestrictions<T>> {
    poly: Vec<T>,
    irr: Vec<T>,
    /// `n` is set to 0 if not specified and ignored.
    n: T,
}

#[macro_export]
macro_rules! poly_n {
    ($name:ident, $t:ty, $l: expr, $n:expr) => {
        struct $name;
        impl $name {
            fn new(p: &[(usize, u128)]) -> Poly<$t> {
                Poly::sparse($l, p, $n)
            }
        }
    };
}

#[macro_export]
macro_rules! poly_n_m {
    ($name:ident, $t:ty, $l:expr, $n:expr, $m:expr) => {
        struct $name;
        impl $name {
            fn new(p: &[(usize, u128)]) -> Poly<$t> {
                Poly::sparse_full($l, p, $n, $m)
            }
            fn random() -> Poly<$t> {
                // TODO: this can panic; also not sure if it's exactly what we want.
                Poly::sparse_random($l, 0..$n, $n, $m)
            }
        }
    };
}

#[macro_export]
macro_rules! poly {
    ($name:ident, $t:ty, $l:expr, $n:expr, $m:expr) => {
        /// The poly struct for fixed-length polynomials.
        /// Every polynomial is over ℤn[x]/mℤ[x] and reduced by mℤ[x].
        #[derive(Clone, Copy)]
        struct $name {
            poly: [$t; $l],
            irr: [$t; $l+1],
            n: $t,
        }
        impl $name {
            /// Get a new sparse polynomial.
            /// For other polynomials use `new_full`.
            fn new(p: &[(usize, $t)]) -> $name {
                let mut poly = [<$t>::default(); $l];
                for c in p.iter() {
                    poly[c.0] = c.1;
                }
                let mut irr = [<$t>::default(); $l+1];
                for c in $m.iter() {
                    irr[c.0] = c.1;
                }
                Self {
                    poly: poly,
                    irr: irr,
                    n: $n,
                }
            }
            /// Get a new polynomial from a full array with coefficients.
            fn new_full(p: [$t; $l]) -> $name {
                let mut irr = [<$t>::default(); $l+1];
                for c in $m.iter() {
                    irr[c.0] = c.1;
                }
                Self {
                    poly: p,
                    irr: irr,
                    n: $n,
                }
            }
            /// Generate a random polynomial with coefficients between 0 and $n.
            fn random() -> $name {
                let mut irr = [<$t>::default(); $l+1];
                for c in $m.iter() {
                    irr[c.0] = c.1;
                }
                let mut rng = rand::thread_rng();
                let p_vec: Vec<$t> = (0..$l)
                    .map(|_| rng.gen_range(0, $n))
                    .collect();
                let mut p = [<$t>::default(); $l];
                for (a, b) in p.iter_mut().zip(p_vec.iter()) {
                    *a = *b;
                }
                Self {
                    poly: p,
                    irr: irr,
                    n: $n,
                }
            }
            /// Check if the two polynomials are defined over the same ring.
            /// **Note** This shouldn't work on secret integers.
            fn compatible(&self, other: &Self) -> bool {
                if self.n != other.n {
                    return false;
                }
                if self.irr.len() != other.irr.len() {
                    return false;
                }
                if self.poly.len() != other.poly.len() {
                    // This should be unreachable.
                    return false;
                }
                for (a, b) in self.irr.iter().zip(other.irr.iter()) {
                    if a != b {
                        return false;
                    }
                }
                return true;
            }
        }

        impl From<Vec<$t>> for $name {
            fn from(v: Vec<$t>) -> $name {
                let (d, _) = leading_coefficient(&v);
                debug_assert!(d <= $l);
                if d > $l {
                    panic!("The vector is too long to fit this polynomial.");
                }
                let mut p = [<$t>::default(); $l];
                for (a, b) in p.iter_mut().zip(v.iter()) {
                    *a = *b;
                }
                let mut irr = [<$t>::default(); $l+1];
                for c in $m.iter() {
                    irr[c.0] = c.1;
                }
                $name {
                    poly: p,
                    irr: irr,
                    n: $n
                }
            }
        }

        impl fmt::Debug for $name {
            // TODO: print irr and n as well
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.poly
                    .iter()
                    .collect::<Vec<_>>()
                    .fmt(f)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                if !self.compatible(other) {
                    return false;
                }
                for (a, b) in self.irr.iter().zip(other.irr.iter()) {
                    if a != b {
                        return false;
                    }
                }
                return true;
            }
        }

        /// Polynomial subtraction
        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                debug_assert!(self.compatible(&rhs));
                let r = poly_sub(&self.poly, &rhs.poly, self.n);
                Self::from(r)
            }
        }

        /// Polynomial addition
        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                debug_assert!(self.compatible(&rhs));
                let r = poly_add(&self.poly, &rhs.poly, self.n);
                Self::from(r)
            }
        }

        /// Polynomial multiplication on ℤn[x]/mℤ[x]
        impl Mul for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                debug_assert!(self.compatible(&rhs));
                let tmp = poly_mul(&self.poly, &rhs.poly, self.n);
                let r = euclid_div(&tmp, &self.irr, self.n).1;
                Self::from(r)
            }
        }

        /// Polynomial division on ℤn[x]/mℤ[x]
        impl Div for $name {
            type Output = (Self, Self);
            fn div(self, rhs: Self) -> Self::Output {
                debug_assert!(self.compatible(&rhs));
                let r = euclid_div(&self.poly, &rhs.poly, self.n);
                (Self::from(r.0), Self::from(r.1))
            }
        }
    };
}

// FIXME: clean-up!
impl<T: TRestrictions<T>> Poly<T> {
    pub fn sparse(l: usize, p: &[(usize, u128)], n: u128) -> Self {
        Self::sparse_full(l, p, n, &[])
    }
    pub fn sparse_full(l: usize, p: &[(usize, u128)], n: u128, irr_in: &[(usize, u128)]) -> Self {
        let mut poly = vec![T::default(); l];
        for c in p.iter() {
            poly[c.0] = T::from_literal(c.1);
        }
        let mut irr = vec![T::default(); l+1];
        for c in irr_in.iter() {
            irr[c.0] = T::from_literal(c.1);
        }
        Self {
            poly: poly,
            irr: irr,
            n: T::from_literal(n),
        }
    }
    pub fn sparse_random(l: usize, r: std::ops::Range<i128>, n: u128, irr_in: &[(usize, u128)]) -> Self {
        let mut irr = vec![T::default(); l+1];
        for c in irr_in.iter() {
            irr[c.0] = T::from_literal(c.1);
        }
        Self {
            poly: random_poly(l, r.start, r.end).b,
            irr: irr,
            n: T::from_literal(n),
        }
    }
    pub fn from_array(p: &[u128], n: u128) -> Self {
        Self {
            poly: Self::u128_to_t(p),
            irr: Self::u128_to_t(&[]),
            n: T::from_literal(n),
        }
    }

    fn build_from_irr_poly(irr_in: &[T], p: &[T]) -> Self {
        Self {
            poly: p.to_vec(),
            irr: irr_in.to_vec(),
            n: T::default(),
        }
    }
    fn build(irr_in: &[T], p: &[T], n_in: &T) -> Self {
        Self {
            poly: p.to_vec(),
            irr: irr_in.to_vec(),
            n: *n_in,
        }
    }

    fn u128_to_t(v: &[u128]) -> Vec<T> {
        v.iter().map(|x| T::from_literal(*x)).collect::<Vec<T>>()
    }

    fn i128_to_t(v: &[i128]) -> Vec<T> {
        v.iter()
            .map(|x| T::from_signed_literal(*x))
            .collect::<Vec<T>>()
    }

    // FIXME: fix the horrible naming and API
    // TODO: should this reduce p and n?
    pub fn new_full(irr_in: &[u128], p: &[u128], n_in: u128) -> Self {
        Self {
            poly: Self::u128_to_t(p),
            irr: Self::u128_to_t(irr_in),
            n: T::from_literal(n_in),
        }
    }
    // TODO: should this reduce p?
    pub fn new(irr_in: &[u128], p: &[u128]) -> Self {
        Self {
            poly: Self::u128_to_t(p),
            irr: Self::u128_to_t(irr_in),
            n: T::default(),
        }
    }
    // TODO: should this reduce p?
    pub fn new_signed(irr_in: &[i128], p: &[i128]) -> Self {
        Self {
            poly: Self::i128_to_t(p),
            irr: Self::i128_to_t(irr_in),
            n: T::default(),
        }
    }
    // TODO: should this reduce p and n?
    pub fn new_signed_full(irr_in: &[i128], p: &[i128], n_in: i128) -> Self {
        Self {
            poly: Self::i128_to_t(p),
            irr: Self::i128_to_t(irr_in),
            n: T::from_signed_literal(n_in),
        }
    }
    // TODO: should this reduce p?
    pub fn new_monomial(irr_in: &[u128], c: T, d: usize) -> Self {
        Self {
            poly: monomial(c, d),
            irr: Self::u128_to_t(irr_in),
            n: T::default(),
        }
    }

    // TODO: should this reduce p?
    pub fn new_poly(self, p: &[u128]) -> Self {
        Self {
            poly: Self::u128_to_t(p),
            irr: self.irr[..].to_vec(),
            n: self.n,
        }
    }

    /// Generate random polynomial with given coefficient bounds and irreducible.
    /// Note that this requires min and max to be `i128`. For random coefficients
    /// in `T` that don't fit in `i128` use generators on `T`.
    pub fn random(irr_in: Seq<T>, r: std::ops::Range<i128>, n_in: T) -> Self {
        Self {
            poly: random_poly(irr_in.len() - 1, r.start, r.end).b,
            irr: irr_in.b,
            n: n_in,
        }
    }

    /// Returns the leading coefficient of this polynomial and it's index.
    pub fn leading_coefficient(self) -> (usize, T) {
        leading_coefficient(&self.poly)
    }
    /// Reduce `self`, i.e. `self.poly` by the irreducible.
    /// Returns `self.poly % self.irr`.
    pub fn reduce(self) -> Self {
        Self {
            poly: euclid_div(&self.poly, &self.irr, self.n).1,
            irr: self.irr[..].to_vec(),
            n: self.n,
        }
        .truncate()
    }
    /// Pad self.poly to length l with zeroes.
    pub fn pad(self, l: usize) -> Self {
        debug_assert!(l >= self.poly.len());
        Self {
            poly: pad(&self.poly, l),
            irr: self.irr[..].to_vec(),
            n: self.n,
        }
    }
    /// Truncate self.poly, removing trailing zeroes.
    pub fn truncate(&self) -> Self {
        Self {
            poly: truncate(&self.poly),
            irr: self.irr[..].to_vec(),
            n: self.n,
        }
    }
    /// Returns `true` if this is an all-zero polynomial.
    pub fn is_zero(self) -> bool {
        is_zero(&self.poly)
    }

    /// Euclidean division returning (q, r)
    pub fn euclid_div(self, rhs: Poly<T>) -> (Poly<T>, Poly<T>) {
        let (q, r) = euclid_div(&self.poly, &rhs.poly, self.n);
        (
            Self {
                poly: q,
                irr: self.irr.clone(),
                n: self.n,
            },
            Self {
                poly: r,
                irr: self.irr.clone(),
                n: self.n,
            },
        )
    }

    /// Invert this polynomial.
    pub fn inv(self) -> Self {
        Self {
            poly: extended_euclid(&self.poly, &self.irr, self.n).unwrap(),
            irr: self.irr.clone(),
            n: self.n,
        }
    }
}

macro_rules! impl_from {
    ($t:ty) => {
        impl<T: TRestrictions<T>> From<&[$t]> for Poly<T> {
            fn from(v: &[$t]) -> Poly<T> {
                Poly::new(
                    &[0],
                    &v.iter().map(|&x| u128::from(x)).collect::<Vec<u128>>(),
                )
            }
        }
        impl<T: TRestrictions<T>> From<(&[$t], &[$t])> for Poly<T> {
            fn from(v: (&[$t], &[$t])) -> Poly<T> {
                Poly::new(
                    &v.0.iter().map(|&x| u128::from(x)).collect::<Vec<u128>>(),
                    &v.1.iter().map(|&x| u128::from(x)).collect::<Vec<u128>>(),
                )
            }
        }
        impl<T: TRestrictions<T>> From<(&[$t], &[$t], $t)> for Poly<T> {
            fn from(v: (&[$t], &[$t], $t)) -> Poly<T> {
                Poly::new_full(
                    &v.0.iter().map(|&x| u128::from(x)).collect::<Vec<u128>>(),
                    &v.1.iter().map(|&x| u128::from(x)).collect::<Vec<u128>>(),
                    u128::from(v.2),
                )
            }
        }
    };
}

macro_rules! impl_from_signed {
    ($t:ty) => {
        impl<T: TRestrictions<T>> From<&[$t]> for Poly<T> {
            fn from(v: &[$t]) -> Poly<T> {
                Poly::new_signed(
                    &[0],
                    &v.iter().map(|&x| i128::from(x)).collect::<Vec<i128>>(),
                )
            }
        }
        impl<T: TRestrictions<T>> From<(&[$t], &[$t])> for Poly<T> {
            fn from(v: (&[$t], &[$t])) -> Poly<T> {
                Poly::new_signed(
                    &v.0.iter().map(|&x| i128::from(x)).collect::<Vec<i128>>(),
                    &v.1.iter().map(|&x| i128::from(x)).collect::<Vec<i128>>(),
                )
            }
        }
        impl<T: TRestrictions<T>> From<(&[$t], &[$t], $t)> for Poly<T> {
            fn from(v: (&[$t], &[$t], $t)) -> Poly<T> {
                Poly::new_signed_full(
                    &v.0.iter().map(|&x| i128::from(x)).collect::<Vec<i128>>(),
                    &v.1.iter().map(|&x| i128::from(x)).collect::<Vec<i128>>(),
                    i128::from(v.2),
                )
            }
        }
    };
}

impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(u128);

impl_from_signed!(i8);
impl_from_signed!(i16);
impl_from_signed!(i32);
impl_from_signed!(i64);
impl_from_signed!(i128);

/// Polynomial subtraction
impl<T: TRestrictions<T>> Sub for Poly<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(self.irr == rhs.irr);
        debug_assert!(self.n == rhs.n);
        Self {
            poly: poly_sub(&self.poly, &rhs.poly, self.n),
            irr: self.irr.clone(),
            n: self.n,
        }
    }
}

/// Polynomial addition
impl<T: TRestrictions<T>> Add for Poly<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(self.irr == rhs.irr);
        debug_assert!(self.n == rhs.n);
        Self {
            poly: poly_add(&self.poly, &rhs.poly, self.n),
            irr: self.irr.clone(),
            n: self.n,
        }
    }
}

/// Polynomial multiplication on ℤn[x]/mℤ[x]
impl<T: TRestrictions<T>> Mul for Poly<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert!(self.n == rhs.n);
        debug_assert!(self.irr == rhs.irr);
        let tmp = poly_mul(&self.poly, &rhs.poly, self.n);
        let r = Self {
            poly: tmp,
            irr: self.irr,
            n: self.n,
        };
        if r.irr.is_empty() {
            r
        } else {
            r.reduce()
        }
    }
}

/// Polynomial division on ℤn[x]/mℤ[x]
/// The result is reduced modulo mℤ[x] only if present.
impl<T: TRestrictions<T>> Div for Poly<T> {
    type Output = (Self, Self);
    fn div(self, rhs: Self) -> Self::Output {
        debug_assert!(self.n == rhs.n);
        debug_assert!(self.irr == rhs.irr);
        let r = euclid_div(&self.poly, &rhs.poly, self.n);
        let r = if self.irr.is_empty() {
            r
        } else {
            // TODO: do we ever want this?
            (
                euclid_div(&r.0, &self.irr, self.n).1,
                euclid_div(&r.1, &self.irr, self.n).1,
            )
        };
        (
            Self {
                poly: r.0,
                irr: self.irr.clone(),
                n: self.n,
            },
            Self {
                poly: r.1,
                irr: self.irr,
                n: self.n,
            },
        )
    }
}

/// Polynomial multiplication on ℤ[x]
impl<T: TRestrictions<T>> Mul for Seq<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            b: poly_mul(&self.b, &rhs.b, T::default()),
            idx: 0,
        }
    }
}

/// Polynomial subtraction on ℤ[x]
impl<T: TRestrictions<T>> Sub for Seq<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            b: poly_sub(&self.b, &rhs.b, T::default()),
            idx: 0,
        }
    }
}

/// Polynomial addition on ℤ[x]
impl<T: TRestrictions<T>> Add for Seq<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            b: poly_add(&self.b, &rhs.b, T::default()),
            idx: 0,
        }
    }
}

/// Polynomial division on ℤ[x]
impl<T: TRestrictions<T>> Div for Seq<T> {
    type Output = (Self, Self);
    fn div(self, rhs: Self) -> Self::Output {
        let r = euclid_div(&self.b, &rhs.b, T::default());
        (Self { b: r.0, idx: 0 }, Self { b: r.1, idx: 0 })
    }
}
