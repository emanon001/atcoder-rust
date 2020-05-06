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
    x: usize, y: usize,
    k: usize
  };

  let a = std::cmp::min(y, k);
  let b = k - a;
  let res = a + x - b;
  println!("{}", res);
}
