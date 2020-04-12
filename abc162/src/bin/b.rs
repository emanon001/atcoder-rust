#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    n: usize
  };

  let mut res = 0_u64;
  for a in 1..=n {
    match (a % 3 == 0, a % 5 == 0) {
      (false, false) => {
        res += a as u64;
      }
      _ => {}
    }
  }
  println!("{}", res);
}
