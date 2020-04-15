#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    a: String,
    b: String
  };

  let res = if a.len() > b.len() { a } else { b };
  println!("{}", res);
}
