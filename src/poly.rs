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
fn poly_sub<T: TRestrictions<T>>(x: &[T], y: &[T]) -> Vec<T> {
    let (x, y) = normalize!(x, y);
    debug_assert!(x.len() == y.len());
    let mut out = vec![T::default(); x.len()];
    for (a, (&b, &c)) in out.iter_mut().zip(x.iter().zip(y.iter())) {
        *a = b - c;
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

fn poly_mul<T: TRestrictions<T>>(x: &[T], y: &[T]) -> Vec<T> {
    // TODO: implement something more efficient!
    let mut out = vec![T::default(); x.len() + y.len()];
    for i in 0..x.len() {
        for j in 0..y.len() {
            out[i + j] = out[i + j] + (x[i] * y[j]);
        }
    }
    out
}

/// Euclidean algorithm to compute quotient `q` and remainder `r` of x/y.
///
/// Returns (quotient, remainder)
fn euclid_div<T: TRestrictions<T>>(x: &[T], y: &[T]) -> (Vec<T>, Vec<T>) {
    let (x, y) = normalize!(x, y);
    let mut q = vec![T::default(); x.len()];
    let mut r = x.clone();
    let (d, c) = leading_coefficient(&y);
    let (mut r_d, mut r_c) = leading_coefficient(&r);

    while r_d >= d && !is_zero(&r) {
        let idx = r_d - d;

        // r_c / c but in ℤn. So this will only work if we're in ℤn and panic otherwise.
        let c_idx = r_c * T::inv(c);
        debug_assert!(c_idx != T::default());

        let s = monomial(c_idx, idx);
        q = poly_add(&q[..], &s[..]);
        let sy = poly_mul(&s[..], &y[..]);
        r = poly_sub(&r, &sy);

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

fn poly_z_inv<T: TRestrictions<T>>(v: &[T]) -> Vec<T> {
    v.iter().map(|&x| T::inv(x)).collect::<Vec<T>>()
}

/// Extended euclidean algorithm to compute the inverse of x in yℤ[x]
pub fn extended_euclid<T: TRestrictions<T>>(x: &[T], y: &[T]) -> Vec<T> {
    let (x, y) = normalize!(x, y);

    let mut new_t = vec![T::default(); x.len()];
    new_t[0] = T::from_literal(1);

    let mut new_r = x.clone();

    let mut t = vec![T::default(); x.len()];
    let mut r = y.clone();

    while !is_zero(&new_r) {
        let q = euclid_div(&r, &new_r).0;

        let tmp = new_r.clone();
        new_r = poly_sub(&r, &poly_mul(&q, &new_r));
        r = tmp;

        let tmp = new_t.clone();
        new_t = poly_sub(&t, &poly_mul(&q, &new_t));
        t = tmp;
    }

    poly_mul(&t, &poly_z_inv(&r))
}

/// The poly struct.
/// Note that while this is a polynomial over ℤn[x]/mℤ[x] it is not necessarily reduced
/// by mℤ[x], i.e. poly might be larger. Call `reduce` to make sure poly is in ℤn/mℤ.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Poly<T: TRestrictions<T>> {
    // TODO: make fields private
    pub poly: Vec<T>,
    pub irr: Vec<T>,
}

impl<T: TRestrictions<T>> Poly<T> {
    // TODO: should this reduce p?
    pub fn new(irr_in: &[u128], p: &[u128]) -> Self {
        let tmp_irr = irr_in
            .iter()
            .map(|x| T::from_literal(*x))
            .collect::<Vec<T>>();
        let tmp_p = p.iter().map(|x| T::from_literal(*x)).collect::<Vec<T>>();
        Self {
            poly: tmp_p,
            irr: tmp_irr,
        }
    }
    // TODO: should this reduce p?
    pub fn new_signed(irr_in: &[i128], p: &[i128]) -> Self {
        let tmp_irr = irr_in
            .iter()
            .map(|x| T::from_signed_literal(*x))
            .collect::<Vec<T>>();
        let tmp_p = p
            .iter()
            .map(|x| T::from_signed_literal(*x))
            .collect::<Vec<T>>();
        Self {
            poly: tmp_p,
            irr: tmp_irr,
        }
    }
    // TODO: should this reduce p?
    pub fn new_monomial(irr_in: &[u128], c: T, d: usize) -> Self {
        let tmp_irr = irr_in
            .iter()
            .map(|x| T::from_literal(*x))
            .collect::<Vec<T>>();
        Self {
            poly: monomial(c, d),
            irr: tmp_irr,
        }
    }

    // TODO: should this reduce p?
    pub fn new_poly(&self, p: &[u128]) -> Self {
        let tmp_p = p.iter().map(|x| T::from_literal(*x)).collect::<Vec<T>>();
        Self {
            poly: tmp_p,
            irr: self.irr[..].to_vec(),
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
            poly: euclid_div(&self.poly, &self.irr).1,
            irr: self.irr[..].to_vec(),
        }
        .truncate()
    }
    /// Pad self.poly to length l with zeroes.
    pub fn pad(&self, l: usize) -> Self {
        debug_assert!(l >= self.poly.len());
        Self {
            poly: pad(&self.poly, l),
            irr: self.irr[..].to_vec(),
        }
    }
    /// Truncate self.poly, removing trailing zeroes.
    pub fn truncate(&self) -> Self {
        Self {
            poly: truncate(&self.poly),
            irr: self.irr[..].to_vec(),
        }
    }

    /// Euclidean division returning (q, r)
    pub fn euclid_div(&self, rhs: &Poly<T>) -> (Poly<T>, Poly<T>) {
        let (q, r) = euclid_div(&self.poly, &rhs.poly);
        (
            Self {
                poly: q,
                irr: self.irr.clone(),
            },
            Self {
                poly: r,
                irr: self.irr.clone(),
            },
        )
    }

    /// Invert this polynomial.
    pub fn inv(&self) -> Self {
        Self {
            poly: extended_euclid(&self.poly, &self.irr),
            irr: self.irr.clone(),
        }
    }
}

/// Polynomial subtraction
impl<T: TRestrictions<T>> Sub for Poly<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(self.irr == rhs.irr);
        Self {
            poly: poly_sub(&self.poly, &rhs.poly),
            irr: self.irr.clone(),
        }
    }
}

/// Polynomial addition
impl<T: TRestrictions<T>> Add for Poly<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(self.irr == rhs.irr);
        Self {
            poly: poly_add(&self.poly, &rhs.poly),
            irr: self.irr.clone(),
        }
    }
}

/// Polynomial multiplication on ℤn[x]/mℤ[x]
impl<T: TRestrictions<T>> std::ops::Mul for Poly<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            poly: poly_mul(&self.poly, &rhs.poly),
            irr: self.irr,
        }
        .reduce()
    }
}
