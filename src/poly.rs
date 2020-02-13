#![allow(dead_code)]

//!
//! # Polynomials
//!
//! This module implements polynomials ℤn[x]/mℤ[x].
//! Polynomials are variable sized only for now.
//!

// TODO: cleanup
use abstract_integers::Integer;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

/// Traits that have to be implemented by the type used for coefficients.
pub trait TRestrictions<T>:
    Default
    + Integer<T>
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Div<T, Output = T>
    + Add<T, Output = T>
    + Sub<T, Output = T>
    + Mul<T, Output = T>
    + Debug
{
}
impl<T> TRestrictions<T> for T where
    T: Default
        + Integer<T>
        + Copy
        + Clone
        + PartialEq
        + PartialOrd
        + Div<T, Output = T>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Debug
{
}

///! First we implement all functions on slices of T.
///! Note that this is equivalent to ℤn[x] (or ℤ[x] depending, depending on T).

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

fn leading_coefficient<T: TRestrictions<T>>(x: &[T]) -> (usize, T) {
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
fn poly_sub<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> Vec<T> {
    let (x, y) = normalize!(x, y);
    debug_assert!(x.len() == y.len());
    let mut out = vec![T::default(); x.len()];
    for (a, (&b, &c)) in out.iter_mut().zip(x.iter().zip(y.iter())) {
        // This is sub mod: `b - c % n`
        *a = b.sub_mod(c, n);
    }
    out
}

fn poly_add<T: TRestrictions<T>>(x: &[T], y: &[T]) -> Vec<T> {
    let (x, y) = normalize!(x, y);
    debug_assert!(x.len() == y.len());
    let mut out = vec![T::default(); x.len()];
    for (a, (&b, &c)) in out.iter_mut().zip(x.iter().zip(y.iter())) {
        *a = b + c;
    }
    out
}

/// Polynomial multiplication using operand scanning.
/// This is very inefficient and prone to side-channel attacks.
fn poly_mul_op_scanning<T: TRestrictions<T>>(x: &[T], y: &[T]) -> Vec<T> {
    let mut out = vec![T::default(); x.len() + y.len()];
    for i in 0..x.len() {
        for j in 0..y.len() {
            out[i + j] = out[i + j] + (x[i] * y[j]);
        }
    }
    out
}

/// Polynomial multiplication using sparse multiplication.
/// This is more efficient than operand scanning but also prone to side-channel
/// attacks. We still have coefficients in ℤn so we still need to multiply
fn poly_mul<T: TRestrictions<T>>(x: &[T], y: &[T]) -> Vec<T> {
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
            out[adx.0 + bdx.0] = out[adx.0 + bdx.0] + (*adx.1 * *bdx.1);
        }
    }
    out
}

use rand::Rng;
pub fn random_poly<T: TRestrictions<T>>(l: usize, min: i128, max: i128) -> Vec<T> {
    let mut rng = rand::thread_rng();
    (0..l)
        .map(|_| T::from_signed_literal(rng.gen_range(min, max)))
        .collect()
}

/// Euclidean algorithm to compute quotient `q` and remainder `r` of x/y.
///
/// Returns (quotient, remainder)
///
fn euclid_div<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> (Vec<T>, Vec<T>) {
    let (x, y) = normalize!(x, y);
    let mut q = vec![T::default(); x.len()];
    let mut r = x.clone();
    let (d, c) = leading_coefficient(&y);
    let (mut r_d, mut r_c) = leading_coefficient(&r);

    while r_d >= d && !is_zero(&r) {
        let idx = r_d - d;

        // r_c / c but in ℤn. So this will only work if we're in ℤn and panic otherwise.
        let c_idx = r_c * T::inv(c, n);
        debug_assert!(c_idx != T::default());

        let s = monomial(c_idx, idx);
        q = poly_add(&q[..], &s[..]);
        let sy = poly_mul(&s[..], &y[..]);
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

/// Extended euclidean algorithm to compute the inverse of x in yℤ[x]
fn extended_euclid<T: TRestrictions<T>>(x: &[T], y: &[T], n: T) -> Vec<T> {
    let (x, y) = normalize!(x, y);

    let mut new_t = vec![T::default(); x.len()];
    new_t[0] = T::from_literal(1);

    let mut new_r = x.clone();

    let mut t = vec![T::default(); x.len()];
    let mut r = y.clone();

    while !is_zero(&new_r) {
        let q = euclid_div(&r, &new_r, n).0;

        let tmp = new_r.clone();
        new_r = poly_sub(&r, &poly_mul(&q, &new_r), n);
        r = tmp;

        let tmp = new_t.clone();
        new_t = poly_sub(&t, &poly_mul(&q, &new_t), n);
        t = tmp;
    }

    poly_mul(&t, &poly_z_inv(&r, n))
}

/// The poly struct.
/// Note that while this is a polynomial over ℤn[x]/mℤ[x] it is not necessarily reduced
/// by mℤ[x], i.e. poly might be larger. Call `reduce` to make sure poly is in ℤn/mℤ.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Poly<T: TRestrictions<T>> {
    poly: Vec<T>,
    irr: Vec<T>,
    n: T,
}

impl<T: TRestrictions<T>> Poly<T> {
    fn build_from_poly(p: &[T]) -> Self {
        Self {
            poly: p.to_vec(),
            irr: Vec::<T>::new(),
            n: T::default(),
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
        v.iter().map(|x| T::from_signed_literal(*x)).collect::<Vec<T>>()
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
    // TODO: should this reduce p?
    pub fn new_monomial(irr_in: &[u128], c: T, d: usize) -> Self {
        Self {
            poly: monomial(c, d),
            irr: Self::u128_to_t(irr_in),
            n: T::default(),
        }
    }

    // TODO: should this reduce p?
    pub fn new_poly(&self, p: &[u128]) -> Self {
        Self {
            poly: Self::u128_to_t(p),
            irr: self.irr[..].to_vec(),
            n: T::default(),
        }
    }

    /// Generate random polynomial with given coefficient bounds and irreducible.
    /// Note that this requires min and max to be `i128`. For random coefficients
    /// in `T` that don't fit in `i128` use generators on `T`.
    pub fn random(irr_in: &[T], r: std::ops::Range<i128>, n_in: T) -> Self {
        Self {
            poly: random_poly(irr_in.len() - 1, r.start, r.end+1),
            irr: irr_in.to_vec(),
            n: n_in,
        }
    }

    /// Returns the leading coefficient of this polynomial and it's index.
    pub fn leading_coefficient(&self) -> (usize, T) {
        leading_coefficient(&self.poly)
    }
    /// Reduce `self`, i.e. `self.poly` by the irreducible.
    /// Returns `self.poly % self.irr`.
    pub fn reduce(&self) -> Self {
        Self {
            poly: euclid_div(&self.poly, &self.irr, self.n).1,
            irr: self.irr[..].to_vec(),
            n: T::default(),
        }
        .truncate()
    }
    /// Pad self.poly to length l with zeroes.
    pub fn pad(&self, l: usize) -> Self {
        debug_assert!(l >= self.poly.len());
        Self {
            poly: pad(&self.poly, l),
            irr: self.irr[..].to_vec(),
            n: T::default(),
        }
    }
    /// Truncate self.poly, removing trailing zeroes.
    pub fn truncate(&self) -> Self {
        Self {
            poly: truncate(&self.poly),
            irr: self.irr[..].to_vec(),
            n: T::default(),
        }
    }

    /// Euclidean division returning (q, r)
    pub fn euclid_div(&self, rhs: &Poly<T>) -> (Poly<T>, Poly<T>) {
        let (q, r) = euclid_div(&self.poly, &rhs.poly, self.n);
        (
            Self {
                poly: q,
                irr: self.irr.clone(),
                n: T::default(),
            },
            Self {
                poly: r,
                irr: self.irr.clone(),
                n: T::default(),
            },
        )
    }

    /// Invert this polynomial.
    pub fn inv(&self) -> Self {
        Self {
            poly: extended_euclid(&self.poly, &self.irr, self.n),
            irr: self.irr.clone(),
            n: T::default(),
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
        debug_assert!(self.n == rhs.n && self.n != T::default());
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
            poly: poly_add(&self.poly, &rhs.poly),
            irr: self.irr.clone(),
            n: self.n,
        }
    }
}

/// Polynomial multiplication on ℤn[x]/mℤ[x]
impl<T: TRestrictions<T>> std::ops::Mul for Poly<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert!(self.n == rhs.n);
        debug_assert!(self.irr == rhs.irr);
        Self {
            poly: poly_mul(&self.poly, &rhs.poly),
            irr: self.irr,
            n: self.n,
        }
        .reduce()
    }
}

// ℤn for `i128` and `u128`.

// #[derive(Copy, Clone)]
// pub struct Int<T: TRestrictions<T>> {
//     v: T,
//     n: T,
// }

// impl<T: TRestrictions<T>> Int<T> {
//     fn inv(&self) -> Self {
//         // FIXME
//         println!("invert {:?} mod {:?}", self.v, self.n);
//         unimplemented!()
//     }
// }

// impl Integer<Int<u128>> for Int<u128> {
//     fn from_literal(x: u128) -> Int<u128> {
//         Self {
//             v: x,
//             n: 0,
//         }
//     }
//     fn from_signed_literal(x: i128) -> Int<u128> {
//         Self {
//             v: x as u128,
//             n: 0,
//         }
//     }
//     fn inv(x: Int<u128>) -> Int<u128> {
//         x.inv()
//     }
// }

// impl Integer<Int<i128>> for Int<i128> {
//     /// **Warning** might be lossy
//     fn from_literal(x: u128) -> Int<i128> {
//         Self {
//             v: x as i128,
//             n: 0,
//         }
//     }
//     fn from_signed_literal(x: i128) -> Int<i128> {
//         Self {
//             v: x,
//             n: 0,
//         }
//     }
//     fn inv(x: Int<i128>) -> Int<i128> {
//         x.inv()
//     }
// }
