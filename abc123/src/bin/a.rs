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
    v: [usize; 5],
    k: usize
  };

  for i in 0..v.len() - 1 {
    for j in i + 1..v.len() {
      if v[j] - v[i] > k {
        println!(":(");
        std::process::exit(0);
      }
    }
  }
  println!("Yay!");
}
