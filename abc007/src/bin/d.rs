#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn count(a: u64) -> u64 {
  let nv = a
    .to_string()
    .chars()
    .map(|ch| ch.to_digit(10).unwrap())
    .collect::<Vec<_>>();
  let len = nv.len();
  // dp[i][j][k]
  // i: digit
  // j: 0: aと同じ, 1: a未満
  // k: 0: {4, 9}を含まない、1: {4, 9}を含む
  let mut dp = vec![vec![vec![0; 2]; 2]; len + 1];
  dp[0][0][0] = 1;
  for i in 0..len {
    let n = nv[i];
    for m in 0..=9 {
      let contains_49 = m == 4 || m == 9;
      // same -> same
      if n == m {
        if contains_49 {
          dp[i + 1][0][1] += dp[i][0][0] + dp[i][0][1];
        } else {
          dp[i + 1][0][0] += dp[i][0][0];
          dp[i + 1][0][1] += dp[i][0][1];
        };
      }
      // same -> smaller
      if m < n {
        if contains_49 {
          dp[i + 1][1][1] += dp[i][0][0] + dp[i][0][1];
        } else {
          dp[i + 1][1][0] += dp[i][0][0];
          dp[i + 1][1][1] += dp[i][0][1];
        }
      }
      // smaller -> smaller
      if contains_49 {
        dp[i + 1][1][1] += dp[i][1][0] + dp[i][1][1];
      } else {
        dp[i + 1][1][0] += dp[i][1][0];
        dp[i + 1][1][1] += dp[i][1][1];
      }
    }
  }
  dp[len][0][1] + dp[len][1][1]
}

fn main() {
  input! {
    a: u64,
    b: u64,
  };

  let res = count(b) - count(a - 1);
  println!("{}", res);
}
