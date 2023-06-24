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

#[cfg(test)]
mod tests {
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
}
