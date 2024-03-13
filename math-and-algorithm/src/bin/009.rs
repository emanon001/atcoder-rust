#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, S: usize,
        A: [usize; N],
    };

    let mut dp = vec![vec![false; S + 1]; N + 1];
    dp[0][0] = true;
    for i in 0..N {
        for j in 0..=S {
            dp[i + 1][j] |= dp[i][j];
            if j + A[i] <= S {
                dp[i + 1][j + A[i]] |= dp[i][j];
            }
        }
    }
    let ans = if dp[N][S] { "Yes" } else { "No" };
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
