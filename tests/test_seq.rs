use hacspec::*;

hacspec_crates!();
hacspec_imports!();

#[test]
fn test_seq_u32() {
    seq!(TestSeq, u32, 64);
}
