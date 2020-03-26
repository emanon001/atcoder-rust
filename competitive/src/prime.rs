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
  res.insert(n, 1);
  res
}

pub fn primes(n: usize) -> Vec<u64> {
  if n < 2 {
    return Vec::new();
  }
  let mut is_prime = vec![true; n + 1];
  is_prime[0] = false;
  is_prime[1] = false;
  let mut res = Vec::new();
  for i in 2..=n {
    if is_prime[i] {
      res.push(i as u64);
      let mut j = 2 * i;
      while j <= n {
        is_prime[j] = false;
        j += i;
      }
    }
  }
  res
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
  fn test_prime_factor() {
    fn map(pairs: Vec<(u64, u64)>) -> std::collections::HashMap<u64, u64> {
      pairs.into_iter().collect()
    }
    assert_eq!(map(Vec::new()), prime_factor(0));
    assert_eq!(map(Vec::new()), prime_factor(1));
    assert_eq!(map(vec![(2, 2), (3, 1)]), prime_factor(12));
    assert_eq!(
      map(vec![(2, 2), (41, 2), (148_721, 1)]),
      prime_factor(1_000_000_004)
    );
    assert_eq!(map(vec![(1_000_000_007, 1)]), prime_factor(1_000_000_007));
  }

  #[test]
  fn test_primes() {
    assert_eq!(Vec::new() as Vec<u64>, primes(0));
    assert_eq!(Vec::new() as Vec<u64>, primes(1));
    assert_eq!(vec![2], primes(2));
    assert_eq!(vec![2, 3], primes(3));
    assert_eq!(vec![2, 3, 5], primes(6));
    assert_eq!(99991, *primes(100_000).last().unwrap());
    assert_eq!(9592, primes(100_000).len());
  }
}
