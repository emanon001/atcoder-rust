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
    n: usize, k: usize, m: usize, r: usize,
    mut sv: [usize; n - 1]
  };

  let required = k * r;
  sv.sort_by_key(|&x| -(x as isize));
  if n == k {
    let sum: usize = sv.into_iter().sum();
    if sum >= required {
      println!("0");
    } else if sum + m < required {
      println!("-1");
    } else {
      println!("{}", required - sum);
    }
  } else {
    let sum: usize = sv[0..k].into_iter().sum();
    if sum >= required {
      println!("0");
    } else if sum - sv[k - 1] + m < required {
      println!("-1");
    } else {
      println!("{}", required - (sum - sv[k - 1]));
    }
  }
}
