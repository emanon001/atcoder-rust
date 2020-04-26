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
  };

  let res = if n <= 59 {
    "Bad"
  } else if n >= 60 && n <= 89 {
    "Good"
  } else if n >= 90 && n <= 99 {
    "Great"
  } else {
    "Perfect"
  };
  println!("{}", res);
}
