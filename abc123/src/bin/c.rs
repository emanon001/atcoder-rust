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
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
  };

  let min = a.min(b).min(c).min(d).min(e);
  let c = (n + min - 1) / min;
  let res = 5 + c - 1;
  println!("{}", res);
}
