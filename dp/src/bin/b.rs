use proconio::input;

fn main() {
  input! {
    n: usize, k: usize,
    hv: [i64; n]
  };

  let inf = 1_i64 << 60;
  let mut dp = vec![inf; n];
  dp[0] = 0;
  for i in 0..n {
    for j in (i + 1)..=(i + k) {
      if j >= n {
        continue;
      }
      let cost = dp[i] + (hv[i] - hv[j]).abs();
      if cost < dp[j] {
        dp[j] = cost;
      }
    }
  }
  println!("{}", dp[n - 1]);
}
