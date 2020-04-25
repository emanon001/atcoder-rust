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
    av: [isize; n]
  };

  let sum: isize = av.iter().sum();
  if sum % n as isize != 0 {
    println!("-1");
    std::process::exit(0);
  }

  let m = sum / n as isize;
  let mut res = 0;
  let mut insufficient = 0;
  for a in av {
    if a - insufficient != m {
      res += 1;
    }
    insufficient += m - a;
  }
  println!("{}", res);
}
