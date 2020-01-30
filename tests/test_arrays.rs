use hacspec::prelude::*;

#[test]
fn test_public_array_u32() {
    public_array!(TestSeq, 64, u32);
}
