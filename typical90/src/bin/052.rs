#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

const M: u64 = 1000000007;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [[u64; 6]; N],
    };

    let mut dp = vec![None; N];
    let ans = dfs(0, &A, &mut dp);
    println!("{}", ans);
}

fn dfs(i: usize, a: &[Vec<u64>], dp: &mut Vec<Option<u64>>) -> u64 {
    if let Some(res) = dp[i] {
        return res;
    }
    if i == a.len() - 1 {
        let sum = a[i].iter().sum::<u64>() % M;
        dp[i] = Some(sum);
        return sum;
    }
    let mut res = 0_u64;
    for j in 0..6 {
        res = (res + (a[i][j] * dfs(i + 1, a, dp) % M)) % M;
    }
    dp[i] = Some(res);
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
