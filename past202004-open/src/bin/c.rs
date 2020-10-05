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
        mut grid: [Chars; n]
    };

    for i in (0..n - 1).rev() {
        for j in 1..n * 2 - 2 {
            if grid[i][j] != '#' {
                continue;
            }
            if grid[i + 1][j - 1] == 'X' || grid[i + 1][j] == 'X' || grid[i + 1][j + 1] == 'X' {
                grid[i][j] = 'X';
            }
        }
    }
    for i in 0..n {
        println!("{}", grid[i].iter().join(""));
    }
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
