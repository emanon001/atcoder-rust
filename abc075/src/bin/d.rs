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
    xyv: [(i64, i64); n]
  };

  let mut res = std::i64::MAX;
  for cv in xyv.clone().into_iter().combinations(std::cmp::min(k, 4)) {
    let xv = cv.iter().map(|&c| c.0).collect::<Vec<_>>();
    let yv = cv.iter().map(|&c| c.1).collect::<Vec<_>>();
    let l = *xv.iter().min().unwrap();
    let r = *xv.iter().max().unwrap();
    let t = *yv.iter().max().unwrap();
    let b = *yv.iter().min().unwrap();
    let mut c = 0;
    for &(x, y) in &xyv {
      if x >= l && x <= r && y <= t && y >= b {
        c += 1;
      }
    }
    if c >= k {
      let area = (l - r).abs() * (t - b).abs();
      if area < res {
        res = area;
      }
    }
  }
  println!("{}", res);
}
