#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn count_list<T: Ord + std::hash::Hash + Clone>(
    list: Vec<T>,
) -> std::collections::HashMap<T, usize> {
    let mut map = std::collections::HashMap::new();
    for v in list {
        *map.entry(v).or_insert(0) += 1;
    }
    map
}

fn solve() {
    input! {
        n: usize,
        sv: [String; n]
    };

    let map = count_list(sv);
    let res = if map.get("For").unwrap_or(&0) > map.get("Against").unwrap_or(&0) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
