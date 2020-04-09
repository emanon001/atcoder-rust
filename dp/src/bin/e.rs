use proconio::input;

fn main() {
  input! {
    n: usize, w: usize,
    wvv: [(usize, usize); n]
  };

  let max_v = 100_000;
  let mut dp = vec![vec![w + 1; max_v + 1]; n + 1];
  dp[0][0] = 0;
  for i in 0..n {
    let (w1, v1) = wvv[i];
    for j in 0..=max_v {
      if dp[i][j] < dp[i + 1][j] {
        dp[i + 1][j] = dp[i][j];
      }
      if j + v1 > max_v {
        continue;
      }
      let cost = dp[i][j] + w1;
      if cost < dp[i + 1][j + v1] {
        dp[i + 1][j + v1] = cost;
      }
    }
  }
  let mut res = 0;
  for i in 0..=max_v {
    if dp[n][i] <= w {
      res = i;
    }
  }
  println!("{}", res);
}
