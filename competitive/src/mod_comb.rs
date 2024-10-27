use crate::mod_int::ModInt1000000007;
use cargo_snippet::snippet;
use num::{One, Zero};

#[snippet("mod_comb")]
#[snippet(include = "mod_int")]
pub struct ModComb {
    n: usize,
    fact: Vec<ModInt1000000007>,
    ifact: Vec<ModInt1000000007>,
}

#[snippet("mod_comb")]
impl ModComb {
    pub fn new(n: usize) -> Self {
        assert!(n < ModInt1000000007::MOD as usize);
        let mut fact = vec![ModInt1000000007::zero(); n + 1];
        let mut ifact = vec![ModInt1000000007::zero(); n + 1];
        fact[0] = ModInt1000000007::one();
        for i in 1..=n {
            fact[i] = fact[i - 1] * ModInt1000000007::from(i);
        }
        ifact[n] = fact[n].inv();
        for i in (1..=n).rev() {
            ifact[i - 1] = ifact[i] * ModInt1000000007::from(i);
        }
        Self { n, fact, ifact }
    }

    pub fn c(&self, n: usize, k: usize) -> ModInt1000000007 {
        assert!(n <= self.n);
        if k > n {
            return ModInt1000000007::zero();
        }
        self.fact[n] * self.ifact[k] * self.ifact[n - k]
    }

    pub fn p(&self, n: usize, k: usize) -> ModInt1000000007 {
        assert!(n <= self.n);
        if k > n {
            return ModInt1000000007::zero();
        }
        self.fact[n] * self.ifact[n - k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c() {
        let comb = ModComb::new(40);
        assert_eq!(comb.c(0, 0), ModInt1000000007::from(1));
        assert_eq!(comb.c(1, 0), ModInt1000000007::from(1));
        assert_eq!(comb.c(1, 1), ModInt1000000007::from(1));
        assert_eq!(comb.c(2, 1), ModInt1000000007::from(2));
        assert_eq!(comb.c(39, 10), ModInt1000000007::from(635_745_396));
        assert_eq!(comb.c(39, 11), ModInt1000000007::from(676_056_037));
        assert_eq!(comb.c(40, 10), ModInt1000000007::from(847_660_528));
        assert_eq!(comb.c(40, 11), ModInt1000000007::from(311_801_426));
    }

    #[test]
    fn p() {
        let comb = ModComb::new(15);
        assert_eq!(comb.p(0, 0), ModInt1000000007::from(1));
        assert_eq!(comb.p(1, 0), ModInt1000000007::from(1));
        assert_eq!(comb.p(1, 1), ModInt1000000007::from(1));
        assert_eq!(comb.p(2, 1), ModInt1000000007::from(2));
        assert_eq!(comb.p(15, 8), ModInt1000000007::from(259_459_200));
        assert_eq!(comb.p(15, 9), ModInt1000000007::from(816_214_393));
    }
}
