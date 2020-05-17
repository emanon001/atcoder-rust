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

  let res = match n % 10 {
    0 => "pon",
    1 => "pon",
    2 => "hon",
    3 => "bon",
    4 => "hon",
    5 => "hon",
    6 => "pon",
    7 => "hon",
    8 => "pon",
    9 => "hon",
    _ => unreachable!(),
  };
  println!("{}", res);
}
