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
    a: u64, b: u64, k: u64, l: u64
  };

  let res = vec![a * k, b * ((k + l - 1) / l), b * (k / l) + a * (k % l)]
    .into_iter()
    .min()
    .unwrap();
  println!("{}", res);
}
