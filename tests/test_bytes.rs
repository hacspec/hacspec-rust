use hacspec::*;

hacspec_crates!();
hacspec_imports!();

#[test]
fn test_bytes() {
    bytes!(TestBytes, 77);
}
