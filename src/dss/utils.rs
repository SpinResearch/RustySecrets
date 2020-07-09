use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// Transmutes a `&[u8]` into a `&[u32]`.
/// Despite `std::mem::transmute` being very unsafe in
/// general, this should actually be safe as long as
/// `input` contains a multiple of 4 bytes.
#[allow(unsafe_code, clippy::transmute_ptr_to_ptr)]
pub(crate) fn slice_u8_to_slice_u32(input: &[u8]) -> &[u32] {
    assert_eq!(input.len() % 4, 0);
    unsafe { std::mem::transmute(input) }
}

/// Creates a `HashMap` from a `BTreeMap`
pub(crate) fn btreemap_to_hashmap<A: Eq + Hash, B>(btree: BTreeMap<A, B>) -> HashMap<A, B> {
    let mut hash = HashMap::new();
    hash.extend(btree.into_iter());
    hash
}

/// Creates a `BTreeMap` from a `HashMap`
pub(crate) fn hashmap_to_btreemap<A: Ord + Hash, B>(hash: HashMap<A, B>) -> BTreeMap<A, B> {
    let mut btree = BTreeMap::new();
    btree.extend(hash.into_iter());
    btree
}
