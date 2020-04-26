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

  let p = 2019;
  let mut ten_factor = 1;
  let mut cur = 0;
  let mut counts = vec![0; p];
  counts[0] = 1;
  for i in 0..s.len() {
    let n = s[s.len() - i - 1].to_digit(10).unwrap() as usize;
    let m = (n * ten_factor) % p;
    cur = (cur + m) % p;
    counts[cur] += 1;
    ten_factor = (ten_factor * 10) % p;
  }
  let mut res = 0_u64;
  for c in counts {
    if c < 2 {
      continue;
    }
    res += (c * (c - 1)) / 2;
  }
  println!("{}", res);
}
