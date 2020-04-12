#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    k: usize,
  };

  let mut v = Vec::new();
  for a in 1..=k {
    for b in 1..=k {
      let x = a.gcd(&b);
      v.push(x);
    }
  }

  let mut table = vec![vec![1; k + 1]; k + 1];
  for a in 1..=k {
    for b in 1..=k {
      let x = a.gcd(&b);
      table[a][b] = x;
    }
  }

  let mut res = 0;
  for a in 1..=k {
    for &b in &v {
      res += table[a][b];
    }
  }

  println!("{}", res);
}
