#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

const MOD: usize = 1_000_000_007;

fn main() {
  input! {
    n: usize, m: usize,
    av: [usize; m]
  };

  let set = av.into_iter().collect::<HashSet<_>>();
  let mut dp = vec![0; n + 1];
  dp[0] = 1;
  for i in 0..n {
    let j = i + 1;
    if j <= n && !set.contains(&j) {
      dp[j] = (dp[j] + dp[i]) % MOD;
    }
    let j = i + 2;
    if j <= n && !set.contains(&j) {
      dp[j] = (dp[j] + dp[i]) % MOD;
    }
  }
  println!("{}", dp[n]);
}
