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
    s: String,
    t: String
  };

  let res = if s == &t[0..s.len()] { "Yes" } else { "No" };
  println!("{}", res);
}
