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
        abc: [i64; 3]
    };

    let candidates = abc
        .into_iter()
        .combinations(2)
        .map(|ab| ab[0] * ab[1])
        .sorted()
        .collect::<Vec<_>>();
    let min = candidates[0];
    let max = candidates[candidates.len() - 1];
    println!("{} {}", min, max);
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
