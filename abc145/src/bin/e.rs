use proconio::input;

fn main() {
  input! {
    n: usize, t: usize,
    mut abv: [(usize, usize); n]
  };

  abv.sort();
  let mut dp = vec![vec![0; 6000]; n + 1];
  for i in 0..n {
    let (a, b) = abv[i];
    for j in 0..6000 {
      dp[i + 1][j] = std::cmp::max(dp[i + 1][j], dp[i][j]);
      if j < t {
        dp[i + 1][j + a] = std::cmp::max(dp[i + 1][j + a], dp[i][j] + b);
      }
    }
  }
  let res = dp[n].iter().max().unwrap();
  println!("{}", res);
}
