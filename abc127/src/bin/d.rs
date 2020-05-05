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
    av: [usize; n],
    bcv: [(usize, usize); m]
  };

  let mut table = HashMap::new();
  for a in av {
    *table.entry(a).or_insert(0) += 1;
  }
  let mut heap = BinaryHeap::from(table.into_iter().collect::<Vec<_>>());
  for (b, c) in bcv {
    heap.push((c, b));
  }
  let mut res = 0_u64;
  let mut rest = n;
  while let Some((a, c)) = heap.pop() {
    let count = std::cmp::min(c, rest);
    res += a as u64 * count as u64;
    rest -= count;
    if rest == 0 {
      break;
    }
  }
  println!("{}", res);
}
