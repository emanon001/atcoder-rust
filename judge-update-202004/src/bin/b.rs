#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    n: usize,
    mut xcv: [(usize, char); n]
  };

  xcv.sort_by_key(|&(x, c)| {
    let c = if c == 'R' { 0 } else { 1 };
    (c, x)
  });
  for (x, _) in xcv {
    println!("{}", x);
  }
}
