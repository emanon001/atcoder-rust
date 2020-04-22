#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    n: usize,
    abv: [(usize, usize); n]
  };

  let color_size = 1_000_001;
  let mut imos = vec![0_isize; color_size + 1];
  for (a, b) in abv {
    imos[a] += 1;
    imos[b + 1] -= 1;
  }
  let mut v = vec![0; color_size];
  let mut cur = 0;
  for i in 0..color_size {
    cur = cur + imos[i];
    v[i] = cur;
  }
  let res = v.iter().max().unwrap();
  println!("{}", res);
}
