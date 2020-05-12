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
    h: usize, w: usize, d: usize,
    grid: [[usize; w]; h],
    q: usize,
    queries: [(usize, usize); q]
  };

  let mut x_to_pos = HashMap::new();
  for i in 0..h {
    for j in 0..w {
      let x = grid[i][j];
      x_to_pos.insert(x, (i, j));
    }
  }
  let max = h * w;
  let mut cusum = vec![0_u64; max + 1];
  for x in 1..=max {
    let next = x + d;
    if next > max {
      continue;
    }
    let &(xx, xy) = x_to_pos.get(&x).unwrap();
    let &(nx, ny) = x_to_pos.get(&next).unwrap();
    let cost = cusum[x]
      + (xx as isize - nx as isize).abs() as u64
      + (xy as isize - ny as isize).abs() as u64;
    cusum[next] = cost;
  }
  for (l, r) in queries {
    let res = cusum[r] - cusum[l];
    println!("{}", res);
  }
}
