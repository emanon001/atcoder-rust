#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        H: usize, W: usize,
        grid: [[usize; W]; H],
    };

    let mut dp = vec![vec![vec![None; H + W]; W + 1]; H + 1];
    let mut ans = 0;
    for i in 1..H + W {
        chmax!(ans, rec(H - 1, W - 1, i, &mut dp, &grid));
        println!("{}", ans);
    }
}

fn rec(
    i: usize,
    j: usize,
    k: usize,
    dp: &mut Vec<Vec<Vec<Option<usize>>>>,
    grid: &Vec<Vec<usize>>,
) -> usize {
    if let Some(res) = dp[i][j][k] {
        return res;
    }
    if i == 0 && j == 0 {
        return if k == 0 { 0 } else { grid[i][j] };
    }
    let mut res = 0;
    if i > 0 {
        if k < i + j + 1 {
            chmax!(res, rec(i - 1, j, k, dp, grid));
        }
        if k > 0 {
            chmax!(res, rec(i - 1, j, k - 1, dp, grid) + grid[i][j]);
        }
    }
    if j > 0 {
        if k < i + j + 1 {
            chmax!(res, rec(i, j - 1, k, dp, grid));
        }
        if k > 0 {
            chmax!(res, rec(i, j - 1, k - 1, dp, grid) + grid[i][j]);
        }
    }
    dp[i][j][k] = Some(res);
    res
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
