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
    x: isize
  };

  for a in -150..=150 {
    for b in -150..=150 {
      if a.pow(5) - b.pow(5) == x {
        println!("{} {}", a, b);
        std::process::exit(0);
      }
    }
  }
}
