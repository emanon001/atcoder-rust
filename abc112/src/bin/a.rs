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

  if n == 1 {
    println!("Hello World");
  } else {
    input! {
      a: usize,
      b: usize
    };
    println!("{}", a + b);
  }
}
