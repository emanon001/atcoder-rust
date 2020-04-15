#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn dfs(i: usize, acc: u8, n: usize, k: usize, answers: &[Vec<u8>]) -> bool {
  if i == n {
    return acc == 0;
  }
  (0..k)
    .into_iter()
    .any(|j| dfs(i + 1, acc ^ answers[i][j], n, k, answers))
}

fn main() {
  input! {
    n: usize, k: usize,
    answers: [[u8; k]; n]
  };

  let res = if dfs(0, 0, n, k, &answers) {
    "Found"
  } else {
    "Nothing"
  };
  println!("{}", res);
}
