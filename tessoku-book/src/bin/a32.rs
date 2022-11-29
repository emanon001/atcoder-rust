#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn rec(turn: usize, c: usize, a: usize, b: usize, dp: &mut [Vec<Option<bool>>]) -> bool {
    if let Some(is_win) = dp[turn][c] {
        return is_win;
    }

    let mut is_win = false;
    if c >= a {
        is_win = is_win || !rec((turn + 1) % 2, c - a, a, b, dp);
    }
    if c >= b {
        is_win = is_win || !rec((turn + 1) % 2, c - b, a, b, dp);
    }
    dp[turn][c] = is_win.into();
    is_win
}

fn solve() {
    input! {
        n: usize, a: usize, b: usize,
    };

    let mut dp = vec![vec![None; n + 1]; 2];
    let is_win = rec(0, n, a, b, &mut dp);
    let res = if is_win { "First" } else { "Second" };
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
