use cargo_snippet::snippet;

#[snippet]
pub fn compress_zahyo<T: Ord + std::hash::Hash + Clone>(
    zahyo: &[T],
) -> (
    std::collections::HashMap<T, usize>,
    std::collections::HashMap<usize, T>,
) {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x.clone());
    }
    let mut map = std::collections::HashMap::new();
    let mut inverse_map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x.clone(), i);
        inverse_map.insert(i, x);
    }
    (map, inverse_map)
}

#[snippet]
pub fn compress_list<T: Copy + std::cmp::PartialEq>(list: Vec<T>) -> Vec<(T, usize)> {
    let mut res = Vec::new();
    if list.is_empty() {
        return res;
    }
    let mut cur_v = list[0];
    let mut count = 1;
    for v in list.into_iter().skip(1) {
        if v == cur_v {
            count += 1;
        } else {
            res.push((cur_v, count));
            count = 1;
        }
        cur_v = v;
    }
    res.push((cur_v, count));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_zahyo() {
        let zahyo = vec![10, 0, 4, 2, 3, 3, 5];
        let (map, inverse_map) = compress_zahyo(&zahyo);
        assert_eq!(map.len(), 6);
        assert_eq!(map[&0], 0);
        assert_eq!(inverse_map[&0], 0);
        assert_eq!(map[&2], 1);
        assert_eq!(inverse_map[&1], 2);
        assert_eq!(map[&3], 2);
        assert_eq!(inverse_map[&2], 3);
        assert_eq!(map[&4], 3);
        assert_eq!(inverse_map[&3], 4);
        assert_eq!(map[&5], 4);
        assert_eq!(inverse_map[&4], 5);
        assert_eq!(map[&10], 5);
        assert_eq!(inverse_map[&5], 10);
    }

    #[test]
    fn test_compress_list_empty() {
        let list: Vec<usize> = vec![];
        assert_eq!(compress_list(list), vec![]);
    }

    #[test]
    fn test_compress_list_char() {
        let list = "abbcccdbb".chars().collect::<Vec<_>>();
        assert_eq!(
            compress_list(list),
            vec![('a', 1), ('b', 2), ('c', 3), ('d', 1), ('b', 2)]
        );
    }

    #[test]
    fn test_compress_list_number() {
        let list = vec![1, 2, 2, 3, 3, 3, 4, 2, 2];
        assert_eq!(
            compress_list(list),
            vec![(1, 1), (2, 2), (3, 3), (4, 1), (2, 2)]
        );
    }
}
