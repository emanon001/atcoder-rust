#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve(n: usize, m: u64, x: u64, pcusum: &[u64]) -> u64 {
  if x == 0 {
    return 0;
  }
  if n == 0 {
    return 1;
  }
  if x == m {
    return pcusum[n];
  }
  let mid = m / 2 + 1;
  let mut res = 0;
  let bsize = (m - 3) / 2;
  let bi = std::cmp::min(x - 1, bsize);
  res += solve(n - 1, bsize, bi, pcusum);
  if x >= mid {
    res += 1;
    let bi = std::cmp::min(x - mid, bsize);
    res += solve(n - 1, bsize, bi, pcusum);
  }
  res
}

fn main() {
  input! {
    n: usize, x: u64
  };

  let mut size = 1;
  let mut pcusum = vec![1_u64; n + 1];
  pcusum.push(1);
  for i in 0..n {
    pcusum[i + 1] = pcusum[i] * 2 + 1;
    size = size * 2 + 3;
  }

  let res = solve(n, size, x, &pcusum);
  println!("{}", res);
}
