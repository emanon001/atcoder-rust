use cargo_snippet::snippet;

#[snippet("chmin")]
#[macro_export]
macro_rules! chmin {
    ($min:expr, $v:expr) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

#[snippet("chmax")]
#[macro_export]
macro_rules! chmax {
    ($max:expr, $v:expr) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

#[snippet]
pub fn compress_zahyo<T: Ord + std::hash::Hash>(
    zahyo: &[T],
) -> std::collections::HashMap<&T, usize> {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x);
    }
    let mut map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x, i);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chmin() {
        let mut min = 10;
        assert_eq!(chmin!(min, 11), false);
        assert_eq!(min, 10);
        assert_eq!(chmin!(min, 10), false);
        assert_eq!(min, 10);
        assert_eq!(chmin!(min, 9), true);
        assert_eq!(min, 9);
    }

    #[test]
    fn test_chmax() {
        let mut max = 10;
        assert_eq!(chmax!(max, 9), false);
        assert_eq!(max, 10);
        assert_eq!(chmax!(max, 10), false);
        assert_eq!(max, 10);
        assert_eq!(chmax!(max, 11), true);
        assert_eq!(max, 11);
    }

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
}
