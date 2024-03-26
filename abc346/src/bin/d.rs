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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        S: Chars,
        C: [u64; N],
    };

    let inf = 1_u64 << 60;
    let mut dp = vec![vec![vec![inf; 2]; 2]; N];
    if S[0] == '0' {
        dp[0][0][0] = 0;
        dp[0][0][1] = C[0];
    } else {
        dp[0][0][0] = C[0];
        dp[0][0][1] = 0;
    }
    for i in 1..N {
        let ch = S[i];
        // 良い文字列ではない(j == 0)
        if ch == '0' {
            chmin!(dp[i][0][0], dp[i - 1][0][1]);
            chmin!(dp[i][0][1], dp[i - 1][0][0] + C[i]);
        } else {
            // ch == '1'
            chmin!(dp[i][0][1], dp[i - 1][0][0]);
            chmin!(dp[i][0][0], dp[i - 1][0][1] + C[i]);
        }
        // 良い文字列(j == 1)
        if ch == '0' {
            chmin!(dp[i][1][0], dp[i - 1][0][0]);
            chmin!(dp[i][1][1], dp[i - 1][0][1] + C[i]);
            chmin!(dp[i][1][1], dp[i - 1][1][0] + C[i]);
            chmin!(dp[i][1][0], dp[i - 1][1][1]);
        } else {
            // ch == '1'
            chmin!(dp[i][1][0], dp[i - 1][0][0] + C[i]);
            chmin!(dp[i][1][1], dp[i - 1][0][1]);
            chmin!(dp[i][1][1], dp[i - 1][1][0]);
            chmin!(dp[i][1][0], dp[i - 1][1][1] + C[i]);
        }
    }
    let ans = dp[N - 1][1][0].min(dp[N - 1][1][1]);
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
