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
    av: [usize; 5]
  };

  let mut sums = av
    .into_iter()
    .combinations(3)
    .map(|c| c.into_iter().sum::<usize>())
    .collect::<Vec<_>>();
  sums.sort_by_key(|&a| -(a as isize));
  println!("{}", sums[2]);
}
