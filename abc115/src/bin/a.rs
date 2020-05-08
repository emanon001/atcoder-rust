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
    d: usize
  };

  let mut res = "Christmas".to_string();
  for _ in 0..25 - d {
    res += " Eve";
  }
  println!("{}", res);
}
