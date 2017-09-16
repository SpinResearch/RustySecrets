
use std;

/// Transmutes a `&[u8]` into a `&[u32]`.
/// Despite `std::mem::transmute` being very unsafe in
/// general, this should actually be safe as long as
/// `input` contains a multiple of 4 bytes.
#[allow(unsafe_code)]
pub(crate) fn slice_u8_to_slice_u32(input: &[u8]) -> &[u32] {
    assert_eq!(input.len() % 4, 0);
    unsafe { std::mem::transmute(input) }
}
