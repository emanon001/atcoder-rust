#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn put(i: usize, j: usize, grid: &mut [Vec<u32>], res: &mut [Vec<u32>]) {
  let c = vec![
    grid[i - 1][j],
    grid[i][j + 1],
    grid[i + 1][j],
    grid[i][j - 1],
  ]
  .into_iter()
  .min()
  .unwrap();
  if c > 0 {
    res[i][j] += c;
    grid[i - 1][j] -= c;
    grid[i][j + 1] -= c;
    grid[i + 1][j] -= c;
    grid[i][j - 1] -= c;
  }
}

fn main() {
  input! {
    n: usize, m: usize,
    grid: [Chars; n]
  };

  let mut grid = grid
    .into_iter()
    .map(|row| {
      row
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let mut res = vec![vec![0_u32; m]; n];
  let mut t = 1;
  let mut b = n - 2;
  let mut l = 1;
  let mut r = m - 2;
  loop {
    for j in l..=r {
      put(t, j, &mut grid, &mut res);
    }
    for i in t..=b {
      put(i, l, &mut grid, &mut res);
      put(i, r, &mut grid, &mut res);
    }
    for j in l..=r {
      put(b, j, &mut grid, &mut res);
    }
    t += 1;
    b -= 1;
    l += 1;
    r -= 1;
    if t > b || l > r {
      break;
    }
  }
  for i in 0..n {
    println!(
      "{}",
      res[i].iter().map(|x| x.to_string()).collect::<String>()
    );
  }
}
