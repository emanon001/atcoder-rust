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

  let res = (a..=b)
    .filter(|&x| x.to_string() == x.to_string().chars().rev().collect::<String>())
    .count();
  println!("{}", res);
}
