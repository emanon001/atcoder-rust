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
        grid: [[i64; w]; h]
    };

    for i in 0..h {
        for j in 0..w {
            for i2 in i + 1..h {
                for j2 in j + 1..w {
                    if grid[i][j] + grid[i2][j2] > grid[i2][j] + grid[i][j2] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
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
