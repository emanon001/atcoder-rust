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
        h: usize, n: usize,
        abv: [(usize, usize); n]
    };

    let inf = 1_usize << 30;
    let max_damage = 10.pow(4);
    let mut dp = vec![inf; h + max_damage + 10];
    dp[0] = 0;
    for i in 0..n {
        let (a, b) = abv[i];
        for j in a..h + max_damage {
            dp[j] = std::cmp::min(dp[j], dp[j - a] + b);
        }
    }
    let mut res = std::usize::MAX;
    for i in h..h + max_damage {
        res = res.min(dp[i]);
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
