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
    s: Chars
  };

  let mut red_c = 0;
  let mut blue_c = 0;
  for ch in s {
    if ch == '0' {
      red_c += 1;
    } else {
      blue_c += 1;
    }
  }
  let res = std::cmp::min(red_c, blue_c) * 2;
  println!("{}", res);
}
