#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    k: usize,
  };

  let mut res = 0_usize;
  for a in 1..k + 1 {
    for b in 1..k + 1 {
      for c in 1..k + 1 {
        res += a.gcd(&b).gcd(&c);
      }
    }
  }
  println!("{}", res);
}
