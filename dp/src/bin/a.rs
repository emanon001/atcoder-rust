use proconio::input;

fn main() {
  input! {
    n: usize,
    hv: [i64; n]
  };

  let inf = 1_i64 << 60;
  let mut dp = vec![inf; n];
  dp[0] = 0;
  for i in 0..n {
    for j in (i + 1)..=(i + 2) {
      if j < n {
        let cost = dp[i] + (hv[i] - hv[j]).abs();
        if cost < dp[j] {
          dp[j] = cost;
        }
      }
    }
  }
  println!("{}", dp[n - 1]);
}
