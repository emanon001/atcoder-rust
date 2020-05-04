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
    n: usize, m: usize,
    hv: [usize; n],
    edges: [(Usize1, Usize1); m]
  };

  let mut max_h = hv.clone();
  for (a, b) in edges {
    max_h[a] = std::cmp::max(max_h[a], hv[b] + 1);
    max_h[b] = std::cmp::max(max_h[b], hv[a] + 1);
  }
  let mut res = 0;
  for i in 0..n {
    if hv[i] == max_h[i] {
      res += 1;
    }
  }
  println!("{}", res);
}
