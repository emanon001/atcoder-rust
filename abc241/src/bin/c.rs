#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn check(i: usize, j: usize, grid: &[Vec<char>]) -> bool {
    let n = grid.len();
    let fill_count = 6;
    // 縦
    if i + fill_count <= n {
        let mut white_count = 0;
        for di in 0..fill_count {
            white_count += if grid[i + di][j] == '.' { 1 } else { 0 };
        }
        if white_count <= 2 {
            return true;
        }
    }

    // 横
    if j + fill_count <= n {
        let mut white_count = 0;
        for dj in 0..fill_count {
            white_count += if grid[i][j + dj] == '.' { 1 } else { 0 };
        }
        if white_count <= 2 {
            return true;
        }
    }

    // 斜め
    if i + fill_count <= n && j + fill_count <= n {
        let mut white_count = 0;
        for dij in 0..fill_count {
            white_count += if grid[i + dij][j + dij] == '.' { 1 } else { 0 };
        }
        if white_count <= 2 {
            return true;
        }
    }
    if i + fill_count <= n && j >= fill_count - 1 {
        let mut white_count = 0;
        for dij in 0..fill_count {
            white_count += if grid[i + dij][j - dij] == '.' { 1 } else { 0 };
        }
        if white_count <= 2 {
            return true;
        }
    }
    false
}

fn solve() {
    input! {
        n: usize,
        grid: [Chars; n]
    };

    for i in 0..n {
        for j in 0..n {
            if check(i, j, &grid) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
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
