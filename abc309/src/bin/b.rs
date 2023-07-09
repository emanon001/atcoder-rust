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
        grid: [Chars; n]
    };

    let mut res = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            let cell = grid[i][j];
            if i == 0 {
                if j == n - 1 {
                    res[i + 1][j] = cell;
                } else {
                    res[i][j + 1] = cell;
                }
            } else if i == n - 1 {
                if j == 0 {
                    res[i - 1][j] = cell;
                } else {
                    res[i][j - 1] = cell;
                }
            } else if j == 0 {
                if i == 0 {
                    res[i][j + 1] = cell;
                } else {
                    res[i - 1][j] = cell;
                }
            } else if j == n - 1 {
                if i == n - 1 {
                    res[i][j - 1] = cell;
                } else {
                    res[i + 1][j] = cell;
                }
            } else {
                res[i][j] = cell;
            }
        }
    }
    println!(
        "{}",
        res.into_iter()
            .map(|row| row.into_iter().join(""))
            .join("\n")
    );
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
