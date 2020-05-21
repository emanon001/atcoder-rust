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
    av: [usize; n]
  };

  let mut que = VecDeque::new();
  for i in 0..n {
    if i % 2 == 0 {
      que.push_back(av[i]);
    } else {
      que.push_front(av[i]);
    }
  }
  let res = if n % 2 == 1 {
    que.into_iter().rev().join(" ")
  } else {
    que.into_iter().join(" ")
  };
  println!("{}", res);
}
