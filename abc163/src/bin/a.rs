#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    r: f64
  };
  let res = 2_f64 * std::f64::consts::PI * r;
  println!("{}", res);
}
