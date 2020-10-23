use cargo_snippet::snippet;

pub mod bit;
pub mod bsearch;
pub mod chars;
pub mod divisors;
pub mod dp;
pub mod graph;
pub mod io;
pub mod lca;
pub mod mod_comb;
pub mod mod_int;
pub mod monoid;
pub mod prime;
pub mod segment_tree;
pub mod square;
pub mod strings;
pub mod union_find;

#[snippet("chmin")]
#[macro_export]
macro_rules! chmin {
    ($min:expr, $v:expr) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

#[snippet("chmax")]
#[macro_export]
macro_rules! chmax {
    ($max:expr, $v:expr) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

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
