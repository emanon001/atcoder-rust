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
    xv: [usize; n]
  };

  let mut xv = xv.into_iter().enumerate().collect::<Vec<_>>();
  xv.sort_by_key(|(_, x)| *x);
  let mut indexes = vec![0; n];
  for i in 0..n {
    indexes[xv[i].0] = i;
  }
  for i in 0..n {
    let j = indexes[i];
    let res = if j < n / 2 {
      xv[n / 2].1
    } else {
      xv[n / 2 - 1].1
    };
    println!("{}", res);
  }
}
