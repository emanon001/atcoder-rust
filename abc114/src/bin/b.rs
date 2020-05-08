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
    s: String
  };

  let mut res = 1000;
  for i in 0..s.len() {
    if i + 3 > s.len() {
      break;
    }
    let n = (753 - &s[i..i + 3].parse::<isize>().unwrap()).abs();
    if n < res {
      res = n;
    }
  }
  println!("{}", res);
}
