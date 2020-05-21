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
    s: Chars
  };

  for a in 1..=s.len() - 2 {
    let len = s.len() - a;
    if len % 2 == 1 {
      continue;
    }
    let size = len / 2;
    let is_ok = &s[..size] == &s[size..len];
    if is_ok {
      println!("{}", len);
      std::process::exit(0);
    }
  }
}
