#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    a: usize, b: usize
  };

  let res = if a * b % 2 == 1 { "Odd" } else { "Even" };
  println!("{}", res);
}
