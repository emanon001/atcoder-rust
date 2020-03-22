use proconio::input;
use proconio::marker::Chars;

fn min_count(i: usize, j: usize, grid: &[Vec<char>], dp: &mut [Vec<i64>]) -> (bool, i64) {
  let is_black = grid[i][j] == '#';
  if dp[i][j] >= 0 {
    return (is_black, dp[i][j]);
  }
  if i == 0 && j == 0 {
    let res = if is_black { 1 } else { 0 };
    dp[i][j] = res;
    return (is_black, res);
  }
  let mut res = 1 << 60;
  if i > 0 {
    let (b, c) = min_count(i - 1, j, grid, dp);
    res = match (is_black, b) {
      (true, true) => c,
      (true, false) => c + 1,
      (false, true) => c,
      (false, false) => c,
    };
  }
  if j > 0 {
    let (b, c) = min_count(i, j - 1, grid, dp);
    res = std::cmp::min(
      res,
      match (is_black, b) {
        (true, true) => c,
        (true, false) => c + 1,
        (false, true) => c,
        (false, false) => c,
      },
    );
  }
  dp[i][j] = res;
  (is_black, res)
}

fn main() {
  input! {
    h: usize, w: usize,
    grid: [Chars; h]
  };

  let mut dp = vec![vec![-1; w]; h];
  let (_, res) = min_count(h - 1, w - 1, &grid, &mut dp);
  println!("{}", res);
}
