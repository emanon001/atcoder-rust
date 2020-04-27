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
    n: usize,
    cs: String
  };

  let commands = vec!['A', 'B', 'X', 'Y'];
  let mut cv = Vec::new();
  for i in 0..4 {
    for j in 0..4 {
      cv.push(format!("{}{}", commands[i], commands[j]));
    }
  }
  let mut res = 1 << 30;
  for lc in &cv {
    for rc in &cv {
      let s = cs.clone();
      let s = s.replace(lc, "_").replace(rc, "_").replace("_", "");
      let count = s.len() + (n - s.len()) / 2;
      if count < res {
        res = count;
      }
    }
  }
  println!("{}", res);
}
