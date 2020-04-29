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

  let k_is_even = k % 2 == 0;
  let mut zero_c = 0_u64;
  let mut halfk_c = 0_u64;
  for x in 1..=n {
    if x % k == 0 {
      zero_c += 1;
    }
    if k_is_even && x % k == k / 2 {
      halfk_c += 1;
    }
  }
  let res = zero_c.pow(3) + halfk_c.pow(3);
  println!("{}", res);
}
