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
    n: usize, k: usize,
    av: [[usize]; k]
  };

  let mut set = HashSet::new();
  for a in av {
    for b in a {
      set.insert(b);
    }
  }
  let res = n - set.len();
  println!("{}", res);
}
