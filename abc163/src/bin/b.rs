#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    n: usize, m: usize,
    av: [usize; m]
  };

  let sum: usize = av.into_iter().sum();
  let res = if sum > n { -1 } else { (n - sum) as isize };
  println!("{}", res);
}
