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
#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn dfs(i: usize, j: usize, h: usize, w: usize, grid: &[Vec<char>], dp: &mut Vec<Vec<Option<i64>>>) -> i64 {
    if let Some(x) = dp[i][j] {
        return x;
    }
    if i == h - 1 && j == w - 1 {
        return 0;
    }
    let t_turn = (i + j) % 2 == 0;
    let x = if t_turn {
        let mut x = -(1_i64 << 60);
        // 右
        if j + 1 < w {
            let add = if grid[i][j + 1] == '+' { 1_i64 } else { -1_i64 };
            chmax!(x, dfs(i, j + 1, h, w, grid, dp) + add);
        }
        // 下
        if i + 1 < h {
            let add = if grid[i + 1][j] == '+' { 1_i64 } else { -1_i64 };
            chmax!(x, dfs(i + 1, j, h, w, grid, dp) + add);
        }
        x
    } else {
        let mut x = 1_i64 << 60;
        // 右
        if j + 1 < w {
            let add = if grid[i][j + 1] == '+' { -1_i64 } else { 1_i64 };
            chmin!(x, dfs(i, j + 1, h, w, grid, dp) + add);
        }
        // 下
        if i + 1 < h {
            let add = if grid[i + 1][j] == '+' { -1_i64 } else { 1_i64 };
            chmin!(x, dfs(i + 1, j, h, w, grid, dp) + add);
        }
        x
    };
    dp[i][j] = Some(x);
    return x;
}

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let mut dp = vec![vec![None; w]; h];
    let p = dfs(0, 0, h, w, &grid, &mut dp);
    let res = if p > 0 {
        "Takahashi"
    } else if p == 0 {
        "Draw"
    } else {
        "Aoki"
    };
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
