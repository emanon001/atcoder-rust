use crate::mod_int::ModInt;
use cargo_snippet::snippet;

#[snippet("mod_comb")]
#[snippet(include = "mod_int")]
pub struct ModComb {
    n: usize,
    fact: Vec<ModInt>,
    ifact: Vec<ModInt>,
}

#[snippet("mod_comb")]
impl ModComb {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![ModInt::zero(); n + 1];
        let mut ifact = vec![ModInt::zero(); n + 1];
        let mut inv = vec![ModInt::zero(); n + 1];
        fact[0] = ModInt::one();
        fact[1] = ModInt::one();
        ifact[0] = ModInt::one();
        ifact[1] = ModInt::one();
        inv[1] = ModInt::one();
        let modulo = ModInt::MOD as usize;
        for i in 2..=n {
            fact[i] = fact[i - 1] * ModInt::from(i);
            inv[i] =
                ModInt::from(ModInt::from(modulo) - (inv[modulo % i] * ModInt::from(modulo / i)));
            ifact[i] = ifact[i - 1] * inv[i]
        }
        Self { n, fact, ifact }
    }

    pub fn comb(&self, n: usize, k: usize) -> ModInt {
        assert!(n <= self.n);
        if k > n {
            return ModInt::zero();
        }
        self.fact[n] * self.ifact[k] * self.ifact[n - k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb() {
        let comb = ModComb::new(40);
        assert_eq!(comb.comb(0, 0), ModInt::one());
        assert_eq!(comb.comb(1, 0), ModInt::one());
        assert_eq!(comb.comb(1, 1), ModInt::one());
        assert_eq!(comb.comb(2, 1), ModInt::from(2));
        assert_eq!(comb.comb(39, 10), ModInt::from(635_745_396));
        assert_eq!(comb.comb(39, 11), ModInt::from(676_056_037));
        assert_eq!(comb.comb(40, 10), ModInt::from(847_660_528));
        assert_eq!(comb.comb(40, 11), ModInt::from(311_801_426));
    }
}
