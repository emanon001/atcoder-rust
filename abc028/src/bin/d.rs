#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
  input! {
    n: usize,
    k: usize
  };

  let all = n * n * n;
  // k, k, k
  let c3 = 1;
  // (k, k, _), (k, _, k), (_, k, k)
  let c2 = (n - 1) * 3;
  // (k, < k, > k), (k, > k, < k)
  // (< k, k, > k), (> k, , < k)
  // (< k, > k, k), (> k, < k, k)
  let c1 = if k == 1 || k == n {
    0
  } else {
    (k - 1) * (n - k) * 6
  };
  let res = (c3 + c2 + c1) as f64 / all as f64;
  println!("{}", res);
}
