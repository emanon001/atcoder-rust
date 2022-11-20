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
        n: usize, s: usize,
        av: [usize; n]
    };

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    let mut from = vec![None; s + 1];
    for i in 0..n {
        let a = av[i];
        for j in 0..=s {
            dp[i + 1][j] |= dp[i][j];
        }
        for j in 0..=s {
            if j + a <= s && dp[i][j] && !dp[i + 1][j + a] {
                from[j + a] = i.into();
                dp[i + 1][j + a] = true;
            }
        }
    }

    if !dp[n][s] {
        println!("-1");
        return;
    }

    let mut res = VecDeque::new();
    let mut sum = s;
    while sum > 0 {
        let i = from[sum].unwrap();
        res.push_front(i + 1); // 1-origin
        sum -= av[i];
    }
    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
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
