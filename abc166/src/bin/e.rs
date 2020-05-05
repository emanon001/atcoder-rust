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
    av: [isize; n]
  };

  let mut add_table: HashMap<isize, u64> = HashMap::new();
  for (i, &a) in av.iter().enumerate() {
    let i = (i + 1) as isize;
    *add_table.entry(i + a).or_insert(0) += 1;
  }
  let mut sub_table = HashMap::new();
  for (i, &a) in av.iter().enumerate() {
    let i = (i + 1) as isize;
    *sub_table.entry(i - a).or_insert(0) += 1;
  }
  let mut res = 0_u64;
  for (&a, &c) in &add_table {
    res += c * sub_table.get(&a).unwrap_or(&0);
  }
  println!("{}", res);
}
