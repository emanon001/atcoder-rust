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
    av: [usize; n]
  };

  let mut s_count = 0;
  let mut bug_count = 0;
  for a in av {
    if a > 0 {
      s_count += 1;
      bug_count += a;
    }
  }
  let res = (bug_count + s_count - 1) / s_count;
  println!("{}", res);
}
