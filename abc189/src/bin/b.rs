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
        n: usize, x: i64,
        vvp: [(i64, i64); n]
    };

    let mut total = 0_i64;
    for i in 0..n {
        let (v, p) = vvp[i];
        total += v * p;
        if total > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
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
