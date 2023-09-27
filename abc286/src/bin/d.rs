#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(i: usize, money: usize, abv: &Vec<(usize, usize)>, dp: &mut Vec<Vec<Option<bool>>>) -> bool {
    if money == 0 {
        return true;
    }

    if i >= abv.len() {
        return false;
    }

    if let Some(res) = dp[i][money] {
        return res;
    }

    let (a, b) = abv[i];
    let mut res = false;
    for count in 0..=b {
        if a * count > money {
            continue;
        }
        let new_money = money - a * count;
        let is_ok = dfs(i + 1, new_money, abv, dp);
        res = res || is_ok;
    }
    dp[i][money] = res.into();
    res
}

fn solve() {
    input! {
        n: usize, x: usize,
        abv: [(usize, usize); n]
    };

    let mut dp = vec![vec![None; 10.pow(4) + 10]; n + 1];
    let res = if dfs(0, x, &abv, &mut dp) {
        "Yes"
    } else {
        "No"
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
