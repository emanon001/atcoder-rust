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
        mut a_grid: [[i64; w]; h],
        b_grid: [[i64; w]; h],
    };

    let mut res = 0;
    for i in 0..=h - 2 {
        for j in 0..=w - 2 {
            let diff = b_grid[i][j] - a_grid[i][j];
            a_grid[i][j] += diff;
            a_grid[i][j + 1] += diff;
            a_grid[i + 1][j] += diff;
            a_grid[i + 1][j + 1] += diff;
            res += diff.abs();
        }
    }
    for i in 0..h {
        for j in 0..w {
            if a_grid[i][j] != b_grid[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
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
