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
    let x = Bytes::from_array(&[0, 1, 2, 3, 4, 5]);

    fn sth(b: ByteSlice) {
        println!("do: {:?}", b);
    }

    let x_slice = x.get_slice();
    // Don't use x behind this point.
    sth(x_slice);
    println!("{:?}", x_slice);
}
