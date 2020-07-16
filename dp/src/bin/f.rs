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
        s: Chars,
        t: Chars
    };

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    let mut prev = vec![vec![(0, 0); t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
                prev[i + 1][j + 1] = (i, j);
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
                prev[i + 1][j + 1] = if dp[i + 1][j] > dp[i][j + 1] {
                    (i + 1, j)
                } else {
                    (i, j + 1)
                };
            }
        }
    }
    let mut res = VecDeque::new();
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 && j > 0 {
        if s[i - 1] == t[j - 1] {
            res.push_front(s[i - 1]);
        }
        let (ii, jj) = prev[i][j];
        i = ii;
        j = jj;
    }
    println!("{}", res.into_iter().join(""));
}
