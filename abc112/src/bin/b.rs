#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

const INF: usize = 1 << 60;

fn main() {
  input! {
    n: usize, t: usize,
    ctv: [(usize, usize); n]
  };

  let mut res = INF;
  for (c, t2) in ctv {
    if t2 <= t && c < res {
      res = c;
    }
  }
  let res = if res == INF {
    "TLE".to_string()
  } else {
    res.to_string()
  };
  println!("{}", res);
}
