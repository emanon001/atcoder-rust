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
    n: i64, m: i64
  };

  if n == 1 || m == 1 {
    let max = n.max(m);
    let res = match max {
      1 => 1,
      _ => max - 2,
    };
    println!("{}", res);
    std::process::exit(0);
  }

  let res = (n - 2) * (m - 2);
  println!("{}", res);
}
