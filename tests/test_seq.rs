use hacspec::prelude::*;

#[test]
fn test_byte_sequences() {
    let msg = ByteSeq::from("0388dace60b6a392f328c2b971b2fe78");
    let msg_u32 = Seq::<u32>::from_array(&[0x0388dace, 0x60b6a392, 0xf328c2b9, 0x71b2fe78]);
    for (i, (l, chunk)) in msg.chunks(4).enumerate() {
        assert_eq!(l, 4);
        assert_eq!((msg_u32[i] & 0xFF) as u8, U8::declassify(chunk[3]));
        assert_eq!(((msg_u32[i] & 0xFF00) >> 8) as u8, U8::declassify(chunk[2]));
        assert_eq!(((msg_u32[i] & 0xFF0000) >> 16) as u8, U8::declassify(chunk[1]));
        assert_eq!(((msg_u32[i] & 0xFF000000) >> 24) as u8, U8::declassify(chunk[0]));
    }
}
