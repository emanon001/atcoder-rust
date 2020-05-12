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
    grid: [Chars; 3]
  };

  let res = format!("{}{}{}", grid[0][0], grid[1][1], grid[2][2]);
  println!("{}", res);
}
