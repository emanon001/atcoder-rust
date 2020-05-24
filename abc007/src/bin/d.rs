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
  // i: 何桁目
  // j: 0: aと同じ, 1: a未満
  // k: 0: {4, 9}を含まない、1: {4, 9}を含む
  let mut dp = vec![vec![vec![0; 2]; 2]; len + 1];
  dp[0][0][0] = 1;
  for i in 0..len {
    let n = nv[i];
    for j in 0..2 {
      for k in 0..2 {
        let d = if j == 0 { n } else { 9 };
        for m in 0..=d {
          let to_j = if j == 1 || m < n { 1 } else { 0 };
          let to_k = if k == 1 || (m == 4 || m == 9) { 1 } else { 0 };
          dp[i + 1][to_j][to_k] += dp[i][j][k];
        }
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
