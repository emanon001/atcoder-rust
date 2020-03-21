use proconio::input;

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

fn main() {
  input! {
    q: usize,
    queries: [(usize, usize); q]
  };

  let n = 100_000;
  let primes = primes(n)
    .into_iter()
    .collect::<std::collections::HashSet<_>>();
  let mut cusum = vec![0; n + 1];
  for i in 2..=n {
    if i % 2 == 0 {
      cusum[i] = cusum[i - 1];
    } else {
      if primes.contains(&(i as u64)) && primes.contains(&(((i + 1) / 2) as u64)) {
        cusum[i] = cusum[i - 1] + 1;
      } else {
        cusum[i] = cusum[i - 1];
      };
    }
  }
  for (l, r) in queries {
    println!("{}", cusum[r] - cusum[l - 1]);
  }
}
