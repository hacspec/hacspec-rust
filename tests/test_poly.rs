use hacspec::prelude::*;

unsigned_integer!(Coefficient, 256);
field_integer!(
    Q,
    Coefficient,
    Coefficient::from_hex("03")
);

#[test]
fn test_poly_add() {
    let irr = [2, 2, 0, 1];
    let a = Poly::<u128>::new(&irr, &[0, 1, 1]);
    let b = Poly::<u128>::new(&irr, &[1, 0, 2]);
    let c = a.clone() + b.clone();
    println!("{:x?} + {:x?} = {:x?}", a, b, c);
    assert_eq!(c, Poly::<u128>::new(&irr, &[1, 1, 3]));

    let irr = [2, 2, 0, 1];
    let a = Poly::<Q>::new(&irr, &[0, 1, 1]);
    let b = Poly::<Q>::new(&irr, &[1, 0, 2]);
    let c = a.clone() + b.clone();
    println!("{:x?} + {:x?} = {:x?}", a, b, c);
    assert_eq!(c, Poly::<Q>::new(&irr, &[1, 1, 0]));
}

#[test]
fn test_poly_sub() {
    let irr = [2, 2, 0, 1];
    let a = Poly::<i128>::new_signed(&irr, &[0, 1, 1]);
    let b = Poly::<i128>::new_signed(&irr, &[1, 0, 2]);
    let c = a.clone() - b.clone();
    println!("{:x?} + {:x?} = {:x?}", a, b, c);
    assert_eq!(c, Poly::<i128>::new_signed(&irr, &[-1, 1, -1]));

    let irr = [2, 2, 0, 1];
    let a = Poly::<Q>::new(&irr, &[0, 1, 1]);
    let b = Poly::<Q>::new(&irr, &[1, 0, 2]);
    let c = a.clone() - b.clone();
    println!("{:x?} + {:x?} = {:x?}", a, b, c);
    assert_eq!(c, Poly::<Q>::new(&irr, &[2, 1, 2]));
}

#[test]
fn test_poly_euclid_div() {
    let irr = [2, 2, 0, 1];
    let a = Poly::<Q>::new(&irr, &[0, 1, 1]);
    let b = Poly::<Q>::new(&irr, &[1, 0, 2]);
    let (c, r) = b.euclid_div(&a);
    println!("{:x?} / {:x?} = {:x?} + r", a, b, c);
    assert_eq!(c.truncate(), Poly::<Q>::new(&irr, &[2]));
    assert_eq!(r.truncate(), Poly::<Q>::new(&irr, &[1, 1]));
}

#[test]
fn test_poly_mul() {
    let irr = [2, 2, 0, 1];
    let a = Poly::<Q>::new(&irr, &[0, 1, 1]);
    let b = Poly::<Q>::new(&irr, &[1, 0, 2]);

    let c = a.clone() * b.clone();
    println!("{:x?} * {:x?} = {:x?}", a, b, c);
    // 2*x^4 + 2*x^3 + x^2 + x  = [1, 0, 1, 2, 2]
    // 2*x + 2                  = [2, 2, 0]
    assert_eq!(c, Poly::<Q>::new(&irr, &[2, 2]));

    let a = Poly::new(&irr, &[2, 2, 0]);
    let b = Poly::new(&irr, &[1, 2, 2]);
    let c = a.clone() * b.clone();
    println!("{:x?} * {:x?} = {:x?}", a, b, c);
    // x^3 + 2*x^2 + 2          = [2, 0, 2, 1]
    // 2*x^2 + x                = [0, 1, 2]
    assert_eq!(c, Poly::<Q>::new(&irr, &[0, 1, 2]));
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
        assert_eq!(test.poly, [Q::from_literal(1)]);
    }

    test_poly_inversion(a, &irr);
    test_poly_inversion(b, &irr);
    test_poly_inversion(c, &irr);
}
