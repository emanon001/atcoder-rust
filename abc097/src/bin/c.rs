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
    s: String,
    k: usize
  };

  let mut set = HashSet::new();
  for i in 0..s.len() {
    for j in i + 1..=i + k {
      if j > s.len() {
        break;
      }
      set.insert(&s[i..j]);
    }
  }
  let mut v = set.into_iter().collect::<Vec<_>>();
  v.sort();
  println!("{}", v[k - 1]);
}
