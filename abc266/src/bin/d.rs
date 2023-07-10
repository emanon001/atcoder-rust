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
        txav: [(usize, usize, u64); n]
    };

    let max_t = 10.pow(5);
    let mut t_to_xa = vec![None; max_t + 1];
    for (t, x, a) in txav {
        t_to_xa[t] = Some((x, a));
    }
    let mut dp = vec![vec![0_u64; 5]; 10.pow(5) + 10];
    for i in 0..=max_t {
        for j in 0..(i + 1).min(5) {
            let (x, a) = t_to_xa[i].unwrap_or((100, 0));
            let add = if x == j { a } else { 0 };
            // 左
            if j > 0 {
                let from_j = j - 1;
                chmax!(dp[i + 1][j], dp[i][from_j] + add);
            };
            // 移動なし
            chmax!(dp[i + 1][j], dp[i][j] + add);
            // 右
            if j < 4 {
                let from_j = j + 1;
                chmax!(dp[i + 1][j], dp[i][from_j] + add);
            };
        }
    }
    let res = dp[max_t + 1].iter().max().unwrap();
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
