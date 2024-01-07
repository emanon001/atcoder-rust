#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize
    };
    let mut grid = vec![vec!["".to_string(); N]; N];
    grid[N / 2][N / 2] = "T".to_string();
    let mut id = 1;
    for p in 0..(N / 2) {
        go_around(&mut id, p, N, &mut grid);
    }
    println!("{}", grid.iter().map(|row| row.iter().join(" ")).join("\n"));
}

fn go_around(id: &mut usize, p: usize, n: usize, grid: &mut [Vec<String>]) {
    // →
    for j in p..(n - p - 1) {
        grid[p][j] = id.to_string();
        *id += 1;
    }
    // ↓
    for i in p..(n - p) {
        grid[i][n - p - 1] = id.to_string();
        *id += 1;
    }
    // ←
    for j in (p..(n - p - 1)).rev() {
        grid[n - p - 1][j] = id.to_string();
        *id += 1;
    }
    // ↑
    for i in ((p + 1)..(n - p - 1)).rev() {
        grid[i][p] = id.to_string();
        *id += 1;
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
