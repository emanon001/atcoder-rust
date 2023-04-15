#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(n: usize, dp: &mut [Option<Vec<String>>]) -> Vec<String> {
    if n == 1 {
        return vec!["1".to_string()];
    }
    if let Some(res) = &dp[n] {
        return res.clone();
    }
    let mut res = dfs(n - 1, dp);
    res.push(n.to_string());
    res.append(&mut dfs(n - 1, dp));
    dp[n] = Some(res.clone());
    res
}

fn solve() {
    input! {
        n: usize
    };
    let mut dp = vec![None; n + 1];
    println!("{}", dfs(n, &mut dp).iter().join(" "));
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
