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
    n: usize, m: usize,
    lrv: [(Usize1, Usize1); m]
  };

  let mut imos = vec![0_isize; n + 1];
  for (l, r) in lrv {
    imos[l] += 1;
    imos[r + 1] -= 1;
  }
  for i in 1..imos.len() {
    imos[i] += imos[i - 1];
  }
  let res = imos.into_iter().filter(|&c| c == m as isize).count();
  println!("{}", res);
}
