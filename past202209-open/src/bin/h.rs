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
        N: usize, X: usize,
        ABC: [(usize, usize, usize); N]
    };

    // dp[i][j] = i番目までの袋を確認して、銅貨をj枚使った時の、(最大の金貨の獲得数、最小の銀貨の使用数)
    let mut dp = vec![vec![(0, std::cmp::Reverse(10_usize << 30)); X + 1]; N + 1];
    dp[0][0] = (0, std::cmp::Reverse(0));
    for (i, (a, b, c)) in ABC.into_iter().enumerate() {
        for j in 0..=X {
            // 袋を購入しない場合
            chmax!(dp[i + 1][j], dp[i][j]);
            // 袋を購入する場合
            if j + b <= X {
                chmax!(
                    dp[i + 1][j + b],
                    (dp[i][j].0 + c, std::cmp::Reverse(dp[i][j].1 .0 + a))
                );
            }
        }
    }
    let mut gold_and_used_silver = (0_usize, std::cmp::Reverse(10_usize << 30));
    let mut used_bronze = 0;
    for i in 0..=X {
        if chmax!(gold_and_used_silver, dp[N][i]) {
            used_bronze = i;
        }
    }
    println!(
        "{} {} {}",
        gold_and_used_silver.0,
        10.pow(9) - gold_and_used_silver.1 .0,
        X - used_bronze
    );
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
