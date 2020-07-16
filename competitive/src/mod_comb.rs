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
        assert!(n < ModInt::MOD as usize);
        let mut fact = vec![ModInt::zero(); n + 1];
        let mut ifact = vec![ModInt::zero(); n + 1];
        fact[0] = ModInt::one();
        for i in 1..=n {
            fact[i] = fact[i - 1] * ModInt::from(i);
        }
        ifact[n] = fact[n].inv();
        for i in (1..=n).rev() {
            ifact[i - 1] = ifact[i] * ModInt::from(i);
        }
        Self { n, fact, ifact }
    }

    pub fn c(&self, n: usize, k: usize) -> ModInt {
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
        assert_eq!(comb.c(0, 0), ModInt::one());
        assert_eq!(comb.c(1, 0), ModInt::one());
        assert_eq!(comb.c(1, 1), ModInt::one());
        assert_eq!(comb.c(2, 1), ModInt::from(2));
        assert_eq!(comb.c(39, 10), ModInt::from(635_745_396));
        assert_eq!(comb.c(39, 11), ModInt::from(676_056_037));
        assert_eq!(comb.c(40, 10), ModInt::from(847_660_528));
        assert_eq!(comb.c(40, 11), ModInt::from(311_801_426));
    }
}
