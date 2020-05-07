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

  // 0, 1, 0, 1, ...
  let mut count1 = 0;
  for i in 0..s.len() {
    let color = if i % 2 == 0 { '0' } else { '1' };
    if s[i] != color {
      count1 += 1;
    }
  }
  // 1, 0, 1, 0, ...
  let mut count2 = 0;
  for i in 0..s.len() {
    let color = if i % 2 == 0 { '1' } else { '0' };
    if s[i] != color {
      count2 += 1;
    }
  }
  let res = std::cmp::min(count1, count2);
  println!("{}", res);
}
