use crate::mod_int::ModInt;

pub struct ModComb {
  max: usize,
  fac: Vec<ModInt>,
  finv: Vec<ModInt>,
}

impl ModComb {
  pub fn new(max: usize) -> Self {
    let mut fac = vec![ModInt::from(0); max + 1];
    let mut finv = vec![ModInt::from(0); max + 1];
    let mut inv = vec![ModInt::from(0); max + 1];
    fac[0] = ModInt::from(1);
    fac[1] = ModInt::from(1);
    finv[0] = ModInt::from(1);
    finv[1] = ModInt::from(1);
    inv[1] = ModInt::from(1);
    let modulo = ModInt::MOD as usize;
    for i in 2..=max {
      fac[i] = fac[i - 1] * ModInt::from(i);
      inv[i] = ModInt::from(ModInt::from(modulo) - (inv[modulo % i] * ModInt::from(modulo / i)));
      finv[i] = finv[i - 1] * inv[i]
    }
    Self { max, fac, finv }
  }

  pub fn comb(&self, n: usize, k: usize) -> ModInt {
    if n > self.max {
      panic!();
    }
    if n < k {
      return ModInt::from(0);
    }
    self.fac[n] * self.finv[k] * self.finv[n - k]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comb() {
    let comb = ModComb::new(40);
    assert_eq!(1, comb.comb(0, 0).into());
    assert_eq!(1, comb.comb(1, 0).into());
    assert_eq!(1, comb.comb(1, 1).into());
    assert_eq!(2, comb.comb(2, 1).into());
    assert_eq!(635_745_396, comb.comb(39, 10).into());
    assert_eq!(676_056_037, comb.comb(39, 11).into());
    assert_eq!(847_660_528, comb.comb(40, 10).into());
    assert_eq!(311_801_426, comb.comb(40, 11).into());
  }
}
