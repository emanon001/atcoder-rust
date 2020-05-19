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
    p: usize, q: usize, r: usize
  };
  let res = vec![
    // a, b, c
    p + q,
    // a, c, b
    r + q,
    // b, a, c
    p + r,
    // b, c, a
    q + r,
    // c, a, b
    r + p,
    // c, b, a
    q + p,
  ]
  .into_iter()
  .min()
  .unwrap();
  println!("{}", res);
}
