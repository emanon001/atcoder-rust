#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn prime_factor(n: u64) -> std::collections::HashMap<u64, u64> {
  if n < 2 {
    return std::collections::HashMap::new();
  }
  let mut res = std::collections::HashMap::new();
  let mut n = n;
  let mut i = 2;
  while i * i <= n {
    while n % i == 0 {
      let c = res.entry(i).or_insert(0);
      *c += 1;
      n /= i;
    }
    i += 1;
  }
  if n != 1 {
    res.insert(n, 1);
  }
  res
}

fn main() {
  input! {
    n: usize
  };

  let mut pf = HashMap::new();
  for a in 1..=n {
    let pf2 = prime_factor(a as u64);
    for (k, v) in pf2 {
      *pf.entry(k).or_insert(0) += v;
    }
  }

  // 3 * 5 * 5
  // 3 * 25
  // 5 * 15
  // 75
  let c3 = pf.iter().filter(|&(_, c)| *c >= 2).count() as isize;
  let c5 = pf.iter().filter(|&(_, c)| *c >= 4).count() as isize;
  let c15 = pf.iter().filter(|&(_, c)| *c >= 14).count() as isize;
  let c25 = pf.iter().filter(|&(_, c)| *c >= 24).count() as isize;
  let c75 = pf.iter().filter(|&(_, c)| *c >= 74).count() as isize;
  let res = (c3 - c5) * c5 * std::cmp::max(c5 - 1, 0) / 2
    + c5 * std::cmp::max(c5 - 1, 0) * std::cmp::max(c5 - 2, 0) / 2
    + (c3 - c25) * c25
    + c25 * std::cmp::max(c25 - 1, 0)
    + (c5 - c15) * c15
    + c15 * std::cmp::max(c15 - 1, 0)
    + c75;
  println!("{}", res);
}
