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
        ttv: [[i64; n]; n]
    };

    let res = (1..n)
        .permutations(n - 1)
        .filter(|v| {
            let mut distance = 0;
            let mut from = 0;
            for &to in v {
                distance += ttv[from][to];
                from = to;
            }
            distance += ttv[from][0];
            distance == k
        })
        .count();
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
