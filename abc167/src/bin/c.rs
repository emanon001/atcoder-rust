#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

const INF: usize = 1_usize << 60;

fn main() {
  input! {
    n: usize, m: usize, x: usize,
    books: [[usize; m + 1]; n]
  };

  let mut res = INF;
  for bits in 0..(1 << n) {
    let mut v = vec![0; m];
    let mut price = 0_usize;
    for i in 0..n {
      if (bits >> i) & 1 == 1 {
        price += books[i][0];
        for j in 0..m {
          v[j] += books[i][j + 1];
        }
      }
    }
    let is_ok = v.into_iter().all(|p| p >= x);
    if is_ok && price < res {
      res = price;
    }
  }
  println!("{}", if res == INF { -1 } else { res as isize });
}
