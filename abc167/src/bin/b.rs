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
    a: isize,
    b: isize,
    c: isize,
    k: isize
  };

  let res: isize = if k <= a {
    k
  } else if k <= a + b {
    a
  } else {
    let rest = k - (a + b);
    a - std::cmp::min(rest, c)
  };
  println!("{}", res);
}
