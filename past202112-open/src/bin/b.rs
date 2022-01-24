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
        n: usize,
        abv: [(i64, i64); n]
    };

    let mut counts = vec![0; 6];
    for (a, b) in abv {
        let mut rest = b - a;
        let coins = vec![500, 100, 50, 10, 5, 1];
        for (i, coin) in coins.iter().enumerate() {
            counts[i] += rest / coin;
            rest = rest % coin;
        }
    }
    let res = counts[2] + counts[4];
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
