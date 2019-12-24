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
