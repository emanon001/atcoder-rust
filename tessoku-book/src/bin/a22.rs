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
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        av: [Usize1; n - 1],
        bv: [Usize1; n - 1],
    };

    let mut dp = vec![-(1 << 30); n];
    dp[0] = 0;
    for i in 0..n - 1 {
        chmax!(dp[av[i]], dp[i] + 100);
        chmax!(dp[bv[i]], dp[i] + 150);
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
