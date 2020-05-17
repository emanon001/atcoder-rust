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
    p + q, // a -> b -> c
    r + q, // a -> c -> b
    p + r, // b -> a -> c
    q + r, // b -> c -> a
    r + p, // c -> a -> b
    q + p, // c -> b -> a
  ]
  .into_iter()
  .min()
  .unwrap();
  println!("{}", res);
}
