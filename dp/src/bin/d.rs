use proconio::input;

fn main() {
  input! {
    n: usize, w: usize,
    wvv: [(usize, u64); n]
  };

  let mut dp = vec![vec![0_u64; w + 1]; n + 1];
  for i in 0..n {
    let (w1, v1) = wvv[i];
    for j in 0..=w {
      if dp[i][j] > dp[i + 1][j] {
        dp[i + 1][j] = dp[i][j];
      }
      if j + w1 <= w {
        let cost = dp[i][j] + v1;
        if cost > dp[i + 1][j + w1] {
          dp[i + 1][j + w1] = cost;
        }
      }
    }
  }
  println!("{}", dp[n][w]);
}
