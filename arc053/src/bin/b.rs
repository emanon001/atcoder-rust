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
    s: Chars
  };

  let all_count = s.len();
  let mut table = HashMap::new();
  for ch in s {
    let c = table.entry(ch).or_insert(0);
    *c += 1;
  }
  let mut n = 0;
  for &c in table.values() {
    if c % 2 == 1 {
      n += 1;
    }
  }
  let res = if n <= 1 {
    all_count
  } else {
    (all_count - n) / 2 / n * 2 + 1
  };
  println!("{}", res);
}
