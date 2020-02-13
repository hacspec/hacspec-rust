use hacspec::prelude::*;

unsigned_integer!(Coefficient, 256);
field_integer!(Q, Coefficient, Coefficient::from_hex("03"));

macro_rules! poly {
    ($t:ty,$i:expr,$v1:expr,$v2:expr,$e:expr) => {{
        (
            Poly::<$t>::from((&$i[..], &$v1[..])),
            Poly::<$t>::from((&$i[..], &$v2[..])),
            Poly::<$t>::from((&$i[..], &$e[..])),
        )
    }};
}

#[test]
fn test_poly_add() {
    fn test_add<T: TRestrictions<T>>(x: Poly<T>, y: Poly<T>, expected: Poly<T>) {
        let c = x.clone() + y.clone();
        println!("{:x?} + {:x?} = {:x?}", x, y, c);
        assert_eq!(c, expected);
    }

    let (a, b, e) = poly!(i128, [0], [0, 1, 1], [1, 0, 2], [1, 1, 3]);
    test_add(a, b, e);
    let (a, b, e) = poly!(i128, [0], [-1, 1, 0], [1, 0, -5], [0, 1, -5]);
    test_add(a, b, e);
    let (a, b, e) = poly!(u128, [0], [0, 1, 1], [1, 0, 2], [1, 1, 3]);
    test_add(a, b, e);
    let (a, b, e) = poly!(Q, [0], [0, 1, 1], [1, 0, 2], [1, 1, 0]);
    test_add(a, b, e);
    let (a, b, e) = poly!(Q, [0], [2, 2, 2], [2, 2, 2], [1, 1, 1]);
    test_add(a, b, e);
}

#[test]
fn test_poly_sub() {
    fn test_sub<T: TRestrictions<T>>(x: Poly<T>, y: Poly<T>, expected: Poly<T>) {
        let c = x.clone() - y.clone();
        println!("{:x?} - {:x?} = {:x?}", x, y, c);
        assert_eq!(c, expected);
    }

    let (a, b, e) = poly!(i128, [10], [0, 1, 1], [1, 0, 2], [-1, 1, -1]);
    test_sub(a, b, e);
    let (a, b, e) = poly!(i128, [10], [-1, 1, 0], [1, 0, -5], [-2, 1, 5]);
    test_sub(a, b, e);
    let (a, b, e) = poly!(u128, [10], [1, 1, 5], [1, 0, 2], [0, 1, 3]);
    test_sub(a, b, e);
    let (a, b, e) = poly!(Q, [10], [0, 1, 1], [1, 0, 2], [2, 1, 2]);
    test_sub(a, b, e);
    let (a, b, e) = poly!(Q, [10], [0, 0, 0], [2, 2, 2], [1, 1, 1]);
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
        let (c, r) = x.euclid_div(&y);
        println!("{:x?} / {:x?} = {:x?}, {:x?}", x, y, c, r);
        assert_eq!(c.truncate(), expected_c);
        assert_eq!(r.truncate(), expected_r);
    }

    let (a, b, e) = poly!(Q, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [2]);
    test_div(b, a, e, Poly::<Q>::new(&[2, 2, 0, 1], &[1, 1]));

    // FIXME: implement inv so this works as well
    // let (a, b, e) = poly!(u128, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [2]);
    // test_div(b, a, e, Poly::<u128>::new(&[2, 2, 0, 1], &[1, 1]));
}

#[test]
fn test_poly_mul() {
    fn test_mul<T: TRestrictions<T>>(x: Poly<T>, y: Poly<T>, expected: Poly<T>) {
        let c = x.clone() * y.clone();
        println!("{:x?} * {:x?} = {:x?}", x, y, c);
        assert_eq!(c, expected);
    }

    let (a, b, e) = poly!(Q, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [2, 2]);
    test_mul(a, b, e);
    let (a, b, e) = poly!(Q, [2, 2, 0, 1], [2, 2, 0], [1, 2, 2], [0, 1, 2]);
    test_mul(a, b, e);

    // let irr = random_poly::<u128>(2048, 0, 4);
    // let a = Poly::<u128>::random(&irr, 0..3, 3);
    // let b = Poly::<u128>::random(&irr, 0..3, 3);
    // let r = a.clone() * b.clone();
    // println!("{:x?} * {:x?} = {:x?}", a, b, r);

    // FIXME: implement inv so this works as well
    // let (a, b, e) = poly!(u128, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [2, 2]);
    // test_mul(a, b, e);
}

#[test]
fn test_poly_inversion() {
    let irr = [2, 2, 0, 1];
    let a = Poly::<Q>::new(&irr, &[2, 2, 0]);
    let b = Poly::<Q>::new(&irr, &[1, 2, 2]);
    let c = Poly::<Q>::new(&irr, &[0, 0, 1]);

    fn test_poly_inversion(p: Poly<Q>, irr: &[u128]) {
        let p_inv = p.inv();
        println!(" > p_inv: {:x?}", p_inv.clone());
        let test = p * p_inv;
        println!(" > (p_inv * p) % irr: {:x?}", test);
        assert_eq!(test, Poly::<Q>::new(&irr, &[1]));
    }

    test_poly_inversion(a, &irr);
    test_poly_inversion(b, &irr);
    test_poly_inversion(c, &irr);
}
