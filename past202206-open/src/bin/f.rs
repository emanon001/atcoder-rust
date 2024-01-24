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
        H: usize, W: usize,
        mut grid: [[usize; W]; H],
        N: usize,
        RC: [(Usize1, Usize1); N]
    };

    for (r, c) in RC {
        let b = grid[r][c];
        if b == 0 {
            continue;
        }

        grid[r][c] = 0;
        for r2 in (1..=r).rev() {
            let t = grid[r2][c];
            grid[r2][c] = grid[r2 - 1][c];
            grid[r2 - 1][c] = t;
        }
    }
    println!(
        "{}",
        grid.into_iter()
            .map(|row| row.into_iter().join(" "))
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
