use cargo_snippet::snippet;

#[snippet]
pub fn invert_hash_map<K, V: std::hash::Hash + std::cmp::Eq>(
    map: std::collections::HashMap<K, V>,
) -> std::collections::HashMap<V, K> {
    let mut res = std::collections::HashMap::new();
    for (k, v) in map {
        res.insert(v, k);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_invert_hash_map() {
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        let inverted = invert_hash_map(map);

        assert_eq!(inverted.get(&1), Some(&"one"));
        assert_eq!(inverted.get(&2), Some(&"two"));
    }
}
