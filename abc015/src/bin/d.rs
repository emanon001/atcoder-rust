#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    w: usize,
    n: usize, k: usize,
    abv: [(usize, usize); n]
  };

  let mut dp = vec![vec![vec![0; k + 1]; w + 1]; n + 1];
  for i in 0..n {
    let (a, b) = abv[i];
    for j in 0..=w {
      for k2 in 0..=k {
        dp[i + 1][j][k2] = std::cmp::max(dp[i + 1][j][k2], dp[i][j][k2]);
        if k2 + 1 > k {
          continue;
        }
        if j + a > w {
          continue;
        }
        dp[i + 1][j + a][k2 + 1] = std::cmp::max(dp[i + 1][j + a][k2 + 1], dp[i][j][k2] + b);
      }
    }
  }
  let mut res = 0;
  for i in 0..=k {
    res = std::cmp::max(dp[n][w][i], res);
  }
  println!("{}", dp[n][w][k]);
}
