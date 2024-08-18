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
        S: Chars,
    };

    let mut dp = vec![vec![-1_i64; 3]; N + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;
    dp[0][2] = 0;
    for i in 0..N {
        let ch = S[i];
        match ch {
            'R' => {
                // グーを出す
                chmax!(dp[i + 1][0], dp[i][1]);
                chmax!(dp[i + 1][0], dp[i][2]);
                // パーを出す
                chmax!(dp[i + 1][1], dp[i][0] + 1);
                chmax!(dp[i + 1][1], dp[i][2] + 1);
            }
            'P' => {
                // パーを出す
                chmax!(dp[i + 1][1], dp[i][0]);
                chmax!(dp[i + 1][1], dp[i][2]);
                // チョキを出す
                chmax!(dp[i + 1][2], dp[i][0] + 1);
                chmax!(dp[i + 1][2], dp[i][1] + 1);
            }
            'S' => {
                // グーを出す
                chmax!(dp[i + 1][0], dp[i][1] + 1);
                chmax!(dp[i + 1][0], dp[i][2] + 1);
                // チョキを出す
                chmax!(dp[i + 1][2], dp[i][0]);
                chmax!(dp[i + 1][2], dp[i][1]);
            }
            _ => unreachable!(),
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
