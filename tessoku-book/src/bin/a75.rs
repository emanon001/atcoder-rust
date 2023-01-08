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
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize,
        mut tdv: [(usize, usize); n]
    };

    tdv.sort_by_key(|(_, d)| *d);
    let max_time = 1440;
    let mut dp = vec![vec![0; max_time + 1]; n + 1];
    for (i, (t, v)) in tdv.into_iter().enumerate() {
        for j in 0..=max_time {
            chmax!(dp[i + 1][j], dp[i][j]);
            if j + t <= v {
                chmax!(dp[i + 1][j + t], dp[i][j] + 1);
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
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
