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
    pv: [usize; n]
  };

  let sum = pv.iter().sum::<usize>();
  let max = pv.iter().max().unwrap();
  let res = sum - max / 2;
  println!("{}", res);
}
