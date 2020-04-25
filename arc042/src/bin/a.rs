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
    av: [usize; m]
  };

  let mut used = HashSet::new();
  let mut res = Vec::new();
  for a in av.into_iter().rev() {
    if used.contains(&a) {
      continue;
    }
    used.insert(a);
    res.push(a);
  }
  for a in 1..=n {
    if used.contains(&a) {
      continue;
    }
    res.push(a);
  }
  for a in res {
    println!("{}", a);
  }
}
