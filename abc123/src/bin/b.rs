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
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize
  };

  let mut res = 1_usize << 30;
  let cv = vec![a, b, c, d, e];
  for v in cv.into_iter().permutations(5) {
    let mut cur = 0;
    for x in v {
      if cur % 10 == 0 {
        cur += x;
      } else {
        cur += 10;
        cur -= cur % 10;
        cur += x;
      }
    }
    if cur < res {
      res = cur;
    }
  }
  println!("{}", res);
}
