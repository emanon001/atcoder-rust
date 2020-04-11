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
    mut av: [usize; n]
  };

  let mut m = 2;
  let mut res = 0;
  loop {
    let is_ok = av.iter().all(|&a| a % m == 0);
    if is_ok {
      res += 1;
      m *= 2;
    } else {
      break;
    }
  }
  println!("{}", res);
}
