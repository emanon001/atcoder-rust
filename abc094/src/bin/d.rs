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
    mut av: [usize; n]
  };

  let i = *av.iter().max().unwrap();
  let mut skipped = false;
  let mut rj = 0;
  let mut diff = 1 << 30;
  for j in av {
    if i == j && !skipped {
      skipped = true;
      continue;
    }
    let d = if j > i - j { j - (i - j) } else { (i - j) - j };
    if d < diff {
      rj = j;
      diff = d;
    }
  }
  println!("{} {}", i, rj);
}
