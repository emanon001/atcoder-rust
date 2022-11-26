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

    let mut dp = vec![vec![0_u64; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                continue;
            }
            if i + 1 < h && grid[i + 1][j] == '.' {
                dp[i + 1][j] += dp[i][j];
            }
            if j + 1 < w && grid[i][j + 1] == '.' {
                dp[i][j + 1] += dp[i][j];
            }
        }
    }
    let res = dp[h - 1][w - 1];
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
