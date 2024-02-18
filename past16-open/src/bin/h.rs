#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, M: usize,
        A: [u64; N]
    };

    let mut dp = vec![vec![vec![0; 2]; M + 1]; N + 1];
    for i in 0..N {
        for j in 0..=(i + 1) / 2 {
            if j > M {
                continue;
            }
            for k in 0..2 {
                chmax!(dp[i + 1][j][k], dp[i][j][k]);
                if k == 0 {
                    // 前日に宿題を行っていない
                    if j < M {
                        chmax!(dp[i + 1][j + 1][1], dp[i][j][k]);
                    }
                    chmax!(dp[i + 1][j][0], dp[i][j][k] + A[i]);
                } else {
                    // 前日に宿題を行った
                    chmax!(dp[i + 1][j][0], dp[i][j][k] + A[i]);
                }
            }
        }
    }
    let ans = dp[N][M][0].max(dp[N][M][1]);
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
