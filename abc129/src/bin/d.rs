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
    h: usize, w: usize,
    grid: [Chars; h]
  };

  let mut cusum_l = vec![vec![0; w]; h];
  for i in 0..h {
    let mut c = 0;
    for j in 0..w {
      if grid[i][j] == '#' {
        cusum_l[i][j] = 0;
        c = 0;
      } else {
        cusum_l[i][j] = c;
        c += 1;
      }
    }
  }
  let mut cusum_r = vec![vec![0; w]; h];
  for i in 0..h {
    let mut c = 0;
    for j in (0..w).rev() {
      if grid[i][j] == '#' {
        cusum_r[i][j] = 0;
        c = 0;
      } else {
        cusum_r[i][j] = c;
        c += 1;
      }
    }
  }
  let mut cusum_t = vec![vec![0; w]; h];
  for j in 0..w {
    let mut c = 0;
    for i in 0..h {
      if grid[i][j] == '#' {
        cusum_t[i][j] = 0;
        c = 0;
      } else {
        cusum_t[i][j] = c;
        c += 1;
      }
    }
  }
  let mut cusum_b = vec![vec![0; w]; h];
  for j in 0..w {
    let mut c = 0;
    for i in (0..h).rev() {
      if grid[i][j] == '#' {
        cusum_b[i][j] = 0;
        c = 0;
      } else {
        cusum_b[i][j] = c;
        c += 1;
      }
    }
  }

  let mut res = 0;
  for i in 0..h {
    for j in 0..w {
      if grid[i][j] == '#' {
        continue;
      }
      let c = cusum_l[i][j] + cusum_r[i][j] + cusum_t[i][j] + cusum_b[i][j] + 1;
      if c > res {
        res = c;
      }
    }
  }
  println!("{}", res);
}
