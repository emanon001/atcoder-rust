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
        n: usize, _: usize, _: usize,
        abv: [(usize, usize); n]
    };

    let nim = abv
        .into_iter()
        .fold(0, |acc, (a, b)| acc ^ (a - 1) ^ (b - 1));
    let res = if nim == 0 { "Second" } else { "First" };
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
