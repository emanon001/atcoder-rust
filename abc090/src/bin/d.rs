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
    n: usize, k: usize
  };

  let mut res = 0_u64;
  for b in k + 1..=n {
    let x = n / b * (b - k);
    let m = n % b;
    let y = match m {
      0 => 0,
      _ => {
        if k == 0 {
          m
        } else if m >= k {
          m - k + 1
        } else {
          0
        }
      }
    };
    let c = x + y;
    res += c as u64;
  }
  println!("{}", res);
}
