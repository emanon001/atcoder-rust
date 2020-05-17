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
    a: f64, b: f64, h: f64, m: f64
  };

  let r1 = 0.5 * ((h * 60.0 + m) % (12.0 * 60.0));
  let r2 = 6.0 * m;
  let r = if (r1 - r2).abs() > 180.0 {
    180.0 - ((r1 - r2).abs() - 180.0)
  } else {
    (r1 - r2).abs()
  }
  .to_radians();
  let cos = r.cos();
  let res = ((a * a) + (b * b) - (2.0 * a * b * cos)).sqrt();
  println!("{}", res);
}
