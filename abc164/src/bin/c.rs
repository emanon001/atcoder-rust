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
    n: usize,
    sv: [String; n]
  };

  let res = sv.into_iter().collect::<HashSet<_>>().len();
  println!("{}", res);
}
