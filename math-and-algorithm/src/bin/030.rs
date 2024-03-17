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
        N: usize, W: usize,
        WV: [(usize, usize); N],
    };

    let mut dp = vec![vec![0; W + 1]; N + 1];
    for i in 0..N {
        let (w, v) = WV[i];
        for j in 0..=W {
            chmax!(dp[i + 1][j], dp[i][j]);
            if j + w <= W {
                chmax!(dp[i + 1][j + w], dp[i][j] + v);
            }
        }
    }
    let ans = dp[N].iter().max().unwrap();
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
