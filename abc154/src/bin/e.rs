#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: Chars,
        k: usize
    };

    let digits = n
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let len = digits.len();
    // dp[i][j][k] = 個数
    // i: 何桁目まで見たか
    // j: 0: nと同じ、1: n未満
    // k: 0 でない数字の数
    let mut dp = vec![vec![vec![None; k + 10]; 2]; len + 1];
    dp[0][0][0] = Some(1_u64);
    for i in 0..len {
        for j in 0..2 {
            let cur_d = digits[i];
            let max_d = if j == 0 { cur_d } else { 9 };
            for d in 0..=max_d {
                let to_i = i + 1;
                let to_j = if j == 1 || d < cur_d { 1 } else { 0 };
                for ik in 0..=k {
                    let to_k = ik + if d == 0 { 0 } else { 1 };
                    let v = match (dp[to_i][to_j][to_k], dp[i][j][ik]) {
                        (x, None) => x,
                        (None, x) => x,
                        (Some(x), Some(y)) => Some(x + y),
                    };
                    dp[to_i][to_j][to_k] = v;
                }
            }
        }
    }
    let res = dp[len][0][k].unwrap_or(0) + dp[len][1][k].unwrap_or(0);
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
