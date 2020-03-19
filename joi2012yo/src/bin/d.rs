use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

const MOD: u64 = 10000;

fn main() {
  input! {
    n: usize, k: usize,
    schedules: [(Usize1, usize); k]
  };
  let schedules: HashMap<_, _> = schedules.into_iter().collect();
  let mut dp = vec![vec![vec![0; 4]; 4]; n + 1];
  dp[0][0][0] = 1_u64;
  for i in 0..n {
    let ps = match schedules.get(&i) {
      Some(&p) => vec![p],
      _ => (1..4).collect(),
    };
    for p in ps {
      for p1 in 0..4 {
        for p2 in 0..4 {
          if p == p1 && p1 == p2 {
            continue;
          }
          dp[i + 1][p][p1] = dp[i + 1][p][p1] + (dp[i][p1][p2] % MOD) % MOD;
        }
      }
    }
  }
  let mut res = 0_u64;
  for i in 1..4 {
    for j in 1..4 {
      res = (res + dp[n][i][j]) % MOD;
    }
  }
  println!("{}", res);
}
