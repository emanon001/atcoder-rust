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
        grid: [Chars; h]
    };

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if i + 1 < h {
                if grid[i][j] == '.' && grid[i + 1][j] == '.' {
                    res += 1;
                }
            }
            if j + 1 < w {
                if grid[i][j] == '.' && grid[i][j + 1] == '.' {
                    res += 1;
                }
            }
        }
    }
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
