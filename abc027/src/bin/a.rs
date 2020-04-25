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
    lv: [usize; 3]
  };

  let mut table = HashMap::new();
  for l in lv {
    let x = table.entry(l).or_insert(0);
    *x += 1;
  }
  for (l, c) in table {
    if c == 1 || c == 3 {
      println!("{}", l);
    }
  }
}
