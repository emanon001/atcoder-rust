#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn primes(n: usize) -> Vec<usize> {
  if n < 2 {
    return Vec::new();
  }
  let mut is_prime = vec![true; n + 1];
  is_prime[0] = false;
  is_prime[1] = false;
  let mut res = Vec::new();
  for i in 2..=n {
    if is_prime[i] {
      res.push(i);
      let mut j = 2 * i;
      while j <= n {
        is_prime[j] = false;
        j += i;
      }
    }
  }
  res
}

fn main() {
  input! {
    n: usize
  };

  let res = primes(55_555)
    .into_iter()
    .filter(|&x| x % 5 == 1)
    .take(n)
    .join(" ");
  println!("{}", res);
}
