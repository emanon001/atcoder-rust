#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct ModComb {
  max: usize,
  modulo: usize,
  fac: Vec<usize>,
  finv: Vec<usize>,
}

impl ModComb {
  pub fn new(max: usize, modulo: usize) -> Self {
    let mut fac = vec![0_usize; max + 1];
    let mut finv = vec![0_usize; max + 1];
    let mut inv = vec![0_usize; max + 1];
    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;
    inv[1] = 1;
    for i in 2..=max {
      fac[i] = (fac[i - 1] * i) % modulo;
      inv[i] = modulo - ((inv[modulo % i] * (modulo / i)) % modulo);
      finv[i] = (finv[i - 1] * inv[i]) % modulo;
    }
    Self {
      max,
      modulo,
      fac,
      finv,
    }
  }

  pub fn comb(&self, n: usize, k: usize) -> usize {
    if n > self.max {
      panic!();
    }
    if n < k {
      return 0;
    }
    (self.fac[n] * ((self.finv[k] * self.finv[n - k]) % self.modulo)) % self.modulo
  }
}

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

const MOD: usize = 1_000_000_007;

fn main() {
  input! {
    n: usize, m: usize
  };

  let pf = prime_factor(m as u64);
  let comb = ModComb::new(n + 50, MOD);
  let mut res = 1_usize;
  for (_, c) in pf {
    res = (res * comb.comb(c as usize + n - 1, c as usize)) % MOD;
  }
  println!("{}", res);
}
