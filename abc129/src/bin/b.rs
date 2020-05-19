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
    wv: [usize; n]
  };

  let sum: usize = wv.iter().sum();
  let cusum = wv
    .into_iter()
    .scan(0, |acc, w| {
      *acc += w;
      Some(*acc)
    })
    .collect::<Vec<_>>();
  let mut res = std::usize::MAX;
  for t in 0..n - 1 {
    let s1: usize = cusum[t];
    let s2 = sum - s1;
    let diff = (s1 as isize - s2 as isize).abs() as usize;
    if diff < res {
      res = diff;
    }
  }
  println!("{}", res);
}
