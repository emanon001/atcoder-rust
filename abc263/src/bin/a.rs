#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        a: usize, b: usize, c: usize, d: usize, e: usize,
    };

    let mut map = HashMap::new();
    *map.entry(&a).or_insert(0) += 1;
    *map.entry(&b).or_insert(0) += 1;
    *map.entry(&c).or_insert(0) += 1;
    *map.entry(&d).or_insert(0) += 1;
    *map.entry(&e).or_insert(0) += 1;
    let res = if map.len() == 2 && map.values().any(|x| x == &2) {
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
