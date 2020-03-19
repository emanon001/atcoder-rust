use proconio::input;

struct Foo(u32);

fn main() {
  input! {
    n: usize,
    ns: [usize; n]
  };

  let mut dp = vec![vec![0_u64; 21]; n];
  dp[0][0] = 1;
  for i in 0..(n - 1) {
    let n = ns[i];
    for j in 0..=20 {
      if j + n <= 20 {
        dp[i + 1][j + n] += dp[i][j];
      }
      if i > 0 && j >= n {
        dp[i + 1][j - n] += dp[i][j];
      }
    }
  }
  println!("{}", dp[n - 1][ns[n - 1]]);
}
