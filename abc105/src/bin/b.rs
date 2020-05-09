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
    n: usize
  };

  for a in 0..=n / 4 {
    let rest = n - a * 4;
    if rest % 7 == 0 {
      println!("Yes");
      std::process::exit(0);
    }
  }
  println!("No");
}
