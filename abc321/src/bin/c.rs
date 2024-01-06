#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        K: usize,
    };

    let mut candidates = vec![];
    for a in 1..=10 {
        for mut v in (0..=9).combinations(a) {
            v.sort_by(|a, b| b.cmp(a));
            candidates.push(v.into_iter().join("").parse::<i64>().unwrap())
        }
    }
    candidates.sort();
    let ans = candidates[K];
    println!("{}", ans);
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
