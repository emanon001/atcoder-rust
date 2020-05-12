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
    s: [char; n]
  };

  let c = s.into_iter().collect::<HashSet<_>>().len();
  let res = if c == 3 { "Three" } else { "Four" };
  println!("{}", res);
}
