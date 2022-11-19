#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        av: [i64; n - 1],
        bv: [i64; n - 2],
    };

    let mut dp = vec![1_i64 << 60; n];
    let mut prev = vec![n; n];
    dp[0] = 0;
    for i in 0..n {
        if i + 1 < n {
            if chmin!(dp[i + 1], dp[i] + av[i]) {
                prev[i + 1] = i;
            }
        }
        if i + 2 < n {
            if chmin!(dp[i + 2], dp[i] + bv[i]) {
                prev[i + 2] = i;
            }
        }
    }
    let mut pos = n - 1;
    let mut path = vec![pos + 1];
    while pos != 0 {
        pos = prev[pos];
        path.push(pos + 1);
    }
    path.reverse();
    println!("{}", path.len());
    println!("{}", path.iter().join(" "));
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
