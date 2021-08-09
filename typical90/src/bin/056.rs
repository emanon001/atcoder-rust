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
        abv: [(usize, usize); n]
    };

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        let (a, b) = abv[i];
        for j in 0..s {
            if j + a <= s {
                dp[i + 1][j + a] = dp[i + 1][j + a] || dp[i][j];
            }
            if j + b <= s {
                dp[i + 1][j + b] = dp[i + 1][j + b] || dp[i][j];
            }
        }
    }
    if !dp[n][s] {
        println!("Impossible");
        return;
    }

    let mut res = VecDeque::new();
    let mut cur = s;
    for i in (0..n).rev() {
        let (a, b) = abv[i];
        if a <= cur && dp[i][cur - a] {
            res.push_front('A');
            cur = cur - a;
        } else {
            res.push_front('B');
            cur = cur - b;
        }
    }
    println!("{}", res.iter().join(""));
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
