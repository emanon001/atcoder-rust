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
        hv: [i64; n]
    };

    let mut dp = vec![1_i64 << 60; n];
    dp[0] = 0;
    for i in 0..n {
        if i + 1 < n {
            chmin!(dp[i + 1], dp[i] + (hv[i] - hv[i + 1]).abs());
        }
        if i + 2 < n {
            chmin!(dp[i + 2], dp[i] + (hv[i] - hv[i + 2]).abs());
        }
    }
    println!("{}", dp[n - 1]);
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
