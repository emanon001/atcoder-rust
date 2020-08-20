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
        n: i64, m: i64, d: i64,
    };

    let mut res = 0_f64;
    for _ in 0..m - 1 {
        res += if d == 0 {
            1.0 / n as f64
        } else {
            ((n - d).max(0) * 2) as f64 / (n * n) as f64
        };
    }
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
