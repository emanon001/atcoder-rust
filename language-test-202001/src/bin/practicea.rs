#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    a: usize,
    b: usize, c: usize,
    s: String
  };

  println!("{} {}", a + b + c, s);
}
