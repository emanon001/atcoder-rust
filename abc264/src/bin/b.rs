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
        r: Usize1, c: Usize1,
    };

    let grid = vec![
        "111111111111111",
        "100000000000001",
        "101111111111101",
        "101000000000101",
        "101011111110101",
        "101010000010101",
        "101010111010101",
        "101010101010101",
        "101010111010101",
        "101010000010101",
        "101011111110101",
        "101000000000101",
        "101111111111101",
        "100000000000001",
        "111111111111111",
    ];
    let cell = &grid[r][c..=c];
    let res = if cell == "1" { "black" } else { "white" };
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
