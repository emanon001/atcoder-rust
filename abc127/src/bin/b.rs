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
    r: usize, d: usize, mut x: usize
  };

  for _ in 0..10 {
    x = r * x - d;
    println!("{}", x);
  }
}
