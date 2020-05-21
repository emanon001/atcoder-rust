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
    mut av: [usize; 3]
  };

  av.sort();
  let res = av[0] + av[1];
  println!("{}", res);
}
