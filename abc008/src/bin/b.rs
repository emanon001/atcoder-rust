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

  let mut table = HashMap::new();
  for s in sv {
    let c = table.entry(s).or_insert(0);
    *c += 1;
  }
  let mut vec = table.into_iter().collect::<Vec<_>>();
  vec.sort_by_key(|(_, c)| -(*c as isize));
  println!("{}", vec[0].0);
}
