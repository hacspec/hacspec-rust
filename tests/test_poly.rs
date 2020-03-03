use hacspec::prelude::*;

macro_rules! gen_poly {
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
    let (a, b, e) = gen_poly!(u128, [0], [0, 1, 1], [1, 0, 2], [1, 1, 0], 3);
    test_add(a, b, e);
    let (a, b, e) = gen_poly!(i128, [0], [0, 1, 1], [1, 0, 2], [1, 1, 0], 3);
    test_add(a, b, e);
    let (a, b, e) = gen_poly!(i128, [0], [-1, 1, 0], [1, 0, -5], [0, 1, 1], 3);
    test_add(a, b, e);

    // Only simple test as irreducible isn't affecting addition.
    let (a, b, e) = gen_poly!(u128, [0, 1, 2, 3], [0, 1, 1], [1, 0, 2], [1, 1, 0], 3);
    test_add(a, b, e);
    let (a, b, e) = gen_poly!(i128, [0, 1, 2, 3], [0, 1, 1], [1, 0, 2], [1, 1, 0], 3);
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
    let (a, b, e) = gen_poly!(i128, [0], [0, 1, 1], [1, 0, 2], [6, 1, 6], 7);
    test_sub(a, b, e);
    let (a, b, e) = gen_poly!(i128, [0], [-1, 1, 0], [1, 0, -5], [253, 1, 5], 255);
    test_sub(a, b, e);
    let (a, b, e) = gen_poly!(u128, [0], [1, 1, 5], [1, 0, 2], [0, 1, 3], 256);
    test_sub(a, b, e);

    // Only simple test as irreducible isn't affecting subtraction.
    let (a, b, e) = gen_poly!(i128, [0, 1, 2, 3], [-1, 1, 0], [1, 0, -5], [253, 1, 5], 255);
    test_sub(a, b, e);
    let (a, b, e) = gen_poly!(u128, [0, 1, 2, 3], [1, 1, 5], [1, 0, 2], [0, 1, 3], 256);
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

    let (a, b, e) = gen_poly!(u128, [2, 2, 0, 1], [1, 0, 2], [0, 1, 1], [2], 3);
    let remainder = Poly::<u128>::new_full(&[2, 2, 0, 1], &[1, 1], 3);
    test_div(a, b, e, remainder);

    let (a, b, e) = gen_poly!(u128, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [6], 11);
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

    let (a, b, e) = gen_poly!(u128, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [7, 4, 8], 11);
    test_mul(a, b, e);
    let (a, b, e) = gen_poly!(i128, [2, 2, 0, 1], [0, 1, 1], [1, 0, 2], [7, 4, 8], 11);
    test_mul(a, b, e);
    let (a, b, e) = gen_poly!(i128, [2, 2, 0, 1], [-3, 5, -1], [1, -2, -7], [8, 8, 7], 11);
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

// Rq = Z[X]/(3329, (X^256+1))
poly_n_m!(Rq_kyber, u128, 256, 3329, &[(0, 1), (256, 1)]);

#[test]
fn test_poly_factory() {
    poly_n!(R3, u128, 4, 3);
    let x = R3::new(&[(0, 1), (1, 2), (3, 1)]);
    println!("x: {:?}", x);

    poly_n_m!(R3q, u128, 4, 3, &[(0, 1), (1, 2), (4, 1)]);
    let x = R3q::new(&[(0, 1), (1, 2), (3, 1)]);
    println!("x: {:?}", x);

    let a = Rq_kyber::random();
    let b = Rq_kyber::new(&[(5, 234), (122, 3000)]);
    let c = a.clone() * b.clone();
    println!("c: {:?}", c);
}

poly_n!(DummyPolyN, u128, 4, 3);
poly_n_m!(DummyPolyNM, u128, 4, 3, &[(0, 1), (1, 2), (4, 1)]);

// Rq = Z[X]/(3329, (X^256+1))
poly!(RqKyberFixedLength, u128, 256, 3329, &[(0, 1), (256, 1)]);

#[test]
fn test_fixed_length() {
    let a = RqKyberFixedLength::new(&[(0, 1), (5, 55), (77, 123)]);
    let b = RqKyberFixedLength::random();
    let c = a * b;
    println!("{:x?} * {:x?} = {:x?}", a, b, c);

    let b = RqKyberFixedLength::new_full([0x72a, 0x50b, 0x6db, 0x26e, 0x536, 0x253, 0x292, 0x42f, 0x2da, 0x92b, 0x9b4, 0xbfc, 0x263, 0x636, 0x78b, 0x82e, 0x54a, 0x8cf, 0xc3, 0xa30, 0x99e, 0x2f4, 0x696, 0x2be, 0x2a6, 0x159, 0x147, 0x4b, 0xa44, 0x255, 0x9c5, 0x1a7, 0xa61, 0x640, 0xca3, 0xb51, 0x761, 0xbf2, 0x210, 0x25e, 0xa90, 0x25b, 0x1ab, 0x5e5, 0x7a2, 0x235, 0x9d0, 0x373, 0x55, 0xc46, 0x1c3, 0x90a, 0x21b, 0xa0d, 0x73e, 0x6ce, 0x4b4, 0x355, 0x681, 0x667, 0x8a0, 0x3e, 0xb79, 0x190, 0xbab, 0x137, 0xb43, 0x493, 0x399, 0x8e8, 0x731, 0x24b, 0x43f, 0x9ef, 0x206, 0x5d4, 0x252, 0x9da, 0x449, 0xa34, 0xc13, 0x5c2, 0x6f, 0x1d1, 0x397, 0x6f7, 0xc9c, 0x736, 0x95a, 0x6ef, 0x724, 0x25b, 0xcec, 0x784, 0xab5, 0xbc2, 0x12f, 0x5ff, 0x834, 0x34e, 0x282, 0x47d, 0x874, 0x46e, 0xced, 0x682, 0x329, 0x5ab, 0x7ca, 0x3df, 0xcd6, 0x412, 0x444, 0xa7e, 0xc61, 0x9b1, 0xa59, 0x612, 0x2bc, 0x391, 0xd, 0xa48, 0x46c, 0xa9a, 0xc7b, 0x4a4, 0x873, 0xc48, 0x114, 0x8a6, 0x666, 0xad9, 0x5ce, 0x13f, 0x88d, 0x4c3, 0xae6, 0x9fe, 0x548, 0x8f8, 0x422, 0x653, 0x67a, 0x39a, 0x57e, 0xa95, 0x33, 0x76d, 0x101, 0xc89, 0xbd, 0x8b0, 0x146, 0x916, 0x5d, 0x577, 0x278, 0x16a, 0x6e, 0x558, 0xc59, 0xce4, 0x7f0, 0xbe5, 0x6c7, 0x84b, 0xac4, 0x8c1, 0x5b5, 0xd7, 0x993, 0x207, 0xb74, 0xf1, 0x926, 0x75c, 0x8c3, 0x1c4, 0x86d, 0x9ee, 0x380, 0x32a, 0x8dd, 0x56, 0x747, 0x20c, 0x737, 0x596, 0x292, 0x811, 0x4a8, 0x4f2, 0xb45, 0x158, 0x226, 0xc72, 0x99a, 0x1cd, 0x520, 0x6b1, 0x250, 0xbbb, 0x140, 0x476, 0x5e8, 0x45, 0x3a, 0x708, 0x3f1, 0x32a, 0xa6a, 0x694, 0x2f4, 0x39e, 0x5ad, 0xb2b, 0x7e1, 0xb5c, 0xe1, 0xc64, 0x1f3, 0x90b, 0x67a, 0x66c, 0x478, 0x647, 0x227, 0x26, 0x912, 0x581, 0x666, 0x884, 0x879, 0x30c, 0x142, 0x8c5, 0x72d, 0x3da, 0x48a, 0x15b, 0xca0, 0x284, 0x4e7, 0x6cc, 0x7ad, 0x29b, 0x3be, 0x63f, 0x655, 0x22a, 0x12f, 0x32b, 0x898, 0xaa1, 0xc3, 0x9fc]);
    let c = a * b;
    let expected_c = RqKyberFixedLength::new_full([766, 3113, 2145, 433, 3310, 2147, 553, 246, 2197, 1441, 928, 2499, 339, 3140, 2150, 1155, 1259, 3175, 981, 701, 145, 2410, 2688, 2028, 323, 3043, 130, 2446, 2933, 334, 1742, 87, 2719, 3217, 2068, 1681, 3068, 972, 493, 1051, 1584, 3173, 387, 3052, 2851, 1915, 1137, 3201, 1839, 1443, 1366, 2251, 678, 123, 2157, 257, 2144, 958, 1928, 631, 2073, 1646, 601, 861, 2285, 3229, 819, 1595, 2681, 472, 739, 2039, 2358, 2312, 962, 2195, 1456, 1595, 145, 621, 1865, 1815, 2279, 1916, 5, 1916, 131, 557, 2524, 971, 2511, 2860, 2868, 526, 1554, 1033, 1378, 267, 1168, 2776, 2863, 1338, 2522, 392, 2471, 688, 946, 1982, 3207, 460, 2093, 685, 1129, 1816, 1038, 1809, 1337, 1384, 2885, 1357, 2491, 2505, 2963, 645, 2227, 2371, 749, 1033, 1398, 1219, 622, 239, 2508, 470, 2506, 3152, 919, 2998, 3220, 1237, 788, 3146, 835, 450, 2070, 1074, 17, 3279, 3324, 1184, 2571, 2163, 2103, 2969, 760, 249, 1471, 2334, 3074, 162, 812, 1292, 1563, 1, 1094, 1051, 3101, 2616, 88, 3174, 2994, 3208, 396, 1350, 3232, 2167, 1686, 990, 276, 1373, 981, 1871, 2346, 1843, 562, 2114, 1840, 1092, 394, 2487, 858, 3275, 534, 2829, 2328, 1712, 1292, 943, 1081, 1420, 307, 1867, 2020, 3122, 3063, 3326, 1446, 1160, 2581, 428, 2421, 1143, 18, 111, 1105, 2859, 868, 1514, 617, 727, 1501, 3223, 2113, 318, 3289, 741, 3259, 1289, 104, 1228, 3273, 647, 3228, 3157, 2499, 2666, 378, 3076, 547, 1673, 1892, 2158, 359, 1829, 412, 1927, 2901, 2578, 1366, 3205, 51, 2009, 2293, 1575, 3036, 1567]);
    println!("{:x?} * {:x?} = {:x?}", a, b, c);
    assert_eq!(c, expected_c);
}
