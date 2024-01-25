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
        N: usize, A: usize, B: usize,
        WV: [(usize, usize); N]
    };

    let mut dp = vec![vec![vec![0; B + 1]; A + 1]; N + 1];
    for (i, (w, v)) in WV.into_iter().enumerate() {
        for a in 0..=A {
            for b in 0..=B {
                // ナップサックに入れない場合
                chmax!(dp[i + 1][a][b], dp[i][a][b]);
                // 1つ目のナップサックに入れる場合
                if a + w <= A {
                    chmax!(dp[i + 1][a + w][b], dp[i][a][b] + v);
                }
                // 2つ目のナップサックに入れる場合
                if b + w <= B {
                    chmax!(dp[i + 1][a][b + w], dp[i][a][b] + v);
                }
            }
        }
    }
    let mut ans = 0;
    for a in 0..=A {
        for b in 0..=B {
            chmax!(ans, dp[N][a][b]);
        }
    }
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
