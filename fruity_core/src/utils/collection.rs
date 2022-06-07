use std::collections::HashMap;
use std::hash::Hash;

/// Insert an element in an hashmap that contains a vec
///
/// # Arguments
/// * `hashmap` - The hashmap
/// * `key` - The key of the value that is added
/// * `value` - The value that will be inserted
///
pub fn insert_in_hashmap_vec<K: Eq + Hash, T>(hashmap: &mut HashMap<K, Vec<T>>, key: K, value: T) {
    if let Some(vec) = hashmap.get_mut(&key) {
        vec.push(value);
    } else {
        hashmap.insert(key, vec![value]);
    }
}

/// A temporary stable implementation for drain_filter
pub fn drain_filter<T, Pred: Fn(&T) -> bool>(source: &mut Vec<T>, pred: Pred) -> Vec<T> {
    let mut drained: Vec<T> = Vec::new();

    while let Some((index, _)) = source.iter().enumerate().find(|elem| pred(elem.1)) {
        drained.push(source.remove(index));
    }

    drained
}
