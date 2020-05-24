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
    n: usize, k: usize,
    av: [u64; n]
  };

  let size = 60;
  // dp[i][j]
  // i: 何桁目
  // j: 0: same, 1: smaller
  let mut dp = vec![vec![-1; 2]; size + 1];
  dp[0][0] = 0;
  for i in 0..size {
    let d = size - 1 - i;
    let mut zero_c = 0;
    let mut one_c = 0;
    for &a in &av {
      if (a >> d) & 1 == 1 {
        one_c += 1;
      } else {
        zero_c += 1;
      }
    }
    let value = |b| -> i64 {
      let v = if b == 1 { zero_c } else { one_c };
      v * 2_i64.pow(d as u32)
    };
    let bit = (k >> d) & 1;
    for j in 0..2 {
      if j == 1 && dp[i][j] == -1 {
        continue;
      }
      let max_v = if j == 1 { 1 } else { bit };
      for b in 0..=max_v {
        let to_j = if j == 1 || b < max_v { 1 } else { 0 };
        dp[i + 1][to_j] = std::cmp::max(dp[i + 1][to_j], dp[i][j] + value(b));
      }
    }
  }
  let res = std::cmp::max(dp[size][0], dp[size][1]);
  println!("{}", res);
}
