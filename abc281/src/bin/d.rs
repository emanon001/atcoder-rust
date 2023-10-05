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
        n: usize, k: usize, d: usize,
        av: [usize; n]
    };

    let mut dp = vec![vec![vec![None; d]; k + 1]; n + 1];
    dp[0][0][0] = Some(0);
    for i in 0..n {
        let a = av[i];
        for k_i in 0..=k {
            for d_i in 0..d {
                chmax!(dp[i + 1][k_i][d_i], dp[i][k_i][d_i]);
                if let Some(count) = dp[i][k_i][d_i] {
                    let v = d * count + d_i + a;
                    let new_k = k_i + 1;
                    if new_k <= k {
                        let new_count = v / d;
                        let new_d = v % d;
                        chmax!(dp[i + 1][new_k][new_d], Some(new_count));
                    }
                }
            }
        }
    }
    let ans = dp[n][k][0].map(|c| (c * d) as isize).unwrap_or(-1);
    println!("{}", ans);
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
