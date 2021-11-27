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
        grid: [Chars; 2]
    };

    for i in 0..2 {
        for j in 0..2 {
            if grid[i][j] == '#' {
                if i == 0 && grid[i + 1][j] == '#' {
                    continue;
                }
                if i == 1 && grid[i - 1][j] == '#' {
                    continue;
                }
                if j == 0 && grid[i][j + 1] == '#' {
                    continue;
                }
                if j == 1 && grid[i][j - 1] == '#' {
                    continue;
                }
                println!("No");
                return;
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
