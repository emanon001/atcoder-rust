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
        N: usize,
        A: [i64; N],
    };

    let mut dp = vec![vec![0; 2]; N + 1];
    for i in 0..N {
        // 勉強しない場合(0)
        chmax!(dp[i + 1][0], dp[i][0]);
        chmax!(dp[i + 1][0], dp[i][1]);
        // 勉強する場合(1)
        chmax!(dp[i + 1][1], dp[i][0] + A[i]);
    }
    let ans = dp[N][0].max(dp[N][1]);
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
