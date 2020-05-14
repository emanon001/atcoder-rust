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
    a: isize, b: isize, c: isize, d: isize
  };

  let res = if (a - c).abs() <= d || ((a - b).abs() <= d && (b - c).abs() <= d) {
    "Yes"
  } else {
    "No"
  };
  println!("{}", res);
}
