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
        grid: [[usize; 3]; 3],
        n: usize,
        bv: [usize; n]
    };

    let mut marks = vec![vec![false; 3]; 3];
    for b in bv {
        for i in 0..3 {
            for j in 0..3 {
                if grid[i][j] == b {
                    marks[i][j] = true;
                }
            }
        }
    }
    let mut count = 0;
    // 横
    for i in 0..3 {
        if (0..3).all(|j| marks[i][j]) {
            count += 1;
        }
    }
    // 縦
    for j in 0..3 {
        if (0..3).all(|i| marks[i][j]) {
            count += 1;
        }
    }
    // 斜め
    if (0..3).all(|ij| marks[ij][ij]) {
        count += 1;
    }
    if marks[0][2] && marks[1][1] && marks[2][0] {
        count += 1;
    }
    let res = if count > 0 { "Yes" } else { "No" };
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
