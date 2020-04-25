#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn convert(n: usize, table: &[usize]) -> usize {
  n.to_string()
    .chars()
    .map(|ch| table[ch.to_digit(10).unwrap() as usize])
    .fold(0, |acc, x| acc * 10 + x)
}

fn main() {
  input! {
    bv: [usize; 10],
    n: usize,
    mut av: [usize; n]
  };

  let mut table = vec![0; 10];
  for (i, b) in bv.into_iter().enumerate() {
    table[b] = i;
  }
  av.sort_by_key(|&a| convert(a, &table));
  for a in av {
    println!("{}", a);
  }
}
