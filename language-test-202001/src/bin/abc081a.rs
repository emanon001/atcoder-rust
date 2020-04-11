#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    s: Chars
  };

  let res = s.into_iter().filter(|&ch| ch == '1').count();
  println!("{}", res);
}
