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
    _: usize, m: usize, x: usize,
    av: [usize; m]
  };

  let res = std::cmp::min(
    av.iter().filter(|&a| *a < x).count(),
    av.iter().filter(|&a| *a > x).count(),
  );
  println!("{}", res);
}
