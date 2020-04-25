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

  let mut cv = Vec::new();
  let mut c = 0;
  let mut i = 0;
  while i + 1 < s.len() {
    match (s[i], s[i + 1]) {
      ('2', '5') => {
        c += 1;
        i += 2;
      }
      (_, '2') => {
        if c > 0 {
          cv.push(c);
        }
        c = 0;
        i += 1;
      }
      _ => {
        if c > 0 {
          cv.push(c);
        }
        c = 0;
        i += 2;
      }
    }
  }
  if c > 0 {
    cv.push(c);
  }
  let mut res = 0_u64;
  for c in cv {
    res += (c + 1) * c / 2;
  }
  println!("{}", res);
}
