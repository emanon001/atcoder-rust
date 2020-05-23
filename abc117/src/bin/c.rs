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
    n: usize, m: usize,
    mut xv: [isize; m]
  };

  if n >= m {
    println!("0");
    std::process::exit(0);
  }

  xv.sort();
  let mut dv = xv
    .windows(2)
    .map(|v| (v[0] - v[1]).abs())
    .collect::<Vec<_>>();
  dv.sort_by_key(|k| -k);

  let k = n - 1;
  let res = dv[k..].into_iter().fold(0, |acc, x| acc + x);
  println!("{}", res);
}
