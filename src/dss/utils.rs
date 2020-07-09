use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

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
