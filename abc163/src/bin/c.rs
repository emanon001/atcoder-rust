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
    av: [Usize1; n - 1]
  };

  let mut res = vec![0; n];
  for i in 1..n {
    res[av[i - 1]] += 1;
  }
  for i in 0..n {
    println!("{}", res[i]);
  }
}
