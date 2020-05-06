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
    n: usize, c: usize,
    av: [i32; n]
  };

  let mut res = 1 << 32;
  for cv in (1..=10).combinations(2) {
    let cl1 = cv[0];
    let cl2 = cv[1];
    // c1, c2, c1, ...
    let mut sum = 0;
    for i in 0..n {
      let cl = if i % 2 == 0 { cl1 } else { cl2 };
      if av[i] != cl {
        sum += c;
      }
    }
    if sum < res {
      res = sum;
    }
    // c2, c1, c2, ...
    let mut sum = 0;
    for i in 0..n {
      let cl = if i % 2 == 0 { cl2 } else { cl1 };
      if av[i] != cl {
        sum += c;
      }
    }
    if sum < res {
      res = sum;
    }
  }
  println!("{}", res);
}
