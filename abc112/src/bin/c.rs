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
    xyhv: [(isize, isize, isize); n],
  };

  for cx in 0_isize..=100 {
    for cy in 0_isize..=100 {
      let mut max = 1_isize << 60;
      for &(x, y, h) in &xyhv {
        if h == 0 {
          let d = (x - cx).abs() + (y - cy).abs();
          if d < max {
            max = d;
          }
        }
      }
      let mut is_ok = true;
      let mut height = -1_isize;
      for &(x, y, h) in &xyhv {
        if h > 0 {
          let d = (x - cx).abs() + (y - cy).abs() + h;
          if d > max {
            is_ok = false;
            break;
          }
          if height == -1 {
            height = d;
          }
          if height != d {
            is_ok = false;
            break;
          }
        }
      }
      if is_ok {
        println!("{} {} {}", cx, cy, height);
      }
    }
  }
}
