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
    n: usize,
    hv: [usize; n]
  };

  let mut max = 0;
  let mut res = 0;
  for h in hv {
    if h >= max {
      max = h;
      res += 1;
    }
  }
  println!("{}", res);
}
