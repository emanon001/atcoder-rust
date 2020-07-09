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
    let mut dp = vec![inf; h + 10];
    dp[0] = 0;
    for i in 0..n {
        let (a, b) = abv[i];
        for j in 0..h {
            let to_j = std::cmp::min(j + a, h);
            dp[to_j] = std::cmp::min(dp[to_j], dp[j] + b);
        }
    }
    println!("{}", dp[h]);
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
