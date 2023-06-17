#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs_red(level: u64, count: u64, x: u64, y: u64) -> (u64, u64) {
    if level == 1 {
        return (count, 0);
    }
    let (r1, b1) = dfs_red(level - 1, count, x, y);
    let (r2, b2) = dfs_blue(level, count * x, x, y);
    (r1 + r2, b1 + b2)
}

fn dfs_blue(level: u64, count: u64, x: u64, y: u64) -> (u64, u64) {
    if level == 1 {
        return (0, count);
    }
    let (r1, b1) = dfs_red(level - 1, count, x, y);
    let (r2, b2) = dfs_blue(level - 1, count * y, x, y);
    (r1 + r2, b1 + b2)
}

fn solve() {
    input! {
        n: u64, x: u64, y: u64
    };

    let (_, res) = dfs_red(n, 1, x, y);
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
