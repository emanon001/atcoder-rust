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

  let max = a.max(b).max(c);
  let res = max * 10 + a + b + c - max;
  println!("{}", res);
}
