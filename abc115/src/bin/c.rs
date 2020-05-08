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
    n: usize, k: usize,
    mut hv: [usize; n]
  };

  hv.sort();
  let mut res = 1 << 30;
  for i in 0..n {
    let j = i + k - 1;
    if j >= n {
      break;
    }
    let diff = hv[j] - hv[i];
    if diff < res {
      res = diff;
    }
  }
  println!("{}", res);
}
