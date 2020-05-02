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
    mut x: usize
  };

  let mut cur = 100;
  let mut res = 0;
  while cur < x {
    res += 1;
    cur += cur / 100;
  }
  println!("{}", res);
}
