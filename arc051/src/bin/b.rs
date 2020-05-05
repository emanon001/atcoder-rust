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
    k: usize
  };

  let mut a = 1_u64;
  let mut b = 1_u64;
  for _ in 0..k {
    let t = a;
    a = a + b;
    b = t;
  }
  println!("{} {}", a, b);
}
