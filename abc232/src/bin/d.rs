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

fn max_step(
    i: usize,
    j: usize,
    h: usize,
    w: usize,
    grid: &[Vec<char>],
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if let Some(res) = dp[i][j] {
        return res;
    }
    if grid[i][j] == '#' {
        dp[i][j] = Some(0);
        return 0;
    }
    let mut res = 0;
    if i + 1 < h {
        chmax!(res, max_step(i + 1, j, h, w, grid, dp));
    }
    if j + 1 < w {
        chmax!(res, max_step(i, j + 1, h, w, grid, dp));
    }
    res += 1;
    dp[i][j] = Some(res);
    res
}

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h],
    };

    let mut dp = vec![vec![None; w]; h];
    let res = max_step(0, 0, h, w, &grid, &mut dp);
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
