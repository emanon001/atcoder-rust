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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comb() {
    let comb = ModComb::new(40, 1_000_000_007);
    assert_eq!(1, comb.comb(0, 0));
    assert_eq!(1, comb.comb(1, 0));
    assert_eq!(1, comb.comb(1, 1));
    assert_eq!(2, comb.comb(2, 1));
    assert_eq!(635_745_396, comb.comb(39, 10));
    assert_eq!(676_056_037, comb.comb(39, 11));
    assert_eq!(847_660_528, comb.comb(40, 10));
    assert_eq!(311_801_426, comb.comb(40, 11));
  }
}
