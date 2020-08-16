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
        r: usize, c: usize, d: usize,
        grid: [[usize; c]; r]
    };

    let mut res = 0;
    for i in 0..r {
        for j in 0..c {
            let cost = i + j;
            if cost > d {
                continue;
            }
            if d % 2 != cost % 2 {
                continue;
            }
            let price = grid[i][j];
            if price > res {
                res = price;
            }
        }
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
