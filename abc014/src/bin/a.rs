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

  let n = a % b;
  let res = if n == 0 { 0 } else { b - n };
  println!("{}", res);
}
