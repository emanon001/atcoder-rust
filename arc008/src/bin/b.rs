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
    _: usize, _: usize,
    name: Chars,
    kit: Chars
  };

  let mut n_table = HashMap::new();
  for ch in name {
    let x = n_table.entry(ch).or_insert(0);
    *x += 1;
  }
  let mut k_table = HashMap::new();
  for ch in kit {
    let x = k_table.entry(ch).or_insert(0);
    *x += 1;
  }
  let mut res = 0;
  for (ch, c) in n_table {
    if let Some(c2) = k_table.get(&ch) {
      let c3 = (c + c2 - 1) / c2;
      if c3 > res {
        res = c3;
      }
    } else {
      println!("-1");
      std::process::exit(0);
    }
  }
  println!("{}", res);
}
