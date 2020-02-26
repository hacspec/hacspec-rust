use hacspec::prelude::*;

// FIXME: clean up
use hacspec::poly::Integer;

macro_rules! poly {
    ($t:ty,$i:expr,$v1:expr,$v2:expr,$e:expr,$n:expr) => {{
        (
            Poly::<$t>::from((&$i[..], &$v1[..], $n)),
            Poly::<$t>::from((&$i[..], &$v2[..], $n)),
            Poly::<$t>::from((&$i[..], &$e[..], $n)),
        )
    }};
}

#[test]
fn test_zn_inv() {
    let n = 65537;
    assert_eq!(u128::inv(23647, n), 52791);
    assert_eq!(u128::inv(37543865, n), 37686);
    let n = 106103;
    assert_eq!(u128::inv(8752725684352, n), 52673);
    assert_eq!(u128::inv(123, n), 99202);

    let n = 106103i128;
    assert_eq!(i128::inv(-123, n), 6901);
}

#[test]
fn test_poly_add() {
    fn test_add<T: TRestrictions<T>>(x: Poly<T>, y: Poly<T>, expected: Poly<T>) {
        let c = x.clone() + y.clone();
        println!("{:x?} + {:x?} = {:x?}", x, y, c);
        assert_eq!(c, expected);
    }

    // Polynomials without irreducible and without coefficient modulus.
    let a = Poly::<u128>::from(&[0, 1, 1][..]);
    let b = Poly::<u128>::from(&[1, 0, 2][..]);
    let e = Poly::<u128>::from(&[1, 1, 3][..]);
    test_add(a, b, e);
    let a = Poly::<i128>::from(&[-1, 1, 0][..]);
    let b = Poly::<i128>::from(&[1, 0, -5][..]);
    let e = Poly::<i128>::from(&[0, 1, -5][..]);
    test_add(a, b, e);
    let a = Poly::<u128>::from(&[0, 1, 1][..]);
    let b = Poly::<u128>::from(&[1, 0, 2][..]);
    let e = Poly::<u128>::from(&[1, 1, 3][..]);
    test_add(a, b, e);

    // Polynomials without irreducible but with coefficient modulus.
    let (a, b, e) = poly!(u128, [0], [0, 1, 1], [1, 0, 2], [1, 1, 0], 3);
    test_add(a, b, e);
    let (a, b, e) = poly!(i128, [0], [0, 1, 1], [1, 0, 2], [1, 1, 0], 3);
    test_add(a, b, e);
    let (a, b, e) = poly!(i128, [0], [-1, 1, 0], [1, 0, -5], [0, 1, 1], 3);
    test_add(a, b, e);

    // Only simple test as irreducible isn't affecting addition.
    let (a, b, e) = poly!(u128, [0, 1, 2, 3], [0, 1, 1], [1, 0, 2], [1, 1, 0], 3);
    test_add(a, b, e);
    let (a, b, e) = poly!(i128, [0, 1, 2, 3], [0, 1, 1], [1, 0, 2], [1, 1, 0], 3);
    test_add(a, b, e);
}

#[test]
fn test_poly_sub() {
    fn test_sub<T: TRestrictions<T>>(x: Poly<T>, y: Poly<T>, expected: Poly<T>) {
        let c = x.clone() - y.clone();
        println!("{:x?} - {:x?} = {:x?}", x, y, c);
        assert_eq!(c, expected);
    }

    // Polynomials without irreducible and without coefficient modulus.
    let a = Poly::<i128>::from(&[0, 1, 1][..]);
    let b = Poly::<i128>::from(&[1, 0, 2][..]);
    let e = Poly::<i128>::from(&[-1, 1, -1][..]);
    test_sub(a, b, e);
    let a = Poly::<u128>::from(&[1, 1, 5][..]);
    let b = Poly::<u128>::from(&[1, 0, 2][..]);
    let e = Poly::<u128>::from(&[0, 1, 3][..]);
    test_sub(a, b, e);

    // Polynomials without irreducible but with coefficient modulus.
    let (a, b, e) = poly!(i128, [0], [0, 1, 1], [1, 0, 2], [6, 1, 6], 7);
    test_sub(a, b, e);
    let (a, b, e) = poly!(i128, [0], [-1, 1, 0], [1, 0, -5], [253, 1, 5], 255);
    test_sub(a, b, e);
    let (a, b, e) = poly!(u128, [0], [1, 1, 5], [1, 0, 2], [0, 1, 3], 256);
    test_sub(a, b, e);

    // Only simple test as irreducible isn't affecting subtraction.
    let (a, b, e) = poly!(i128, [0, 1, 2, 3], [-1, 1, 0], [1, 0, -5], [253, 1, 5], 255);
    test_sub(a, b, e);
    let (a, b, e) = poly!(u128, [0, 1, 2, 3], [1, 1, 5], [1, 0, 2], [0, 1, 3], 256);
    test_sub(a, b, e);
}

#[test]
fn test_poly_euclid_div() {
    fn test_div<T: TRestrictions<T>>(
        x: Poly<T>,
        y: Poly<T>,
        expected_c: Poly<T>,
        expected_r: Poly<T>,
    ) {
        let (c, r) = x.clone().euclid_div(y.clone());
        println!("{:x?} / {:x?} = {:x?}, {:x?}", x, y, c, r);
        assert_eq!(c.truncate(), expected_c);
        assert_eq!(r.truncate(), expected_r);
    }

    let (a, b, e) = poly!(u128, [2, 2, 0, 1], [1, 0, 2], [0, 1, 1], [2], 3);
    let remainder = Poly::<u128>::new_full(&[2, 2, 0, 1], &[1, 1], 3);
    test_div(a, b, e, remainder);

    let (a, b, e) = poly!(u128, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [6], 11);
    let remainder = Poly::<u128>::new_full(&[2, 2, 0, 1], &[5, 1], 11);
    test_div(a, b, e, remainder);
}

#[test]
fn test_poly_mul() {
    fn test_mul<T: TRestrictions<T>>(x: Poly<T>, y: Poly<T>, expected: Poly<T>) {
        let c = x.clone() * y.clone();
        println!("{:x?} * {:x?} = {:x?}", x, y, c);
        assert_eq!(c, expected);
    }

    let (a, b, e) = poly!(u128, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [7, 4, 8], 11);
    test_mul(a, b, e);
    let (a, b, e) = poly!(i128, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [7, 4, 8], 11);
    test_mul(a, b, e);
    let (a, b, e) = poly!(i128, [2, 2, 0, 1], [-3, 5, -1], [1, -2, -7], [8, 8, 7], 11);
    test_mul(a, b, e);

    // Use random values, so no expected value possible here.
    let irr = random_poly::<u128>(2048, 0, 3);
    let a = Poly::<u128>::random(irr.clone(), 0..3, 3);
    let b = Poly::<u128>::random(irr.clone(), 0..3, 3);
    let r = a.clone() * b.clone();
    println!("{:x?} * {:x?} = {:x?}", a, b, r);
}

#[test]
fn test_poly_inversion() {
    let irr = [2, 2, 0, 1];
    let a = Poly::<u128>::new_full(&irr, &[2, 2, 0], 3);
    let b = Poly::<u128>::new_full(&irr, &[1, 2, 2], 3);
    let c = Poly::<u128>::new_full(&irr, &[0, 0, 1], 3);

    fn test_poly_inversion(p: Poly<u128>, irr: &[u128]) {
        let p_inv = p.clone().inv();
        println!(" > p: {:x?}", p.clone());
        println!(" > p_inv: {:x?}", p_inv.clone());
        let test = p * p_inv;
        println!(" > (p_inv * p) % irr: {:x?}", test);
        assert_eq!(test, Poly::<u128>::new_full(&irr, &[1], 3));
    }

    test_poly_inversion(a, &irr);
    test_poly_inversion(b, &irr);
    test_poly_inversion(c, &irr);

    // let irr = random_poly::<u128>(2048, 0, 3);
    // let a = Poly::<u128>::random(&irr, 0..3, 3);
    // let b = Poly::<u128>::random(&irr, 0..3, 3);
    // test_poly_inversion(a, &irr);
    // test_poly_inversion(b, &irr);
}

#[test]
#[should_panic]
fn test_poly_inversion_panic() {
    // Not invertible
    let irr = [2, 2, 1, 2, 2, 1, 2, 0, 2, 0, 2, 2];
    let a = Poly::<u128>::new_full(&irr, &[0, 1, 2, 0, 2, 2, 0, 0, 2, 0, 0], 3);
    let a_inv = a.inv();
}

#[test]
fn test_poly_ops() {
    let irr = [2, 2, 0, 1];
    let x = Poly::<u128>::new_full(&irr, &[2, 2, 1], 3);
    let y = Poly::<u128>::new_full(&irr, &[1, 2, 0], 3);

    let z = x.clone() + y.clone();
    let z = z.clone() - x.clone();
    assert_eq!(z.truncate(), y.truncate());

    let z = x.clone() * y.clone();
    println!("{:x?} * {:x?} = {:x?}", x.clone(), y.clone(), z.clone());
    assert_eq!(z.truncate(), Poly::<u128>::new_full(&irr, &[1, 2, 2], 3));

    let (zq, zr) = x.clone() / y.clone();
    println!(
        "{:x?} / {:x?} = {:x?}; {:x?}",
        x.clone(),
        y.clone(),
        zq.clone(),
        zr.clone()
    );
    assert_eq!(zr.truncate(), Poly::<u128>::new_full(&irr, &[2], 3));
    assert_eq!(zq.truncate(), Poly::<u128>::new_full(&irr, &[0, 2], 3));
}

#[test]
fn test_poly_ops_doc() {
    let x = Poly::<u128>::from_array(&[5, 2, 7, 8, 9], 11); // 5 + 2x + 7x² + 8x³ + 9x⁴
    let y = Poly::<u128>::from_array(&[2, 1, 0, 2, 4], 11); // 2 + 1x       + 2x³ + 4x⁴
    let z = x.clone() * y.clone();
    assert_eq!(
        z.truncate(),
        Poly::<u128>::from_array(&[10, 9, 5, 0, 6, 9, 0, 6, 3], 11)
    );
    let z = x.clone() + y.clone();
    assert_eq!(z, Poly::<u128>::from_array(&[7, 3, 7, 10, 2], 11));
    let z = x.clone() - y.clone();
    assert_eq!(z, Poly::<u128>::from_array(&[3, 1, 7, 6, 5], 11));
    let (q, r) = x.clone() / y.clone();
    assert_eq!(q.truncate(), Poly::<u128>::from_array(&[5], 11));
    assert_eq!(r.truncate(), Poly::<u128>::from_array(&[6, 8, 7, 9], 11));

    // Now over ℤn/mℤ[x]
    let irr = [1, 3, 5, 0, 8, 6];
    let x = Poly::<u128>::new_full(&irr, &[5, 2, 7, 8, 9], 11); // 5 + 2x + 7x² + 8x³ + 9x⁴
    let y = Poly::<u128>::new_full(&irr, &[2, 1, 0, 2, 4], 11); // 2 + 1x       + 2x³ + 4x⁴
    let z = x.clone() * y.clone();
    assert_eq!(
        z.truncate(),
        Poly::<u128>::new_full(&irr, &[7, 9, 2, 5, 10], 11)
    );
    let z = x.clone() + y.clone();
    assert_eq!(z, Poly::<u128>::new_full(&irr, &[7, 3, 7, 10, 2], 11));
    let z = x.clone() - y.clone();
    assert_eq!(z, Poly::<u128>::new_full(&irr, &[3, 1, 7, 6, 5], 11));
    let (q, r) = x.clone() / y.clone();
    assert_eq!(q.truncate(), Poly::<u128>::new_full(&irr, &[5], 11));
    assert_eq!(r.truncate(), Poly::<u128>::new_full(&irr, &[6, 8, 7, 9], 11));
}

#[test]
fn test_poly_factory() {
    poly_n!(R3, u128, 3);
    let x = R3::new(&[1, 2, 0, 1]);
    println!("x: {:?}", x);

    poly_n_m!(R3q, u128, 3, &[1, 2, 0, 1]);
    let x = R3q::new(&[1, 2, 0, 1]);
    println!("x: {:?}", x);
}
