#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: Chars
    };

    let digits = n
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let len = digits.len();

    // dp[i][j] = (総数, 1の総数)
    // i: 何桁目
    // j: 0: nと同じ、1: n未満
    let mut dp = vec![vec![(0, 0); 2]; len + 1];
    dp[0][0].0 = 1;
    for i in 0..len {
        let d = digits[i];
        for j in 0..2 {
            if i == 0 && j == 1 {
                continue;
            }
            let max_d = if j == 0 { d } else { 9 };
            for m in 0..=max_d {
                let to_j = if j == 1 || m < d { 1 } else { 0 };
                dp[i + 1][to_j].0 += dp[i][j].0;
                dp[i + 1][to_j].1 += dp[i][j].1;
                if m == 1 {
                    dp[i + 1][to_j].1 += dp[i][j].0;
                }
            }
        }
    }
    println!("{}", dp[len][0].1 + dp[len][1].1);
}
