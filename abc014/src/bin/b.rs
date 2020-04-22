#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    n: usize, x: usize,
    av: [usize; n]
  };

  let mut res = 0;
  for i in 0..n {
    if (x >> i) & 1 == 1 {
      res += av[i];
    }
  }
  println!("{}", res);
}
