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
        h: usize, w: usize,
        grid: [[isize; w]; h]
    };

    let mut min = 1 << 30;
    for i in 0..h {
        for j in 0..w {
            let c = grid[i][j];
            if c < min {
                min = c;
            }
        }
    }
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            let c = grid[i][j];
            res += c - min;
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
