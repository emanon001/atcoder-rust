#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn divisors(n: u64) -> Vec<u64> {
  let mut res = Vec::new();
  let mut x = 1;
  while x * x <= n {
    if n % x == 0 {
      res.push(x);
      let y = n / x;
      if y != x {
        res.push(y);
      }
    }
    x += 1;
  }
  res
}

fn main() {
  input! {
    n: u64, m: u64
  };

  let mut res = 0;
  for d in divisors(m) {
    if d >= n {
      let a = m / d;
      if a > res {
        res = a;
      }
    }
  }
  println!("{}", res);
}
