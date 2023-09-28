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
        n: usize, x: usize,
        abv: [(usize, usize); n]
    };

    let mut dp = vec![false; x + 10];
    dp[x] = true;
    for i in 0..n {
        let (a, b) = abv[i];
        for j in 1..=x {
            if !dp[j] {
                continue;
            }
            for k in 1..=b {
                if a * k <= j {
                    dp[j - a * k] = true;
                }
            }
        }
    }

    let res = if dp[0] { "Yes" } else { "No" };
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
