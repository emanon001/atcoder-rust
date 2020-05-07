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
    a: usize, b: usize, c: usize
  };

  let res = std::cmp::min(b / a, c);
  println!("{}", res);
}
