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
        A: [usize; N],
    };

    let a_sum = A.iter().sum::<usize>();
    // dp[i][j][k] = i番目まで見た時に、Y座標の総和がjで、直前のY座標がkの時の線分の長さの最小値
    let mut dp = vec![vec![vec![(1 << 30) as f64; 100 + 1]; 100 + 1]; N];
    dp[0][0][0] = 0_f64;
    for i in 0..N - 1 {
        for j in 0..=100 {
            for k in 0..=100 {
                // Y座標
                let l_max = if i == N - 2 { 0 } else { 100 };
                for l in 0..=l_max {
                    if j + l > a_sum {
                        break;
                    }
                    let new_i = i + 1;
                    let new_j = j + l;
                    let new_k = l;
                    let distance = (1_f64 + (k as f64 - l as f64).powf(2.0)).sqrt();
                    chmin!(dp[new_i][new_j][new_k], dp[i][j][k] + distance);
                }
            }
        }
    }
    let ans = dp[N - 1][a_sum][0];
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
