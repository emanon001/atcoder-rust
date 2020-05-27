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
            inv[i] =
                ModInt::from(ModInt::from(modulo) - (inv[modulo % i] * ModInt::from(modulo / i)));
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
        assert_eq!(comb.comb(0, 0), ModInt::from(1));
        assert_eq!(comb.comb(1, 0), ModInt::from(1));
        assert_eq!(comb.comb(1, 1), ModInt::from(1));
        assert_eq!(comb.comb(2, 1), ModInt::from(2));
        assert_eq!(comb.comb(39, 10), ModInt::from(635_745_396));
        assert_eq!(comb.comb(39, 11), ModInt::from(676_056_037));
        assert_eq!(comb.comb(40, 10), ModInt::from(847_660_528));
        assert_eq!(comb.comb(40, 11), ModInt::from(311_801_426));
    }
}
