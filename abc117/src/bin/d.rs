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
  // 0: same, 1: lower
  let mut dp = vec![vec![-1_i64; 2]; size + 1];
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
    // same -> same
    dp[i + 1][0] = dp[i][0] + value(bit);
    // same -> lower
    if bit == 1 {
      dp[i + 1][1] = dp[i][0] + value(0);
    }
    // lower -> lower
    if dp[i][1] != -1 {
      dp[i + 1][1] = std::cmp::max(dp[i + 1][1], dp[i][1] + std::cmp::max(value(0), value(1)));
    }
  }
  let res = std::cmp::max(dp[size][0], dp[size][1]);
  println!("{}", res);
}
