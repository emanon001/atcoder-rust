use cargo_snippet::snippet;

#[snippet]
pub fn compress_zahyo<T: Ord + std::hash::Hash + Clone>(
    zahyo: &[T],
) -> std::collections::HashMap<T, usize> {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x.clone());
    }
    let mut map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x, i);
    }
    map
}

#[snippet]
pub fn count_list<T: Ord + std::hash::Hash + Clone>(
    list: Vec<T>,
) -> std::collections::HashMap<T, usize> {
    let mut map = std::collections::HashMap::new();
    for v in list {
        *map.entry(v).or_insert(0) += 1;
    }
    map
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
    use std::collections::*;
    #[test]
    fn test_compress_zahyo() {
        let zahyo = vec![10, 0, 4, 2, 3, 3, 5];
        let res = compress_zahyo(&zahyo);
        assert_eq!(res.len(), 6);
        assert_eq!(res[&0], 0);
        assert_eq!(res[&2], 1);
        assert_eq!(res[&3], 2);
        assert_eq!(res[&4], 3);
        assert_eq!(res[&5], 4);
        assert_eq!(res[&10], 5);
    }

    #[test]
    fn test_count_list_empty() {
        let list: Vec<usize> = vec![];
        assert_eq!(count_list(list), HashMap::<usize, usize>::new());
    }

    #[test]
    fn test_count_list_string() {
        let list = vec![
            "foo".to_owned(),
            "bar".to_owned(),
            "foo".to_owned(),
            "piyo".to_owned(),
            "bar".to_owned(),
            "foo".to_owned(),
        ];
        let mut expected = HashMap::new();
        expected.insert("foo".to_owned(), 3);
        expected.insert("bar".to_owned(), 2);
        expected.insert("piyo".to_owned(), 1);
        assert_eq!(count_list(list), expected);
    }

    #[test]
    fn test_count_list_number() {
        let list = vec![1, 2, 1, 3, 1, 2];
        let mut expected = HashMap::new();
        expected.insert(1, 3);
        expected.insert(2, 2);
        expected.insert(3, 1);
        assert_eq!(count_list(list), expected);
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
