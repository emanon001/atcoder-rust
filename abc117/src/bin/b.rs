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
    n: usize,
    lv: [usize; n]
  };

  let sum: usize = lv.iter().sum();
  let max = lv.into_iter().max().unwrap();
  let res = if max < sum - max { "Yes" } else { "No" };
  println!("{}", res);
}
