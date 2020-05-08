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
    x: usize
  };

  let is_ok = match x {
    7 => true,
    5 => true,
    3 => true,
    _ => false,
  };
  let res = if is_ok { "YES" } else { "NO" };
  println!("{}", res);
}
