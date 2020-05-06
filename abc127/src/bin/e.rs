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

const MOD: usize = 1_000_000_007;

fn main() {
  input! {
    n: usize, m: usize, k: usize
  };

  let comb = ModComb::new(n * m, MOD);
  let mut res = 0;
  for i in 0..n - 1 {
    let d = (n - i) * (n - i - 1) / 2 % MOD;
    res = (res + (d * m * m * comb.comb(n * m - 2, k - 2) % MOD)) % MOD;
  }
  for i in 0..m - 1 {
    let d = (m - i) * (m - i - 1) / 2 % MOD;
    res = (res + (d * n * n * comb.comb(n * m - 2, k - 2) % MOD)) % MOD;
  }
  println!("{}", res);
}
