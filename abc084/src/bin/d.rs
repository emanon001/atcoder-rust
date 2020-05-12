use proconio::input;
use std::collections::HashSet;

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

fn main() {
  input! {
    q: usize,
    lrv: [(usize, usize); q]
  };

  let max = 100_000;
  let prime_set = primes(max).into_iter().collect::<HashSet<_>>();
  let cusum = (0..=max)
    .scan(0, |acc, x| {
      let is_ok = x % 2 == 1 && prime_set.contains(&x) && prime_set.contains(&((x + 1) / 2));
      *acc += if is_ok { 1 } else { 0 };
      Some(*acc)
    })
    .collect::<Vec<_>>();
  for (l, r) in lrv {
    let res = cusum[r] - cusum[l - 1];
    println!("{}", res);
  }
}
