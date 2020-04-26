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
    av: [i64; n]
  };

  let mut table = HashMap::new();
  table.insert(0, 1);
  let mut cur = 0_i64;
  for a in av {
    cur += a;
    let x = table.entry(cur).or_insert(0);
    *x += 1;
  }
  let mut res = 0_u64;
  for (_, c) in table {
    res += c * (c - 1) / 2;
  }
  println!("{}", res);
}
