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
    av: [usize; n]
  };

  let mut table = HashMap::new();
  table.insert(0, 1);
  let mut cur = 0;
  for &a in &av {
    cur = (cur + a) % m;
    *table.entry(cur).or_insert(0) += 1;
  }
  let mut res = 0_u64;
  for (_, c) in table {
    res += c * (c - 1) / 2;
  }
  println!("{}", res);
}
