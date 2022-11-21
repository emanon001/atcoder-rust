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
        n: usize, w: usize,
        wvv: [(usize, usize); n]
    };

    let mut dp = vec![vec![-1; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (wi, vi) = wvv[i];
        for j in 0..=w {
            chmax!(dp[i + 1][j], dp[i][j]);
            if dp[i][j] >= 0 && j + wi <= w {
                chmax!(dp[i + 1][j + wi], dp[i][j] + vi as isize);
            }
        }
    }
    let res = dp[n].iter().max().unwrap();
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
