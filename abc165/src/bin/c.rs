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
    n: usize, m: usize, q: usize,
    abcdv: [(Usize1, Usize1, usize, usize); q]
  };

  let mut res = 0;
  for v in (1..=m).combinations_with_replacement(n) {
    let mut v = v;
    v.sort();
    let mut sum = 0;
    for (a, b, c, d) in &abcdv {
      if v[*b] - v[*a] == *c {
        sum += d;
      }
    }
    if sum > res {
      res = sum;
    }
  }
  println!("{}", res);
}
