#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    s: String
  };

  let res = if s.contains("7") { "Yes" } else { "No" };
  println!("{}", res);
}
