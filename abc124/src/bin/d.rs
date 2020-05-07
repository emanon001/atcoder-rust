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
    n: usize, k: usize,
    s: Chars
  };

  // ('0'|'1', count)
  let mut cv = Vec::new();
  let mut prev = s[0];
  let mut c = 1;
  for i in 1..n {
    if prev == s[i] {
      c += 1;
    } else {
      cv.push((prev, c));
      c = 1;
    }
    prev = s[i];
  }
  cv.push((prev, c));

  let mut cusum = Vec::new();
  let mut sum = 0;
  for i in 0..cv.len() {
    let (ch, c) = cv[i];
    sum += c;
    if ch == '0' {
      cusum.push((i, sum));
    }
  }

  if cusum.is_empty() {
    println!("{}", cv[0].1);
    std::process::exit(0);
  }

  let mut res = 0;
  for i in 0..cusum.len() {
    let j = std::cmp::min(i + k - 1, cusum.len() - 1);
    let mut sum = if i > 0 {
      cusum[j].1 - cusum[i - 1].1
    } else {
      cusum[j].1
    };
    let ci = cusum[j].0;
    if ci + 1 < cv.len() {
      sum += cv[ci + 1].1;
    }
    if sum > res {
      res = sum;
    }
  }
  println!("{}", res);
}
