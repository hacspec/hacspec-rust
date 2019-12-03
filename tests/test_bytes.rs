use hacspec::*;

hacspec_crates!();
hacspec_imports!();

#[test]
fn test_bytes() {
    bytes!(TestBytes, 77);
    #[field(ffff)]
    struct TestField;

    let x = TestField::from(5);
    println!("{:?}", x);
    let x = TestBytes::from_field(x);
    println!("{:?}", x);
}

#[test]
fn test_vlbytes() {
    vlbytes!(x, x_, "000102030405");
    // let x_ = Bytes::from_array(&[0, 1, 2, 3, 4, 5]);
    // let x = x_.get_slice();

    fn sth(b: ByteSlice) {
        println!("do: {:?}", b);
    }

    sth(x);
    println!("{:?}", x);
}

#[test]
fn test_bytes_conversion() {
    bytes!(TestBytes, 77);
    bytes!(OtherBytes, 77);

    fn do_test(b: TestBytes) {
        println!("TestBytes: {:?}", b);
    }
    fn do_other(b: OtherBytes) {
        println!("OtherBytes: {:?}", b);
    }

    let t = TestBytes::new();
    let o = OtherBytes::new();

    do_test(TestBytes::from_byte_array(&o));
    do_other(OtherBytes::from_byte_array(&t));
}
