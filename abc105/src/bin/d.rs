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
  let mut cur = 0;
  for &a in &av {
    cur = (cur + a) % m;
    *table.entry(cur).or_insert(0) += 1;
  }
  let mut res = 0_u64;
  let mut cur = 0;
  for a in av {
    cur = (cur + a) % m;
    let b = if a % m >= cur {
      (m - (a % m - cur)) % m
    } else {
      cur - a % m
    };
    if let Some(x) = table.get(&b) {
      res += *x;
    }
    if let Some(x) = table.get_mut(&cur) {
      if *x > 0 {
        *x -= 1;
      }
    }
  }
  println!("{}", res);
}
