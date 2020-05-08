#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve(i: usize, acc: usize, n: usize) -> usize {
  if i == 0 {
    return 0;
  }
  let mut res = 0;
  for x in vec![7, 5, 3] {
    let m = acc * 10 + x;
    if m <= n && m.to_string().chars().collect::<HashSet<_>>().len() == 3 {
      res += 1;
    }
    res += solve(i - 1, m, n);
  }
  res
}

fn main() {
  input! {
    n: usize
  };

  let d = n.to_string().len();
  let res = solve(d, 0, n);
  println!("{}", res);
}
