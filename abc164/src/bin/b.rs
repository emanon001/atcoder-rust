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
    mut a: isize, b: isize, mut c: isize, d: isize
  };

  loop {
    c -= b;
    if c <= 0 {
      println!("Yes");
      std::process::exit(0);
    }
    a -= d;
    if a <= 0 {
      println!("No");
      std::process::exit(0);
    }
  }
}
