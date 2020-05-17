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
    s: String
  };

  if k >= s.len() {
    println!("{}", s);
  } else {
    println!("{}{}", &s[0..k], "...");
  }
}
