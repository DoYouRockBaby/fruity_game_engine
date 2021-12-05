use std::collections::HashMap;
use std::hash::Hash;

pub fn insert_in_hashmap_vec<K: Eq + Hash, T>(hashmap: &mut HashMap<K, Vec<T>>, key: K, value: T) {
    if let Some(vec) = hashmap.get_mut(&key) {
        vec.push(value);
    } else {
        hashmap.insert(key, vec![value]);
    }
}
