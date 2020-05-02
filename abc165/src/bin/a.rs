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
    k: usize,
    a: usize, b:usize
  };

  for x in a..b + 1 {
    if x % k == 0 {
      println!("OK");
      std::process::exit(0);
    }
  }
  println!("NG");
}
