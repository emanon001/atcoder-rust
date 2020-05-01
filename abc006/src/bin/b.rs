#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

const MOD: isize = 10_007;

fn solve(n: usize, dp: &mut [isize]) -> isize {
  if dp[n] > -1 {
    return dp[n];
  }
  match n {
    1 => dp[n] = 0,
    2 => dp[n] = 0,
    3 => dp[n] = 1,
    _ => dp[n] = (solve(n - 1, dp) + solve(n - 2, dp) + solve(n - 3, dp)) % MOD,
  };
  dp[n]
}

fn main() {
  input! {
    n: usize
  };
  let mut dp = vec![-1; n + 1];
  println!("{}", solve(n, &mut dp));
}
