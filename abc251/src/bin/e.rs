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
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize,
        av: [i64; n]
    };

    let mut dp1 = vec![vec![1_i64 << 60; 2]; n];
    dp1[0][0] = 0_i64;
    for i in 0..n - 1 {
        // i に餌をあげていない場合
        chmin!(dp1[i + 1][1], dp1[i][0] + av[i]);
        // i に餌をあげている場合
        chmin!(dp1[i + 1][0], dp1[i][1]);
        chmin!(dp1[i + 1][1], dp1[i][1] + av[i]);
    }
    // eprintln!("{:?}", dp1);
    let mut dp2 = vec![vec![1_i64 << 60; 2]; n];
    dp2[0][1] = av[n - 1];
    // dp[0][1] = 0_i64;
    for i in 0..n - 1 {
        // i に餌をあげていない場合
        chmin!(dp2[i + 1][1], dp2[i][0] + av[i]);
        // i に餌をあげている場合
        chmin!(dp2[i + 1][0], dp2[i][1]);
        chmin!(dp2[i + 1][1], dp2[i][1] + av[i]);
    }
    // eprintln!("{:?}", dp2);
    let res = dp1[n - 1][1].min(dp2[n - 1][0]).min(dp2[n - 1][1]);
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
