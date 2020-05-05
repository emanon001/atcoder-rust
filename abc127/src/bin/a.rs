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
    a: usize, b: usize
  };

  let res = if a >= 13 {
    b
  } else if a >= 6 {
    b / 2
  } else {
    0
  };
  println!("{}", res);
}
