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
    a: usize, b: usize, x: usize
  };

  let res = if a <= x && a + b >= x { "YES" } else { "NO" };
  println!("{}", res);
}
