#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

const MOD: usize = 1_000_000_007;

fn mod_factrial(n: usize) -> usize {
  let mut res = 1;
  for x in 1..n + 1 {
    res = (res * x) % MOD;
  }
  res
}

fn main() {
  input! {
    n: usize,
    mut tv: [usize; n]
  };

  tv.sort();
  let mut min = 0;
  let mut cur = 0;
  for &t in &tv {
    cur += t;
    min += cur;
  }
  let mut table = BTreeMap::new();
  for t in tv {
    let c = table.entry(t).or_insert(0);
    *c += 1;
  }
  let mut count = 1;
  for (_, c) in table {
    count = (count * mod_factrial(c)) % MOD;
  }
  println!("{}", min);
  println!("{}", count);
}
