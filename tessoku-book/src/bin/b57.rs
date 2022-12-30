#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn sum(n: usize) -> usize {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum()
}

fn solve() {
    input! {
        n: usize, k: usize,
    };
    let mut dp = vec![vec![0; n + 1]; 30];
    for j in 0..=n {
        let k = j - sum(j);
        dp[0][j] = k;
    }
    for i in 1..30 {
        for j in 0..=n {
            let k = dp[i - 1][dp[i - 1][j]];
            dp[i][j] = k;
        }
    }

    for n in 1..=n {
        let mut n = n;
        let mut rest = k;
        for d in (0..30).rev() {
            if rest & 2.pow(d) != 0 {
                rest -= 2.pow(d);
                n = dp[d as usize][n];
            }
        }
        println!("{}", n);
    }
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
