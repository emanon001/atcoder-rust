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
    a: usize, b: usize, k: usize
  };

  let mut c = 0;
  for x in (1..=std::cmp::max(a, b)).rev() {
    if a % x == 0 && b % x == 0 {
      c += 1;
    }
    if c == k {
      println!("{}", x);
      std::process::exit(0);
    }
  }
}
