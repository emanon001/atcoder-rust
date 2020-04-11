#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    s: isize, l: isize, r: isize
  };

  let res = if s >= l && s <= r {
    s
  } else if s < l {
    l
  } else {
    r
  };
  println!("{}", res);
}
