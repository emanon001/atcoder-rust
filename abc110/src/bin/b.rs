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
    n: usize, m: usize, x: isize, y: isize,
    xv: [isize; n],
    yv: [isize; m],
  };

  let is_ok = (x + 1..=y).any(|z| {
    for i in 0..n {
      if xv[i] >= z {
        return false;
      }
    }
    for i in 0..m {
      if yv[i] < z {
        return false;
      }
    }
    true
  });
  let res = if is_ok { "No War" } else { "War" };
  println!("{}", res);
}
