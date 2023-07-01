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
        n: usize, m: usize
    };

    let res = (1..=m)
        .combinations(n)
        .map(|v| v.into_iter().sorted().collect::<Vec<_>>())
        .sorted()
        .collect::<Vec<_>>();
    println!(
        "{}",
        res.into_iter().map(|v| v.into_iter().join(" ")).join("\n ")
    );
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
