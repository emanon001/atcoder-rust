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
        N: usize,
        rectangles: [(usize, usize, usize, usize); N],
    };

    let mut grid = vec![vec![0_isize; 1001]; 1001];
    for (lx, ly, rx, ry) in rectangles {
        grid[ly][lx] += 1;
        grid[ly][rx] -= 1;
        grid[ry][lx] -= 1;
        grid[ry][rx] += 1;
    }
    for i in 0..=1000 {
        for j in 1..=1000 {
            grid[i][j] += grid[i][j - 1];
        }
    }
    for j in 0..=1000 {
        for i in 1..=1000 {
            grid[i][j] += grid[i - 1][j];
        }
    }
    let mut counts = vec![0_usize; N + 1];
    for i in 0..=1000 {
        for j in 0..=1000 {
            let c = grid[i][j];
            if c >= 0 {
                counts[c as usize] += 1;
            }
        }
    }
    for k in 1..=N {
        println!("{}", counts[k]);
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
