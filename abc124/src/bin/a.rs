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
    a: usize, b: usize
  };

  let res = if a == b {
    a * 2
  } else {
    let max = std::cmp::max(a, b);
    max + max - 1
  };
  println!("{}", res);
}
