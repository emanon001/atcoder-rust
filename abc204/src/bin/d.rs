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
        mut tv: [usize; n]
    };

    tv.sort();
    let mut dp = vec![vec![1_usize << 30; 100000 + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let t = tv[i];
        for j in 0..=100000 {
            if j + t > 100000 {
                continue;
            }
            // オーブン1を使用
            chmin!(dp[i + 1][j + t], dp[i][j]);
            // オーブン2を使用
            chmin!(dp[i + 1][j], dp[i][j] + t);
        }
    }
    let mut res = 1_usize << 30;
    for j in 0..=100000 {
        chmin!(res, j.max(dp[n][j]));
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
