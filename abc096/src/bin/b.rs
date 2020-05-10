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
    a: usize, b: usize, c: usize,
    k: usize
  };

  let max = a.max(b).max(c);
  let mut x = max;
  for _ in 0..k {
    x *= 2;
  }
  let res = a + b + c - max + x;
  println!("{}", res);
}
