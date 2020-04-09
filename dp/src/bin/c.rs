use proconio::input;

fn main() {
  input! {
    n: usize,
    abcv: [[usize; 3]; n]
  };

  let mut dp = vec![[0; 3]; n + 1];
  for i in 0..n {
    let abc = &abcv[i];
    for j in 0..3 {
      for k in 0..3 {
        if j == k {
          continue;
        }
        let cost = dp[i][j] + abc[k];
        if cost > dp[i + 1][k] {
          dp[i + 1][k] = cost;
        }
      }
    }
  }
  println!("{}", dp[n].iter().max().unwrap());
}
