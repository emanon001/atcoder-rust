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
    x: usize
  };

  let mut res = 1;
  for a in 2..=num::integer::sqrt(x) {
    let mut b = a * a;
    while b * a <= x {
      b *= a;
    }
    if b > res {
      res = b;
    }
  }
  println!("{}", res);
}
