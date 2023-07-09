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
        n: usize, k: i64,
        mut abv: [(i64, i64); n]
    };

    let mut sum: i64 = abv.iter().map(|(_, c)| c).sum();
    let mut days = BTreeMap::new();
    days.insert(1, 0);
    for (a, b) in abv {
        *days.entry(a + 1).or_insert(0_i64) += b;
    }
    for (d, c) in days {
        sum -= c;
        if sum <= k {
            println!("{}", d);
            return;
        }
    }
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
