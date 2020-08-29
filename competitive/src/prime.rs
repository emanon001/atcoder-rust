use cargo_snippet::snippet;

#[snippet]
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[snippet]
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

#[snippet]
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

#[snippet("prime_factors")]
pub struct PrimeFactors {
    n: usize,
    min_primes: Vec<usize>,
}

#[snippet("prime_factors")]
impl PrimeFactors {
    pub fn new(n: usize) -> Self {
        let mut min_primes = vec![0; n + 1];
        for i in 2..=n {
            if min_primes[i] == 0 {
                let mut j = i;
                while j <= n {
                    min_primes[j] = i;
                    j += i;
                }
            }
        }
        Self { n, min_primes }
    }

    pub fn prime_factor(&self, n: usize) -> std::collections::HashMap<usize, usize> {
        if n < 2 {
            return std::collections::HashMap::new();
        }
        if n > self.n {
            panic!("n > self.n");
        }
        let mut res = std::collections::HashMap::new();
        let mut x = n;
        while x != 1 {
            let y = self.min_primes[x];
            *res.entry(y).or_insert(0) += 1;
            x = x / y;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(6));
        assert!(is_prime(1_000_000_007));
    }

    #[test]
    fn test_primes() {
        assert_eq!(primes(0), Vec::new() as Vec<usize>);
        assert_eq!(primes(1), Vec::new() as Vec<usize>);
        assert_eq!(primes(2), vec![2]);
        assert_eq!(primes(3), vec![2, 3]);
        assert_eq!(primes(6), vec![2, 3, 5]);
        assert_eq!(*primes(100_000).last().unwrap(), 99991);
        assert_eq!(primes(100_000).len(), 9592);
    }

    #[test]
    fn test_prime_factor() {
        fn to_map(pairs: Vec<(u64, u64)>) -> std::collections::HashMap<u64, u64> {
            pairs.into_iter().collect()
        }
        assert_eq!(prime_factor(0), to_map(Vec::new()));
        assert_eq!(prime_factor(1), to_map(Vec::new()));
        assert_eq!(prime_factor(12), to_map(vec![(2, 2), (3, 1)]));
        assert_eq!(prime_factor(36), to_map(vec![(2, 2), (3, 2)]));
        assert_eq!(
            prime_factor(1_000_000_004),
            to_map(vec![(2, 2), (41, 2), (148_721, 1)]),
        );
        assert_eq!(
            prime_factor(1_000_000_007),
            to_map(vec![(1_000_000_007, 1)])
        );
    }

    #[test]
    fn test_prime_factors() {
        fn to_map(pairs: Vec<(usize, usize)>) -> std::collections::HashMap<usize, usize> {
            pairs.into_iter().collect()
        }
        let pfs = PrimeFactors::new(100_000);
        assert_eq!(pfs.prime_factor(0), to_map(Vec::new()));
        assert_eq!(pfs.prime_factor(1), to_map(Vec::new()));
        assert_eq!(pfs.prime_factor(2), to_map(vec![(2, 1)]));
        assert_eq!(pfs.prime_factor(12), to_map(vec![(2, 2), (3, 1)]));
        assert_eq!(pfs.prime_factor(13), to_map(vec![(13, 1)]));
        assert_eq!(pfs.prime_factor(36), to_map(vec![(2, 2), (3, 2)]));
        assert_eq!(pfs.prime_factor(100_000), to_map(vec![(2, 5), (5, 5)]));
    }
}
