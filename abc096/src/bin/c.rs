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

  let dy = vec![-1, 0, 1, 0];
  let dx = vec![0, 1, 0, -1];
  for i in 0..h {
    for j in 0..w {
      let cell = grid[i][j];
      if cell == '.' {
        continue;
      }
      let is_ok = (0..4).any(|d| {
        let new_i = i as isize + dy[d];
        let new_j = j as isize + dx[d];
        if new_i < 0 || new_i >= h as isize || new_j < 0 || new_j >= w as isize {
          return false;
        }
        let new_i = new_i as usize;
        let new_j = new_j as usize;
        grid[new_i][new_j] == '#'
      });
      if !is_ok {
        println!("No");
        std::process::exit(0);
      }
    }
  }
  println!("Yes");
}
