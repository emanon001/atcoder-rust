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
    s: usize, w: usize
  };

  let res = if w >= s { "unsafe" } else { "safe" };
  println!("{}", res);
}
